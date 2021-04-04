use super::{sizes, Float, Int, ParseError, Unit};
use std::fmt;
mod flags {
    #![allow(non_upper_case_globals)]

    use bitflags::bitflags;

    bitflags! {
        #[derive(Default)]
        pub struct Mode: u8 {
            const Default  = 0 << 0;
            const Bits     = 1 << 0;
            const Decimal  = 1 << 1;
            const NoPrefix = 1 << 2;
        }
    }

    bitflags! {
        #[derive(Default)]
        pub struct Format: u16 {
            const Default            = 0 << 0;
            const Initials           = 1 << 0;
            const Condensed          = 1 << 1;
            const Long               = 1 << 2;
            const NoPlural           = 1 << 3;
            const ForcePlural        = 1 << 4;
            const NoMultiCaps        = 1 << 5;
            const LowerCaps          = 1 << 6;
            const UpperCaps          = 1 << 7;
            const NoFraction         = 1 << 8;
            const ForceFraction      = 1 << 9;
            const ThousandsSeparator = 1 << 10;
            const NoSpace            = 1 << 11;
        }
    }
}

pub use flags::*;

#[derive(Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ReprFormat {
    flags: Format,
    precision: usize,
}

impl Default for ReprFormat {
    fn default() -> Self {
        Self::default()
    }
}

impl ReprFormat {
    const FLAGS: Format = Format::empty();

    const fn default() -> Self {
        Self {
            flags: Self::FLAGS,
            precision: 2,
        }
    }

    pub const fn with_format(&self, format: Format) -> Self {
        let mut new = *self;
        new.flags = bitflags_const_or!(Format::{new.flags, format});
        new
    }

    pub const fn with_precision(&self, precision: usize) -> Self {
        let mut new = *self;
        new.precision = precision;
        new
    }

    pub const fn reset_flags(&self) -> Self {
        let mut new = *self;
        new.flags = Self::FLAGS;
        new
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

#[derive(Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ByteSize(Int);

impl ByteSize {
    #[inline]
    #[cfg(feature = "bits")]
    pub const fn from_bits(value: Int) -> Self {
        Self(value)
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn from_bytes(value: Int) -> Result<Self, ParseError> {
        match ok_or!(value.checked_mul(8), ParseError::ValueOverflow) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn from_bytes(value: Int) -> Self {
        Self(value)
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn from_bits(value: Int) -> Result<Self, ParseError> {
        match ok_or!(value.checked_div(8), ParseError::ValueOverflow) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn bits(&self) -> Int {
        self.0
    }

    #[inline]
    #[cfg(feature = "bits")]
    pub const fn bytes(&self) -> Result<Int, ParseError> {
        ok_or!(self.0.checked_div(8), ParseError::ValueOverflow)
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn bytes(&self) -> Int {
        self.0
    }

    #[inline]
    #[cfg(not(feature = "bits"))]
    pub const fn bits(&self) -> Result<Int, ParseError> {
        ok_or!(self.0.checked_mul(8), ParseError::ValueOverflow)
    }

    #[rustfmt::skip]
    fn prep_value(&self, mode: Mode) -> Float {
        let value = f!(self.0);
        let wants_bits = mode.contains(Mode::Bits);
        if exec! {
            bits { !wants_bits },
            nobits { wants_bits }
        } {
            exec! {
                bits { value / f!(8) },
                nobits { exec! {
                    unsafe { value * f!(8) },
                    safely { saturate!(value.checked_mul(&{ f!(8) })) }
                } }
            }
        } else { value }
    }

    #[rustfmt::skip]
    pub fn repr(&self, mode: Mode) -> ByteSizeRepr {
        let as_bits = mode.contains(Mode::Bits);
        let no_prefix = mode.contains(Mode::NoPrefix);
        let as_decimal = mode.contains(Mode::Decimal);
        let mut value = self.prep_value(mode);
        let divisor = if as_decimal { f!(1000) } else { f!(1024) };
        let unit_stack = if as_bits { sizes::BITS } else { sizes::BYTES };
        let max_index = if no_prefix { 0 } else { unit_stack.len() - 1 };
        let mut prefix_index = 0;
        while prefix_index < max_index && value >= divisor {
            value /= divisor;
            prefix_index += 2;
        }
        if prefix_index > 0 && as_decimal { prefix_index -= 1 }
        ByteSizeRepr::of(value, unit_stack[prefix_index])
    }

    pub fn repr_as(&self, unit: impl Into<Unit>) -> ByteSizeRepr {
        let unit = unit.into();

        let value = self.prep_value(unit.mode()) / f!(unit.effective_value());
        let value = exec! {
            unsafe { value * f!(8) },
            safely { saturate!(value.checked_mul(&{ f!(8) })) }
        };

        ByteSizeRepr::of(value, unit)
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.repr(Mode::empty()), f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ByteSizeRepr(Float, Unit, ReprFormat);

impl ByteSizeRepr {
    const fn of(value: Float, unit: Unit) -> Self {
        Self(value, unit, ReprFormat::default())
    }

    pub const fn with(&self, format: Format) -> Self {
        let mut new = *self;
        new.2 = new.2.with_format(format);
        new
    }

    pub const fn with_precision(&self, precision: usize) -> Self {
        let mut new = *self;
        new.2 = new.2.with_precision(precision);
        new
    }
}

// thousands separator
// thsep("503") -> ['503']
// thsep("405503") -> ['405', '503']
// thsep("1234567") -> ['1', '234', '567']
fn thsep(digits: &str) -> (usize, usize, impl Iterator<Item = &str>) {
    let chars = digits.as_bytes();
    let len = chars.len();
    let part = len / 3;
    let tip = len - (part * 3);
    let tip_chars = &chars[..tip];
    (
        len,
        (tip_chars.is_empty()).then(|| part - 1).unwrap_or(part),
        std::iter::from_fn(move || (!tip_chars.is_empty()).then(|| tip_chars))
            .take(1)
            .chain(chars[tip..].chunks(3))
            .map(|digits| {
                std::str::from_utf8(digits).expect("where did the non-utf8 character come from?")
            }),
    )
}

impl fmt::Display for ByteSizeRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = self.2.flags;
        let long = f.sign_plus() || flags.contains(Format::Long);
        let plural = flags.contains(Format::ForcePlural)
            || (f.sign_plus() && f.alternate())
            || !flags.contains(Format::NoPlural);
        let condensed = (f.sign_minus() && f.alternate()) || flags.contains(Format::Condensed);
        let initials = f.sign_minus() && !f.alternate() || flags.contains(Format::Initials);
        let precision = self.2.precision;
        let value = self.0;

        if flags.contains(Format::NoFraction) {
            value = value.trunc();
        }

        let mut value_part = if flags.contains(Format::ForceFraction) || !f_is_zero!(value.fract())
        {
            format!("{:.1$}", value, precision)
        } else {
            format!("{}", value)
        };

        if flags.contains(Format::ThousandsSeparator) {
            let (whole, fract) = value_part
                .find('.')
                .map_or((&value_part[..], ""), |index| value_part.split_at(index));
            let (len, holes, parts) = thsep(whole);
            let mut whole = Vec::with_capacity(len + holes);
            whole.extend(parts);
            value_part = format!("{}{}", whole.join(","), fract);
        }

        #[rustfmt::skip]
        let unit_part = format!(
            "{}{}",
            if !flags.contains(Format::NoSpace) { " " } else { "" },
            self.1
        );

        write!(f, "{}{}", value_part, unit_part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytesize() {
        let bytes = 1048576;

        let size = exec! {
            bits { ByteSize::from_bits(bytes * 8) },
            nobits { ByteSize::from_bytes(bytes) }
        };

        assert_eq!("1 MiB", format!("{}", size));
    }
}
