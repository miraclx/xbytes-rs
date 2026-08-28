use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use super::numeric::Numeric;
use super::{Float, Int, ParseError, Unit, sizes};

mod flags {
    #![allow(non_upper_case_globals)]

    use bitflags::bitflags;

    bitflags! {
        /// Which number system and denomination [`ByteSize::repr`](super::ByteSize::repr)
        /// renders in: the prefix base (binary or decimal), the unit (bytes or
        /// bits), and whether to prefix at all.
        #[derive(Eq, Copy, Hash, Clone, Debug, Default, PartialEq)]
        pub struct Mode: u8 {
            /// Binary prefixes, byte-denominated, prefixed: `1.50 MiB`.
            const Default  = 0;
            /// Denominate in bits rather than bytes: `12 Mib`.
            const Bits     = 1 << 0;
            /// Use decimal (power-of-1000) prefixes: `1.57 MB`.
            const Decimal  = 1 << 1;
            /// Render in the base unit with no prefix at all.
            const NoPrefix = 1 << 2;
        }
    }

    bitflags! {
        /// How a [`ByteSizeRepr`](super::ByteSizeRepr) spells its unit and
        /// number: symbol style, pluralization, capitalization, fractions, and
        /// spacing. The example after each flag shows its effect.
        #[derive(Eq, Copy, Hash, Clone, Debug, Default, PartialEq)]
        pub struct Format: u16 {
            /// Symbol form with the binary `i`: `1 B, 2.13 KB, 1024.43 MiB`.
            const Default                = 0;

            /// Drop the binary `i` from symbols: `2.13 KB, 1024.43 MB`.
            const Initials               = 1 << 0;
            /// Prefix initial only, no unit letter: `2.13 K, 1024.43 M`.
            const Condensed              = 1 << 1;
            /// Spell the unit in full: `2.13 KiloBytes, 1024.43 MebiBytes`.
            const Long                   = 1 << 2;

            /// With [`Long`](Self::Long), never pluralize: `2.13 KiloByte`.
            const NoPlural               = 1 << 3;

            /// With [`Long`](Self::Long), capitalize only the first letter:
            /// `2.13 Kilobytes`.
            const NoMultiCaps            = 1 << 4;

            /// Lowercase the whole unit: `2.13 kb, 1024.43 mib`.
            const LowerCaps              = 1 << 5;
            /// Uppercase the whole unit: `2.13 KB, 1024.43 MIB`.
            const UpperCaps              = 1 << 6;

            /// Truncate the fractional part: `2 KB, 1024 MiB`.
            const NoFraction             = 1 << 7;
            /// Always show a fractional part: `1.00 B, 2.13 KB`.
            const ForceFraction          = 1 << 8;

            /// Group the whole part in thousands: `1,024.43 MiB`.
            const ShowThousandsSeparator = 1 << 9;
            /// Omit the space between number and unit: `2.13KB`.
            const NoSpace                = 1 << 10;
        }
    }
}

pub use flags::*;

/// The full formatting state a [`ByteSizeRepr`] carries: the [`Format`] flags
/// plus the numeric knobs (spacing, precision, thousands separator) that flags
/// alone cannot express. Build one up with [`with`](ReprFormat::with).
///
/// ```
/// use xbytes::prelude::*;
///
/// let format = ReprFormat::default()
///     .with(Precision(4))
///     .with(Format::ForceFraction);
/// let repr = ByteSize::of(4096, KIBI_BYTE).repr(Mode::Default);
/// assert_eq!(repr.with(format).to_string(), "4.0000 MiB");
/// ```
#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct ReprFormat {
    flags: Format,
    n_spaces: usize,
    precision: usize,
    thousands_separator: &'static str,
}

impl Default for ReprFormat {
    fn default() -> Self {
        Self::const_default()
    }
}

impl ReprFormat {
    /// The starting format: default flags, one space, precision two, comma
    /// separator. A `const` sibling to the [`Default`] impl for use in `const`
    /// contexts.
    const fn const_default() -> Self {
        Self {
            flags: Format::Default,
            n_spaces: 1,
            precision: 2,
            thousands_separator: ",",
        }
    }

    /// Return a copy with one more piece of configuration folded in: a
    /// [`Format`] flag, a [`ReprConfigVariant`], or another `ReprFormat`.
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let format = ReprFormat::default().with(Format::NoSpace).with(Spaces(0));
    /// let repr = ByteSize::of(1, MEBI_BYTE).repr(Mode::Default);
    /// assert_eq!(repr.with(format).to_string(), "1MiB");
    /// ```
    #[must_use]
    pub fn with(&self, conf: impl ReprConfig) -> Self {
        conf.apply(self)
    }
}

/// A piece of configuration that can be folded into a [`ReprFormat`]. Implemented
/// for [`Format`] flags, [`ReprConfigVariant`]s, and whole `ReprFormat`s, so
/// [`ReprFormat::with`] and [`ByteSizeRepr::with`] accept any of them.
pub trait ReprConfig {
    /// Apply this configuration on top of `r_fmt`, returning the updated format.
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat;
}

impl<T: ReprConfig> ReprConfig for &T {
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat {
        (*self).apply(r_fmt)
    }
}

impl ReprConfig for Format {
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat {
        ReprFormat {
            flags: bitflags_const_or!(Format::{r_fmt.flags, self}),
            ..*r_fmt
        }
    }
}

/// The numeric formatting knobs that a plain [`Format`] flag cannot carry a
/// value for. Pass any variant to [`ReprFormat::with`] or
/// [`ByteSizeRepr::with`].
#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum ReprConfigVariant {
    /// The string inserted between thousands groups (default `","`), used only
    /// when [`Format::ShowThousandsSeparator`] is set.
    ThousandsSeparator(&'static str),
    /// Digits after the decimal point when a fraction is shown (default `2`).
    Precision(usize),
    /// Number of spaces between the number and the unit (default `1`).
    Spaces(usize),
}

use ReprConfigVariant::*;

impl ReprConfig for ReprConfigVariant {
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat {
        let mut new = *r_fmt;
        match *self {
            ThousandsSeparator(sep) => new.thousands_separator = sep,
            Precision(precision) => new.precision = precision,
            Spaces(n_spaces) => new.n_spaces = n_spaces,
        }
        new
    }
}

impl ReprConfig for ReprFormat {
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat {
        r_fmt.flags.apply(self)
    }
}

macro_rules! ok_or {
    ($value:expr, $err:expr) => {
        match ($value, $err) {
            (Some(value), _) => Ok(value),
            (_, err) => Err(err),
        }
    };
}

/// A byte size: an exact count of bytes (or bits, under the `bits` feature)
/// that knows how to render itself in binary or decimal units and how to be
/// parsed back from a string.
///
/// The count is stored as an [`Int`]. Arithmetic operators combine two sizes or
/// scale a size by a scalar; construction goes through [`of`](ByteSize::of),
/// [`from_bytes`](ByteSize::from_bytes)/[`from_bits`](ByteSize::from_bits), or
/// [`FromStr`](core::str::FromStr).
///
/// ```
/// use xbytes::prelude::*;
///
/// let size = ByteSize::of(2, GIBI_BYTE);
/// assert_eq!(size.to_string(), "2 GiB");
/// assert_eq!((size * 2).to_string(), "4 GiB");
/// ```
#[derive(Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ByteSize(Int);

impl ByteSize {
    /// Build a size from a `value` (integer or float) and a [`Unit`].
    ///
    /// The value is scaled into the backing store and truncated to a whole
    /// count. Under the default byte store that truncation is the one documented
    /// rounding rule to keep in mind: a value smaller than one byte rounds down,
    /// so `ByteSize::of(1, BIT)` is `0` bytes (0.125 truncated). Overflow
    /// saturates rather than panicking.
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let size = ByteSize::of(10, MEBI_BYTE);
    /// assert_eq!(size.to_string(), "10 MiB");
    ///
    /// let a = ByteSize::of(1.2, GIGA_BYTE);
    /// let b = ByteSize::of(1.3, GIGA_BYTE);
    /// assert_eq!((a + b).repr(Mode::Decimal).to_string(), "2.50 GB");
    /// ```
    #[must_use]
    pub fn of(value: impl Into<Float>, unit: Unit) -> Self {
        let bit_value = Float::from_int(unit.effective_value());
        // With `bits` on, the store is in bits, so a byte-denominated unit is
        // used as-is; without it the store is in bytes, so the unit's bit value
        // is scaled down by 8. That scale truncates below one byte, which is why
        // `ByteSize::of(1, BIT)` rounds to zero (see the type-level docs).
        #[cfg(feature = "bits")]
        let unit_value = bit_value;
        #[cfg(not(feature = "bits"))]
        let unit_value = bit_value.saturating_div(Float::from_small(8));

        let value = value.into().saturating_mul(unit_value);
        ByteSize(value.to_int())
    }

    /// Build a size from a whole `value` and a [`Unit`], in `const` context.
    ///
    /// The integer twin of [`of`](ByteSize::of): for whole inputs it produces the
    /// exact same value, but takes an [`Int`] instead of `impl Into<Float>` so it
    /// stays out of the float backend and can run at compile time. It cannot
    /// express a fractional quantity like `1.5 MiB`; use [`of`](ByteSize::of) for
    /// that. Overflow saturates and a sub-byte value truncates to zero, matching
    /// [`of`](ByteSize::of).
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// // Runs at compile time, and equals `of` for whole inputs:
    /// const FOUR_MIB: ByteSize = ByteSize::of_int(4, MEBI_BYTE);
    /// assert_eq!(FOUR_MIB, ByteSize::of(4, MEBI_BYTE));
    /// ```
    ///
    /// On the default byte store [`byte_count`](ByteSize::byte_count) is `const`
    /// too, so `ByteSize::of_int(4, MEBI_BYTE).byte_count()` is a `const` byte
    /// count with no hand-rolled `1024`.
    #[must_use]
    pub const fn of_int(value: Int, unit: Unit) -> Self {
        // The unit's value in bits; the byte store scales it down by eight, the
        // same truncating divide `of` does, so the two agree for whole inputs.
        let bits = value.saturating_mul(unit.effective_value());
        #[cfg(feature = "bits")]
        let store = bits;
        #[cfg(not(feature = "bits"))]
        let store = bits / 8;
        ByteSize(store)
    }

    /// Build a size from a raw bit count. Infallible, since bits are the store
    /// unit under the `bits` feature. (Without `bits`, this instead returns a
    /// [`Result`], as the bits-to-bytes division can lose precision; see the
    /// crate-level feature notes.)
    #[inline]
    #[cfg(feature = "bits")]
    #[must_use]
    pub const fn from_bits(value: Int) -> Self {
        Self(value)
    }

    /// Build a size from a raw byte count. Fallible under the `bits` feature,
    /// where the byte-to-bit multiply can overflow the store; returns
    /// [`ParseError::ValueOverflow`] if it does.
    #[inline]
    #[cfg(feature = "bits")]
    pub const fn from_bytes(value: Int) -> Result<Self, ParseError> {
        match ok_or!(value.checked_mul(8), ParseError::ValueOverflow) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    /// Build a size from a raw byte count. Infallible, since bytes are the store
    /// unit without the `bits` feature. (Under `bits`, this instead returns a
    /// [`Result`], as the byte-to-bit multiply can overflow.)
    #[inline]
    #[cfg(not(feature = "bits"))]
    #[must_use]
    pub const fn from_bytes(value: Int) -> Self {
        Self(value)
    }

    /// Build a size from a raw bit count. Fallible without the `bits` feature,
    /// where the bit-to-byte division rounds; returns
    /// [`ParseError::ValueOverflow`] on a store that cannot represent the count.
    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn from_bits(value: Int) -> Result<Self, ParseError> {
        match ok_or!(value.checked_div(8), ParseError::ValueOverflow) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    /// The raw bit count. Infallible under the `bits` feature (bits are the
    /// store unit); a [`Result`] without it, where the byte-to-bit multiply can
    /// overflow. (Named `bit_count`, not `bits`, to leave the short verb for the
    /// [`bits`](ByteSize::bits) renderer.)
    #[inline]
    #[cfg(feature = "bits")]
    #[must_use]
    pub const fn bit_count(&self) -> Int {
        self.0
    }

    /// The raw byte count. Fallible under the `bits` feature, where the
    /// bit-to-byte division would round; returns [`ParseError::ValueOverflow`]
    /// on failure.
    #[inline]
    #[cfg(feature = "bits")]
    pub const fn byte_count(&self) -> Result<Int, ParseError> {
        ok_or!(self.0.checked_div(8), ParseError::ValueOverflow)
    }

    /// The raw byte count. Infallible without the `bits` feature (bytes are the
    /// store unit); a [`Result`] under it.
    #[inline]
    #[cfg(not(feature = "bits"))]
    #[must_use]
    pub const fn byte_count(&self) -> Int {
        self.0
    }

    /// The raw bit count. Fallible without the `bits` feature, where the
    /// byte-to-bit multiply can overflow the store; returns
    /// [`ParseError::ValueOverflow`] on failure. (Named `bit_count`, not `bits`,
    /// to leave the short verb for the [`bits`](ByteSize::bits) renderer.)
    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn bit_count(&self) -> Result<Int, ParseError> {
        ok_or!(self.0.checked_mul(8), ParseError::ValueOverflow)
    }

    /// The raw byte count as an [`Int`], truncating any sub-byte remainder.
    ///
    /// Unlike [`byte_count`](ByteSize::byte_count), whose return type flips to a [`Result`] under the
    /// `bits` feature, this is always an infallible `const fn` returning `Int` on every feature setting.
    /// Reach for it when a caller (especially a `const`) wants a plain byte count without caring which
    /// store the crate was built with.
    #[inline]
    #[must_use]
    pub const fn byte_count_lossy(&self) -> Int {
        #[cfg(feature = "bits")]
        let count = self.0 / 8;
        #[cfg(not(feature = "bits"))]
        let count = self.0;
        count
    }

    /// The raw bit count as an [`Int`], saturating rather than overflowing the store.
    ///
    /// The infallible, feature-stable companion to [`bit_count`](ByteSize::bit_count), which returns a
    /// [`Result`] without the `bits` feature. Always a `const fn` returning `Int`.
    #[inline]
    #[must_use]
    pub const fn bit_count_lossy(&self) -> Int {
        #[cfg(feature = "bits")]
        let count = self.0;
        #[cfg(not(feature = "bits"))]
        let count = self.0.saturating_mul(8);
        count
    }

    /// Lift the stored count into the scalar domain the requested mode wants:
    /// bits when `Mode::Bits` is set, bytes otherwise. The store's own unit is
    /// the `bits` feature's job, so the two branches convert in opposite
    /// directions.
    fn prep_value(&self, mode: Mode) -> Float {
        let value = Float::from_int(self.0);
        let wants_bits = mode.contains(Mode::Bits);
        #[cfg(feature = "bits")]
        let convert = !wants_bits;
        #[cfg(not(feature = "bits"))]
        let convert = wants_bits;
        if !convert {
            return value;
        }
        let eight = Float::from_small(8);
        #[cfg(feature = "bits")]
        {
            value.saturating_div(eight)
        }
        #[cfg(not(feature = "bits"))]
        {
            value.saturating_mul(eight)
        }
    }

    /// Render this size at the largest prefix whose value is at least one, in
    /// the system the `mode` selects (binary by default, decimal under
    /// `Mode::Decimal`, bit- or byte-denominated, prefixed unless
    /// `Mode::NoPrefix`).
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let size = ByteSize::of(1536, KIBI_BYTE);
    /// assert_eq!(size.repr(Mode::Default).to_string(), "1.50 MiB");
    /// assert_eq!(size.repr(Mode::Decimal).to_string(), "1.57 MB");
    /// ```
    #[must_use]
    pub fn repr(&self, mode: Mode) -> ByteSizeRepr {
        let as_bits = mode.contains(Mode::Bits);
        let no_prefix = mode.contains(Mode::NoPrefix);
        let as_decimal = mode.contains(Mode::Decimal);
        let mut value = self.prep_value(mode);
        let divisor = Float::from_int(if as_decimal { 1000 } else { 1024 });
        let unit_stack = if as_bits { sizes::BITS } else { sizes::BYTES };
        let max_index = if no_prefix { 0 } else { unit_stack.len() - 1 };
        let mut prefix_index = 0;
        while prefix_index < max_index && value >= divisor {
            value = value.saturating_div(divisor);
            prefix_index += 2;
        }
        if prefix_index > 0 && as_decimal {
            prefix_index -= 1;
        }
        ByteSizeRepr::of(value, unit_stack[prefix_index])
    }

    /// Render this size at one explicit unit, however large or small the
    /// resulting number is (unlike [`repr`](ByteSize::repr), which picks the
    /// prefix for you).
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let size = ByteSize::of(1, GIBI_BYTE);
    /// assert_eq!(size.repr_as(MEBI_BYTE).to_string(), "1024 MiB");
    /// ```
    #[must_use]
    pub fn repr_as(&self, unit: impl Into<Unit>) -> ByteSizeRepr {
        let unit = unit.into();
        let value = self
            .prep_value(unit.mode())
            .saturating_div(Float::from_int(unit.effective_value()))
            .saturating_mul(Float::from_small(8));
        ByteSizeRepr::of(value, unit)
    }

    /// Render in binary (IEC, power-of-1024) units: `1.50 MiB`. The auto-prefixed
    /// default, and the fluent front door to the [`Format`] refiners on
    /// [`ByteSizeRepr`].
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let size = ByteSize::of(1536, KIBI_BYTE);
    /// assert_eq!(size.iec().to_string(), "1.50 MiB");
    /// assert_eq!(size.iec().long().precision(3).to_string(), "1.500 MebiBytes");
    /// ```
    #[must_use]
    pub fn iec(&self) -> ByteSizeRepr {
        self.repr(Mode::Default)
    }

    /// Render in decimal (SI, power-of-1000) units: `1.57 MB`.
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let size = ByteSize::of(1536, KIBI_BYTE);
    /// assert_eq!(size.si().to_string(), "1.57 MB");
    /// ```
    #[must_use]
    pub fn si(&self) -> ByteSizeRepr {
        self.repr(Mode::Decimal)
    }

    /// Render denominated in bits rather than bytes, binary-prefixed: `12 Mib`.
    /// For decimal bits, chain from the power-user [`repr`](ByteSize::repr) with
    /// `Mode::Decimal | Mode::Bits`.
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let size = ByteSize::of(1.5, MEBI_BYTE);
    /// assert_eq!(size.bits().to_string(), "12 Mib");
    /// ```
    #[must_use]
    pub fn bits(&self) -> ByteSizeRepr {
        self.repr(Mode::Bits)
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.repr(Mode::Default), f)
    }
}

macro_rules! impl_ops {
    ($($class:ident::$method:ident)+) => {
        $(
            impl std::ops::$class<Self> for ByteSize {
                type Output = ByteSize;
                fn $method(self, rhs: Self) -> Self::Output {
                    ByteSize(std::ops::$class::$method(self.0, rhs.0))
                }
            }
        )+
    };
    (@ { $($class:ident::$method:ident)+ }) => {
        $(
            impl<T: TryInto<Int>> std::ops::$class<T> for ByteSize {
                type Output = ByteSize;
                fn $method(self, rhs: T) -> Self::Output {
                    let me = Float::from_int(self.0);
                    let scaled = rhs.try_into().map_or(me, |rhs| {
                        std::ops::$class::$method(me, Float::from_int(rhs))
                    });
                    ByteSize(scaled.to_int())
                }
            }
        )+
    };
    (mut $($class:ident::$method:ident)+) => {
        $(
            impl std::ops::$class<Self> for ByteSize {
                fn $method(&mut self, rhs: Self) {
                    std::ops::$class::$method(&mut self.0, rhs.0)
                }
            }
        )+
    };
    (@ mut { $($class:ident::$method:ident)+ }) => {
        $(
            impl<T: TryInto<Int>> std::ops::$class<T> for ByteSize {
                fn $method(&mut self, rhs: T) {
                    if let Ok(rhs) = rhs.try_into() {
                        std::ops::$class::$method(&mut self.0, rhs)
                    }
                }
            }
        )+
    };
}

impl_ops!(Add::add Sub::sub);
impl_ops!(@ { Mul::mul Div::div });
impl_ops!(mut AddAssign::add_assign SubAssign::sub_assign);
impl_ops!(@ mut { MulAssign::mul_assign DivAssign::div_assign });

/// A [`ByteSize`] rendered at a specific [`Unit`], carrying a [`ReprFormat`]
/// that its [`Display`](core::fmt::Display) impl reads. Produced by
/// [`ByteSize::repr`] and [`ByteSize::repr_as`]; refine its formatting with
/// [`with`](ByteSizeRepr::with) and convert back to a [`ByteSize`] via [`From`].
///
/// Ordered first by unit magnitude, then by value, so `1 MiB < 1 MB`.
///
/// ```
/// use xbytes::prelude::*;
///
/// let repr = ByteSize::of(1, MEBI_BYTE).repr(Mode::Default);
/// assert_eq!(repr.with(Format::Long).to_string(), "1 MebiByte");
/// ```
#[cfg_attr(feature = "lossless", derive(Eq))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ByteSizeRepr(Float, Unit, ReprFormat);

// Ordered first by unit magnitude, then by scalar value, so `1 MiB` sorts
// below `1 MB` and `1 kB` below a larger `1 kB`. The lossless backend
// (`GenericFraction`) is totally ordered, so `Ord` is available and
// `partial_cmp` defers to it (the canonical, non-panicking direction); the
// `f64` backend is only `PartialOrd`, so no `Ord` impl exists there.
#[cfg(feature = "lossless")]
impl Ord for ByteSizeRepr {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Self(value, unit, _) = self;
        let Self(other_value, other_unit, _) = other;
        (unit, value).cmp(&(other_unit, other_value))
    }
}

#[cfg(feature = "lossless")]
impl PartialOrd for ByteSizeRepr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(not(feature = "lossless"))]
impl PartialOrd for ByteSizeRepr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let Self(value, unit, _) = self;
        let Self(other_value, other_unit, _) = other;
        (unit, value).partial_cmp(&(other_unit, other_value))
    }
}

impl ByteSizeRepr {
    const fn of(value: Float, unit: Unit) -> Self {
        Self(value, unit, ReprFormat::const_default())
    }

    /// Return a copy with more formatting folded in: a [`Format`] flag, a
    /// [`ReprConfigVariant`], or a whole [`ReprFormat`].
    ///
    /// This is the power-user seam. The named refiners below
    /// ([`long`](Self::long), [`precision`](Self::precision),
    /// [`separator`](Self::separator), ...) are thin, discoverable wrappers over
    /// it and are the primary documented path.
    ///
    /// ```
    /// use xbytes::prelude::*;
    ///
    /// let repr = ByteSize::of(1, KILO_BYTE).repr(Mode::Decimal);
    /// assert_eq!(repr.with(Format::Condensed | Format::NoSpace).to_string(), "1K");
    /// ```
    #[must_use]
    pub fn with(&self, conf: impl ReprConfig) -> Self {
        let Self(value, unit, format) = self;
        Self(*value, *unit, conf.apply(format))
    }

    /// Spell the unit in full: `MebiBytes`, `KiloBytes`. Pluralizes and
    /// capitalizes per [`plural`](Self::plural) and the caps refiners.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// assert_eq!(ByteSize::of(2, MEBI_BYTE).iec().long().to_string(), "2 MebiBytes");
    /// ```
    #[must_use]
    pub fn long(&self) -> Self {
        self.with(Format::Long)
    }

    /// Reduce the unit to the prefix initial only: `M`, `Ki` becomes `K`.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// assert_eq!(ByteSize::of(2, MEBI_BYTE).iec().condensed().to_string(), "2 M");
    /// ```
    #[must_use]
    pub fn condensed(&self) -> Self {
        self.with(Format::Condensed)
    }

    /// Drop the binary `i` from the symbol: `MiB` becomes `MB`, `KiB` becomes
    /// `KB`. The numeric value is unchanged, only the spelling.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// assert_eq!(ByteSize::of(2, MEBI_BYTE).iec().symbol().to_string(), "2 MB");
    /// ```
    #[must_use]
    pub fn symbol(&self) -> Self {
        self.with(Format::Initials)
    }

    /// Set the number of digits after the decimal point, and show them even for
    /// a whole value: `precision(3)` renders `1.5` as `1.500`.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// assert_eq!(ByteSize::of(1536, KIBI_BYTE).iec().precision(3).to_string(), "1.500 MiB");
    /// assert_eq!(ByteSize::of(2, MEBI_BYTE).iec().precision(2).to_string(), "2.00 MiB");
    /// ```
    #[must_use]
    pub fn precision(&self, digits: usize) -> Self {
        self.with(Precision(digits)).with(Format::ForceFraction)
    }

    /// Group the whole part into thousands with a custom separator, and turn
    /// grouping on. Pass any `&'static str`, including a multi-byte glyph.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// let size = ByteSize::of(58375, MEBI_BYTE);
    /// assert_eq!(size.iec().pin(MEBI_BYTE).separator("_").to_string(), "58_375 MiB");
    /// ```
    #[must_use]
    pub fn separator(&self, separator: &'static str) -> Self {
        self.with(ThousandsSeparator(separator))
            .with(Format::ShowThousandsSeparator)
    }

    /// Group the whole part into thousands with the default comma separator.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// let size = ByteSize::of(58375, MEBI_BYTE);
    /// assert_eq!(size.iec().pin(MEBI_BYTE).thousands().to_string(), "58,375 MiB");
    /// ```
    #[must_use]
    pub fn thousands(&self) -> Self {
        self.with(Format::ShowThousandsSeparator)
    }

    /// Re-render at one explicit unit instead of the auto-picked prefix, keeping
    /// the formatting built up so far. The fluent sibling of
    /// [`ByteSize::repr_as`].
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// let size = ByteSize::of(1, GIBI_BYTE);
    /// assert_eq!(size.iec().pin(MEBI_BYTE).to_string(), "1024 MiB");
    /// ```
    #[must_use]
    pub fn pin(&self, unit: impl Into<Unit>) -> Self {
        let Self(value, from_unit, format) = self;
        let unit = unit.into();
        // Rescale the display value from the current unit into the requested one
        // without collapsing to a whole byte count, so no precision is lost:
        // value * from_unit_bits / to_unit_bits.
        let rescaled = value
            .saturating_mul(Float::from_int(from_unit.effective_value()))
            .saturating_div(Float::from_int(unit.effective_value()));
        Self(rescaled, unit, *format)
    }

    /// Choose whether the long unit pluralizes. `plural(true)` is the default
    /// (pluralize any value that is not exactly one); `plural(false)` is the
    /// [`Format::NoPlural`] house-style pin that holds the singular even for a
    /// plural count. Grammatically-wrong forced plurals like `1 Bytes` are not
    /// expressible by design.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// let size = ByteSize::of(2, MEBI_BYTE);
    /// assert_eq!(size.iec().long().plural(true).to_string(), "2 MebiBytes");
    /// assert_eq!(size.iec().long().plural(false).to_string(), "2 MebiByte");
    /// ```
    #[must_use]
    pub fn plural(&self, plural: bool) -> Self {
        if plural {
            *self
        } else {
            self.with(Format::NoPlural)
        }
    }

    /// Drop the space between the number and the unit: `2 MiB` becomes `2MiB`.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// assert_eq!(ByteSize::of(2, MEBI_BYTE).iec().no_space().to_string(), "2MiB");
    /// ```
    #[must_use]
    pub fn no_space(&self) -> Self {
        self.with(Format::NoSpace)
    }

    /// Lowercase the whole unit: `MiB` becomes `mib`, `KB` becomes `kb`.
    ///
    /// ```
    /// use xbytes::prelude::*;
    /// assert_eq!(ByteSize::of(2, MEBI_BYTE).iec().lower().to_string(), "2 mib");
    /// ```
    #[must_use]
    pub fn lower(&self) -> Self {
        self.with(Format::LowerCaps)
    }
}

impl From<ByteSizeRepr> for ByteSize {
    fn from(repr: ByteSizeRepr) -> Self {
        let ByteSizeRepr(value, unit, _) = repr;
        ByteSize::of(value, unit)
    }
}

/// Split a run of digits into thousands groups, most-significant first.
///
/// Slices on character boundaries via `char_indices`, so no panicking
/// `from_utf8` reconstruction is needed even were the input non-ASCII:
/// the leading group carries the `len % 3` remainder, then groups of three.
///
/// ```text
/// thsep("503")     -> ["503"]
/// thsep("405503")  -> ["405", "503"]
/// thsep("1234567") -> ["1", "234", "567"]
/// ```
fn thsep(digits: &str) -> impl Iterator<Item = &str> {
    // Byte offset of every character boundary, plus the end, so every slice
    // taken below lands on a boundary and `from_utf8` is never needed.
    let bounds: Vec<usize> = digits
        .char_indices()
        .map(|(offset, _)| offset)
        .chain(std::iter::once(digits.len()))
        .collect();
    let count = bounds.len() - 1;
    let tip = count % 3;
    // Walk the boundary list in steps: a leading group of `tip` characters
    // (when the length is not a clean multiple of three) then groups of three.
    let mut start = 0;
    std::iter::from_fn(move || {
        (start < count).then(|| {
            let size = if start == 0 && tip != 0 { tip } else { 3 };
            let (lo, hi) = (bounds[start], bounds[start + size]);
            start += size;
            &digits[lo..hi]
        })
    })
}

impl fmt::Display for ByteSizeRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(repr_value, size_unit, format) = self;
        let (is_plural, has_fract);
        let flags = format.flags;

        let value_part = {
            let (mut force_fraction, no_fraction) = (
                flags.contains(Format::ForceFraction),
                flags.contains(Format::NoFraction),
            );
            let (mut value, precision) = (
                *repr_value,
                f.precision().map_or(format.precision, |precision| {
                    force_fraction = true;
                    precision
                }),
            );
            if !force_fraction && no_fraction {
                value = value.trunc();
            }
            is_plural = !value.is_one();
            has_fract = force_fraction || !(no_fraction || value.fract().is_zero());
            let mut value_part = if has_fract {
                format!("{:#.1$}", value, precision)
            } else {
                format!("{}", value)
            };
            if flags.contains(Format::ShowThousandsSeparator) {
                let (whole, fract) = value_part
                    .find('.')
                    .map_or((&value_part[..], ""), |index| value_part.split_at(index));
                let mut parts = thsep(whole);
                let mut whole = String::with_capacity(whole.len() + ((whole.len() - 1) / 3));
                whole.extend(parts.next().into_iter().chain(parts.flat_map(|s| {
                    std::iter::once(format.thousands_separator).chain(std::iter::once(s))
                })));
                value_part = format!("{}{}", whole, fract);
            }
            value_part
        };

        let spaces = {
            if !flags.contains(Format::NoSpace) {
                " ".repeat(format.n_spaces)
            } else {
                "".to_string()
            }
        };

        let unit_part = {
            let (initials, condensed, long) = (
                flags.contains(Format::Initials),
                flags.contains(Format::Condensed),
                flags.contains(Format::Long),
            );

            let mut unit = if long {
                size_unit.symbol_long(
                    !flags.contains(Format::NoPlural) && (is_plural || has_fract),
                    !flags.contains(Format::NoMultiCaps),
                )
            } else if condensed {
                size_unit.symbol_condensed().to_string()
            } else if initials {
                size_unit.symbol_initials()
            } else {
                size_unit.symbol()
            };
            if flags.contains(Format::UpperCaps) {
                unit = unit.to_uppercase()
            } else if flags.contains(Format::LowerCaps) {
                unit = unit.to_lowercase()
            }
            unit
        };

        write!(f, "{}{}{}", value_part, spaces, unit_part)
    }
}

impl FromStr for ByteSize {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            let (mut commas, mut cursor, mut frac_pos) = (0, 0, None);
            let index = s
                .find(|c| {
                    #[rustfmt::skip]
                    if frac_pos.is_none() {
                        if matches!(c, '.') { frac_pos = Some(cursor) };
                        if matches!(c, ',') { commas += 1 };
                    };
                    cursor += 1;
                    c.is_alphabetic() || c.is_whitespace()
                })
                .ok_or(ParseError::MissingUnit)?;
            if matches!(index, 0) {
                Err(ParseError::MissingValue)?
            }
            let (value, unit) = s.split_at(index);
            let value = if !matches!(commas, 0) {
                {
                    // ensure proper comma alignment
                    //  • valid   : '1,203.34' '10,293,344'
                    //  • invalid : '1,23,45' '1,2,3,4.342'
                    let value = &value[..frac_pos.unwrap_or(value.len())];
                    let mut parts = value.split(',');
                    #[rustfmt::skip]
                    if !({
                        if !matches!((value.len() - commas) % 3, 0) { parts.next() } else { None }
                            .is_none_or(|tip| tip.len() < 3)
                    } && parts.all(|part| part.len() == 3))
                    { Err(ParseError::InvalidThousandsFormat)? };
                }
                Float::parse(&value.replacen(',', "", commas))
            } else {
                Float::parse(value)
            }
            .ok_or(ParseError::InvalidValue)?;
            let unit = unit
                .trim_start_matches(|c: char| c.is_whitespace())
                .parse()?;
            Ok(ByteSize::of(value, unit))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sizes::*;
    use super::*;

    /// Lift a literal into the active `Float` backend, for building fixtures.
    macro_rules! f {
        ($value:expr) => {{
            #[cfg(feature = "lossless")]
            let val = Float::from($value);
            #[cfg(not(feature = "lossless"))]
            let val = $value as Float;
            val
        }};
    }

    /// Select a fixture expression by the `bits` feature, so a test can assert
    /// against whichever store the crate was compiled with.
    macro_rules! exec {
        (@ bits $expr:block) => {
            #[cfg(feature = "bits")] break $expr
        };
        (@ nobits $expr:block) => {
            #[cfg(not(feature = "bits"))] break $expr
        };
        ($($term:tt { $expr:expr }),+) => {
            loop { $( exec!(@ $term { $expr }); )+ }
        };
    }

    #[test]
    fn thousands_grouping() {
        fn group(s: &str) -> Vec<&str> {
            thsep(s).collect()
        }
        assert_eq!(group(""), Vec::<&str>::new());
        assert_eq!(group("5"), ["5"]);
        assert_eq!(group("503"), ["503"]);
        assert_eq!(group("1234"), ["1", "234"]);
        assert_eq!(group("405503"), ["405", "503"]);
        assert_eq!(group("1234567"), ["1", "234", "567"]);
    }

    #[test]
    fn bytesize() {
        let bytes = 1048576;

        let size = exec! {
            bits { ByteSize::from_bits(bytes * 8) },
            nobits { ByteSize::from_bytes(bytes) }
        };

        assert_eq!("1 MiB", format!("{}", size));
    }

    #[test]
    fn bytesize_from_cmp() {
        let left = exec! {
            bits { ByteSize::from_bits(8388608) },
            nobits { ByteSize::from_bytes(1048576) }
        };

        let right = exec! {
            bits { ByteSize::from_bytes(1048576).unwrap() },
            nobits { ByteSize::from_bits(8388608).unwrap() }
        };

        assert_eq!(left, right);
    }

    #[test]
    fn mode_default() {
        assert_eq!(Mode::default(), Mode::Default);
    }

    #[test]
    fn format_default() {
        assert_eq!(Format::default(), Format::Default);
    }

    #[test]
    fn repr_format_default() {
        assert_eq!(
            ReprFormat {
                flags: Format::Default,
                n_spaces: 1,
                precision: 2,
                thousands_separator: ","
            },
            ReprFormat::default()
        )
    }

    #[test]
    fn byte_size_repr_eq() {
        let l = ByteSizeRepr::of(f!(104.5), TEBI_BYTE);
        let r = ByteSizeRepr::of(f!(104.5), TEBI_BYTE);

        assert_eq!(l, r); // 104.50 TiB == 104.50 TiB
        assert_ne!(l.with(Precision(4)), r); // 104.5000 TiB != 104.50 TiB
        assert_ne!(l.with(Format::Long), r); // 104.50 TebiBytes != 104.50 TiB
        assert_ne!(
            l.with(Format::Long | Format::NoMultiCaps),
            r.with(Format::Long)
        ); // 104.50 Tebibytes != 104.50 TebiBytes
        assert_ne!(
            l.with(Format::Initials | Format::NoFraction),
            r.with(Format::Condensed | Format::NoSpace)
        ); // 104 TB != 104.50T

        let format = ReprFormat::default()
            .with(Spaces(2))
            .with(Precision(2))
            .with(ThousandsSeparator("_"))
            .with(Format::Long)
            .with(Format::ShowThousandsSeparator);

        assert_eq!(l.with(format), r.with(format));
    }

    #[test]
    fn byte_size_repr_cmp() {
        let l = ByteSizeRepr::of(f!(1), MEBI_BYTE);
        let r = ByteSizeRepr::of(f!(1), MEGA_BYTE);
        println!("{}", l < r); // 1 MiB < 1 MB

        let l = ByteSizeRepr::of(f!(1), KILO_BYTE);
        let r = ByteSizeRepr::of(f!(1), KILO_BIT);
        println!("{}", l > r); // 1 kB > 1 kb

        let l = ByteSizeRepr::of(f!(1), GIGA_BYTE);
        let r = ByteSizeRepr::of(f!(1), PEBI_BYTE);
        println!("{}", l < r); // 1 GB < 1 PiB
    }

    #[test]
    fn byte_size_repr_to_string() {
        let repr = ByteSizeRepr::of(f!(58375.284), EXBI_BYTE);

        assert_eq!("58375.28 EiB", repr.to_string());
        assert_eq!(
            "58,375.28 EiB",
            repr.with(Format::ShowThousandsSeparator).to_string()
        );
        assert_eq!(
            "58375.28 EiB",
            repr.with(ThousandsSeparator("_")).to_string()
        );
        assert_eq!("58375.28EiB", repr.with(Format::NoSpace).to_string());
        assert_eq!(
            "58_375.28    EiB",
            repr.with(Spaces(4))
                .with(ThousandsSeparator("_"))
                .with(Format::ShowThousandsSeparator)
                .to_string()
        );
        assert_eq!(
            "58,375.2840  EiB",
            repr.with(Spaces(2))
                .with(Precision(4))
                .with(Format::ShowThousandsSeparator)
                .to_string()
        );
    }

    #[test]
    fn bytesize_of() {
        assert_eq!(
            exec! {
                bits { ByteSize(8) },
                nobits { ByteSize(1) }
            },
            ByteSize::of(1, BYTE)
        );

        assert_eq!(
            exec! {
                bits { ByteSize(1) },
                nobits { ByteSize(0) } // 0.125 (saturated)
            },
            ByteSize::of(1, BIT)
        );

        assert_eq!(
            exec! {
                bits { ByteSize(8388608) },
                nobits { ByteSize(1048576) }
            },
            ByteSize::of(1, MEBI_BYTE)
        );

        assert_eq!(
            exec! {
                bits { ByteSize(1048576) },
                nobits { ByteSize(131072) }
            },
            ByteSize::of(1, MEBI_BIT)
        );

        #[cfg(feature = "u128")]
        assert_eq!(
            exec! {
                bits { ByteSize(9671406556917033397649408) },
                nobits { ByteSize(1208925819614629174706176) }
            },
            ByteSize::of(1, YOBI_BYTE)
        );

        #[cfg(feature = "u128")]
        assert_eq!(
            exec! {
                bits { ByteSize(1208925819614629174706176) },
                nobits { ByteSize(151115727451828646838272) }
            },
            ByteSize::of(1, YOBI_BIT)
        );
    }

    #[test]
    fn ops() {
        let a = ByteSize::of(1, MEGA_BYTE);
        let b = ByteSize::of(2, MEGA_BYTE);
        let sum = a + b;
        assert_eq!(ByteSize::of(3, MEGA_BYTE), sum);

        let a = ByteSize::of(4, GIGA_BYTE);
        let b = ByteSize::of(2, GIBI_BYTE);
        let sub = a - b;
        assert_eq!(ByteSize::of(1.7252902985, GIBI_BYTE), sub);

        let size = ByteSize::of(5, MEGA_BYTE);
        let size_x5 = size * 5;
        assert_eq!(ByteSize::of(25, MEGA_BYTE), size_x5);

        let size = ByteSize::of(1, GIBI_BYTE);
        let size_by_1024 = size / 1024;
        assert_eq!(ByteSize::of(1, MEBI_BYTE), size_by_1024);
    }

    #[test]
    fn ops_assign() {
        let mut size = ByteSize::of(5, GIBI_BYTE);
        size += ByteSize::of(5.5, GIBI_BYTE);
        assert_eq!(ByteSize::of(10.5, GIBI_BYTE), size);

        let mut size = ByteSize::of(50, GIBI_BYTE);
        size -= ByteSize::of(10, GIBI_BYTE);
        assert_eq!(ByteSize::of(40, GIBI_BYTE), size);

        let mut size = ByteSize::of(1, GIBI_BYTE);
        size *= 512;
        assert_eq!(ByteSize::of(512, GIBI_BYTE), size);

        let mut size = ByteSize::of(1, TERA_BYTE);
        size /= 8;
        assert_eq!(ByteSize::of(1, TERA_BIT), size);
    }

    #[test]
    fn byte_size_mode() {
        let size = ByteSize::of(1.50, MEBI_BYTE);

        assert_eq!("1.50 MiB", size.to_string());
        assert_eq!("12 Mib", size.repr(Mode::Bits).to_string());
        assert_eq!("1.57 MB", size.repr(Mode::Decimal).to_string());
        assert_eq!(
            "12.58 Mb",
            size.repr(Mode::Decimal | Mode::Bits).to_string()
        );
    }

    #[test]
    fn format_plurality() {
        let repr_1 = ByteSize::of(1, MEGA_BYTE).repr(Mode::Decimal);
        let repr_2 = ByteSize::of(2, MEGA_BYTE).repr(Mode::Decimal);

        assert_eq!("1 MegaByte", repr_1.with(Format::Long).to_string());
        assert_eq!("2 MegaBytes", repr_2.with(Format::Long).to_string());

        // NoPlural pins the singular even for a plural count.
        assert_eq!(
            "2 MegaByte",
            repr_2.with(Format::Long | Format::NoPlural).to_string()
        );
    }

    #[test]
    fn format_fractions() {
        let repr_1 = ByteSize::of(1, MEGA_BYTE).repr(Mode::Decimal);
        let repr_1_2 = ByteSize::of(1.2, MEGA_BYTE).repr(Mode::Decimal);
        let repr_2 = ByteSize::of(2, MEGA_BYTE).repr(Mode::Decimal);
        let repr_2_7 = ByteSize::of(2.7234258, MEGA_BYTE).repr(Mode::Decimal);

        assert_eq!("1 MB", format!("{}", repr_1));
        assert_eq!("1.20 MB", format!("{}", repr_1_2));
        assert_eq!("2 MB", format!("{}", repr_2));
        assert_eq!("2.72 MB", format!("{}", repr_2_7));
        // --
        assert_eq!("1.0000 MB", format!("{:.4}", repr_1));
        assert_eq!("1.2000 MB", format!("{:.4}", repr_1_2));
        assert_eq!("2.0000 MB", format!("{:.4}", repr_2));
        assert_eq!("2.7234 MB", format!("{:.4}", repr_2_7));
        // --
        let force_fraction = Format::ForceFraction;
        assert_eq!("1.00 MB", format!("{}", repr_1.with(force_fraction)));
        assert_eq!("1.20 MB", format!("{}", repr_1_2.with(force_fraction)));
        assert_eq!("2.00 MB", format!("{}", repr_2.with(force_fraction)));
        assert_eq!("2.72 MB", format!("{}", repr_2_7.with(force_fraction)));
        // --
        let no_fraction = Format::NoFraction;
        assert_eq!("1 MB", format!("{}", repr_1.with(no_fraction)));
        assert_eq!("1 MB", format!("{}", repr_1_2.with(no_fraction)));
        assert_eq!("2 MB", format!("{}", repr_2.with(no_fraction)));
        assert_eq!("2 MB", format!("{}", repr_2_7.with(no_fraction)));
        // --
        // the format spec's `precision (.4)` took precedence over repr config's `NoFraction`
        // and forced the representation to use fractions
        assert_eq!("1.0000 MB", format!("{:.4}", repr_1.with(no_fraction)));
        assert_eq!("1.2000 MB", format!("{:.4}", repr_1_2.with(no_fraction)));
        assert_eq!("2.0000 MB", format!("{:.4}", repr_2.with(no_fraction)));
        assert_eq!("2.7234 MB", format!("{:.4}", repr_2_7.with(no_fraction)));
        // --
        let precision = ReprFormat::default()
            .with(Precision(4))
            .with(Format::ForceFraction);
        assert_eq!("1.0000 MB", format!("{}", repr_1.with(precision)));
        assert_eq!("1.2000 MB", format!("{}", repr_1_2.with(precision)));
        assert_eq!("2.0000 MB", format!("{}", repr_2.with(precision)));
        assert_eq!("2.7234 MB", format!("{}", repr_2_7.with(precision)));
        // --
        // the format spec's `precision (.2)` took precedence over repr config's `Precision(4)`
        assert_eq!("1.00 MB", format!("{:.2}", repr_1.with(precision)));
        assert_eq!("1.20 MB", format!("{:.2}", repr_1_2.with(precision)));
        assert_eq!("2.00 MB", format!("{:.2}", repr_2.with(precision)));
        assert_eq!("2.72 MB", format!("{:.2}", repr_2_7.with(precision)));
    }

    #[test]
    fn format_repr() {
        let repr = ByteSize::of(1.59, MEGA_BYTE).repr(Mode::Decimal);

        // a fractional value pluralizes the long unit
        assert_eq!("1.59 MegaBytes", repr.with(Format::Long).to_string());

        // truncating the fraction drops back to a singular whole
        assert_eq!(
            "1 MegaByte",
            repr.with(Format::Long | Format::NoFraction).to_string()
        );

        // NoPlural holds the singular even with a fraction present
        assert_eq!(
            "1.59 MegaByte",
            repr.with(Format::Long | Format::NoPlural).to_string()
        );

        // Condensed alone leaves just the prefix initial
        assert_eq!(
            "1.59M",
            repr.with(Format::Condensed | Format::NoSpace).to_string()
        );

        // Long wins when both Long and Condensed are set
        assert_eq!(
            "1.59MegaBytes",
            repr.with(Format::Long | Format::Condensed | Format::NoSpace)
                .to_string()
        );
    }

    #[test]
    fn fluent_api() {
        let size = ByteSize::of(1536, KIBI_BYTE);

        // mode entries
        assert_eq!("1.50 MiB", size.iec().to_string());
        assert_eq!("1.57 MB", size.si().to_string());
        assert_eq!("12 Mib", size.bits().to_string());

        // spelling refiners, chainable
        assert_eq!("1.50 MebiBytes", size.iec().long().to_string());
        assert_eq!("1.50 M", size.iec().condensed().to_string());
        assert_eq!("1.50 MB", size.iec().symbol().to_string());
        assert_eq!("1.50 mib", size.iec().lower().to_string());
        assert_eq!("1.50MiB", size.iec().no_space().to_string());

        // numeric refiners
        assert_eq!("1.500 MiB", size.iec().precision(3).to_string());
        assert_eq!(
            "1.500 MebiBytes",
            size.iec().long().precision(3).to_string()
        );

        // pin + grouping
        let big = ByteSize::of(58375, MEBI_BYTE);
        assert_eq!(
            "58,375 MiB",
            big.iec().pin(MEBI_BYTE).thousands().to_string()
        );
        assert_eq!(
            "58_375 MiB",
            big.iec().pin(MEBI_BYTE).separator("_").to_string()
        );

        // plural control
        let two = ByteSize::of(2, MEBI_BYTE);
        assert_eq!("2 MebiBytes", two.iec().long().plural(true).to_string());
        assert_eq!("2 MebiByte", two.iec().long().plural(false).to_string());

        // pin is lossless: 1 GiB shown at MiB is exactly 1024
        assert_eq!(
            "1024 MiB",
            ByteSize::of(1, GIBI_BYTE).iec().pin(MEBI_BYTE).to_string()
        );
    }

    #[test]
    fn parse_thousands_separator() {
        assert_eq!(
            Ok(ByteSize::of(1_024, MEBI_BYTE)),
            "1,024 MiB".parse::<ByteSize>()
        );

        assert_eq!(
            Ok(ByteSize::of(268_435_456, KIBI_BYTE)),
            "268,435,456 KiB".parse::<ByteSize>()
        );

        assert_eq!(
            Ok(ByteSize::of(43456.2466, KIBI_BYTE)),
            "43,456.2466 KiB".parse::<ByteSize>()
        );

        assert_eq!(
            Err(ParseError::InvalidThousandsFormat),
            "434,56.53 KiB".parse::<ByteSize>()
        );

        assert_eq!(
            Err(ParseError::InvalidThousandsFormat),
            "2,68,43,54,56 KiB".parse::<ByteSize>()
        );
    }

    #[test]
    fn format_then_parse_round_trips() {
        // A size rendered at a pinned unit and full precision parses back to the same size, so the
        // formatter and the parser are inverse over the fluent front door.
        for (value, unit) in [(1536, KIBI_BYTE), (58375, MEBI_BYTE), (2048, GIBI_BYTE)] {
            let size = ByteSize::of(value, unit);
            let text = size.iec().pin(unit).precision(10).to_string();
            assert_eq!(
                Ok(size),
                text.parse::<ByteSize>(),
                "{text:?} should parse back to its own size"
            );
        }

        // The default Display (auto-scaled, two decimals) also round-trips for an exactly
        // representable value.
        let size = ByteSize::of(1536, KIBI_BYTE);
        assert_eq!(Ok(size), size.to_string().parse::<ByteSize>());
    }

    #[test]
    fn of_int_matches_of_for_whole_inputs() {
        // The const integer constructor agrees with `of` across units and the sub-byte truncation, so a
        // caller can reach for whichever fits without a value surprise.
        for (value, unit) in [
            (4, MEBI_BYTE),
            (10, GIBI_BYTE),
            (1536, KIBI_BYTE),
            (1, BYTE),
            (7, BIT), // under one byte: truncates to zero, like `of`
        ] {
            assert_eq!(
                ByteSize::of_int(value, unit),
                ByteSize::of(value, unit),
                "of_int and of disagree for {value} {unit:?}"
            );
        }
    }

    #[test]
    fn of_int_is_usable_in_a_const() {
        const FOUR_MIB: ByteSize = ByteSize::of_int(4, MEBI_BYTE);
        assert_eq!(FOUR_MIB, ByteSize::of(4, MEBI_BYTE));
    }

    #[test]
    fn lossy_counts_are_feature_stable_infallible_ints() {
        // byte_count/bit_count flip to a Result under the `bits` feature; the lossy pair always returns
        // an Int, and the same value, on either store. These assertions hold with or without `bits`.
        let size = ByteSize::of_int(4, MEBI_BYTE);
        assert_eq!(size.byte_count_lossy(), 4 * 1024 * 1024);
        assert_eq!(size.bit_count_lossy(), 4 * 1024 * 1024 * 8);

        // Usable in a const on any feature setting, which is the point.
        const BYTES: u64 = ByteSize::of_int(2, KIBI_BYTE).byte_count_lossy() as u64;
        assert_eq!(BYTES, 2048);
    }
}
