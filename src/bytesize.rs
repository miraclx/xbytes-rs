use super::{sizes, Float, Int, ParseError, Unit};
use std::{convert::TryInto, fmt, str::FromStr};

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
            const Default                = 0 << 0; // 1 B, 2.13 KB, 1024.43 MiB

            const Initials               = 1 << 0; // 1 B, 2.13 K, 1024.43 M
            const Condensed              = 1 << 1; // 1 B, 2.13 KB, 1024.43 MB
            const Long                   = 1 << 2; // 1 Byte, 2.13 KiloBytes, 1024.43 MebiBytes

            // (requires Long)
            const NoPlural               = 1 << 3; // 1 Byte, 2.13 KiloByte, 1024.43 MebiByte
            const ForcePlural            = 1 << 4; // 1 Bytes, 2.13 KiloBytes, 1024.43 MebiBytes

            // (requires Long)
            const NoMultiCaps            = 1 << 5; // 1 Byte, 2.13 Kilobytes, 1024.43 Mebibytes

            const LowerCaps              = 1 << 6; // 1 b, 2.13 kb, 1024.43 mib
            const UpperCaps              = 1 << 7; // 1 B, 2.13 KB, 1024.43 MIB

            const NoFraction             = 1 << 8; // 1 B, 2 KB, 1024 MiB
            const ForceFraction          = 1 << 9; // 1.00 B, 2.13 KB, 1024.43 MiB

            const ShowThousandsSeparator = 1 << 10; // 1 B, 2.13 KB, 1,024.43 MiB
            const NoSpace                = 1 << 11; // 1B, 2.13KB, 1024.43MiB
        }
    }
}

pub use flags::*;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct ReprFormat {
    flags: Format,
    n_spaces: usize,
    precision: usize,
    thousands_separator: &'static str,
}

impl ReprFormat {
    const fn default() -> Self {
        Self {
            flags: Format::Default,
            n_spaces: 1,
            precision: 2,
            thousands_separator: ",",
        }
    }

    pub fn with(&self, conf: impl ReprConfig) -> Self {
        conf.apply(self)
    }
}

pub trait ReprConfig {
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

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum ReprConfigVariant {
    ThousandsSeparator(&'static str),
    Precision(usize),
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

#[derive(Eq, Ord, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ByteSize(Int);

impl ByteSize {
    pub fn of(value: impl Into<Float>, unit: Unit) -> Self {
        let u_value = exec! {
            bits { unit.effective_value() },
            nobits { match unit.effective_value().checked_div(8) {
                Some(value) => value,
                None => 0,
            } }
        };

        let value = exec! {
            unsafe { value.into() * f!(u_value) },
            safely { saturate!(value.into().checked_mul(&{ f!(u_value) })) }
        };

        ByteSize(i!(value))
    }

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
                    let me = f!(self.0);
                    ByteSize(
                        i!(
                            rhs.try_into()
                                .map_or(me, |rhs| std::ops::$class::$method(me, f!(rhs)))
                        ),
                    )
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

#[cfg_attr(feature = "lossless", derive(Eq))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ByteSizeRepr(Float, Unit, ReprFormat);

#[cfg(feature = "lossless")]
impl Ord for ByteSizeRepr {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ByteSizeRepr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.1, self.0).partial_cmp(&(other.1, other.0))
    }
}

impl ByteSizeRepr {
    const fn of(value: Float, unit: Unit) -> Self {
        Self(value, unit, ReprFormat::default())
    }

    pub fn with(&self, conf: impl ReprConfig) -> Self {
        Self {
            2: conf.apply(&self.2),
            ..*self
        }
    }
}

impl From<ByteSizeRepr> for ByteSize {
    fn from(repr: ByteSizeRepr) -> Self {
        ByteSize::of(repr.0, repr.1)
    }
}

// thousands separator
// thsep("503") -> ['503']
// thsep("405503") -> ['405', '503']
// thsep("1234567") -> ['1', '234', '567']
fn thsep(digits: &str) -> impl Iterator<Item = &str> {
    let (chars, tip) = (digits.as_bytes(), digits.len() % 3);
    if tip != 0 { Some(&chars[..tip]) } else { None }
        .into_iter()
        .chain(chars[tip..].chunks(3))
        .map(|digits| std::str::from_utf8(digits).expect("unexpected non-utf8 char encountered"))
}

impl fmt::Display for ByteSizeRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (is_plural, has_fract);
        let flags = self.2.flags;

        let value_part = {
            let (mut force_fraction, no_fraction) = (
                flags.contains(Format::ForceFraction),
                flags.contains(Format::NoFraction),
            );
            let (mut value, precision) = (
                self.0,
                f.precision().map_or(self.2.precision, |precision| {
                    force_fraction = true;
                    precision
                }),
            );
            if !force_fraction && no_fraction {
                value = value.trunc();
            }
            is_plural = !f_is_one!(value);
            has_fract = force_fraction || !(no_fraction || f_is_zero!(value.fract()));
            let mut value_part = if has_fract {
                format!("{:.1$}", value, precision)
            } else {
                format!("{}", value)
            };
            let (whole, fract) = match value_part.find('.') {
                Some(index) => {
                    #[cfg(feature = "lossless")]
                    value_part.extend(
                        std::iter::repeat('0').take(precision - (value_part.len() - index - 1)),
                    );
                    value_part.split_at(index)
                }
                None => {
                    #[cfg(feature = "lossless")]
                    if force_fraction {
                        value_part.extend(
                            std::iter::once('.').chain(std::iter::repeat('0').take(precision)),
                        );
                    }
                    (&value_part[..], "")
                }
            };

            if flags.contains(Format::ShowThousandsSeparator) {
                let mut parts = thsep(whole);
                let mut whole = String::with_capacity(whole.len() + ((whole.len() - 1) / 3));
                whole.extend(parts.next().into_iter().chain(parts.flat_map(|s| {
                    std::iter::once(self.2.thousands_separator).chain(std::iter::once(s))
                })));
                value_part = format!("{}{}", whole, fract);
            }
            value_part
        };

        let spaces = {
            if !flags.contains(Format::NoSpace) {
                " ".repeat(self.2.n_spaces)
            } else {
                "".to_string()
            }
        };

        let unit_part = {
            let (sign_minus, alternate, sign_plus) = (f.sign_minus(), f.alternate(), f.sign_plus());

            let (initials, condensed, long) = if sign_minus || alternate || sign_plus {
                (
                    (sign_minus && !alternate),
                    (sign_minus && alternate),
                    sign_plus,
                )
            } else {
                (
                    flags.contains(Format::Initials),
                    flags.contains(Format::Condensed),
                    flags.contains(Format::Long),
                )
            };

            let mut unit = if long {
                self.1.symbol_long(
                    (flags.contains(Format::ForcePlural) || (sign_plus && alternate))
                        || (!flags.contains(Format::NoPlural) && (is_plural || has_fract)),
                    !flags.contains(Format::NoMultiCaps),
                )
            } else if condensed {
                self.1.symbol_condensed().to_string()
            } else if initials {
                self.1.symbol_initials()
            } else {
                self.1.symbol()
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
            let (mut commas, mut past_fraction) = (0, false);
            let index = s
                .find(|c| {
                    past_fraction |= matches!(c, '.');
                    if !past_fraction && matches!(c, ',') {
                        commas += 1
                    };
                    c.is_alphabetic() || c.is_whitespace()
                })
                .ok_or(ParseError::MissingUnit)?;
            if matches!(index, 0) {
                Err(ParseError::MissingValue)?
            }
            let (value, unit) = s.split_at(index);
            let value: f64 = if !matches!(commas, 0) {
                value.replacen(',', "", commas).parse()
            } else {
                value.parse()
            }
            .map_err(|_| ParseError::InvalidValue)?;
            let unit = unit
                .trim_start_matches(|c: char| c.is_whitespace())
                .parse()?;
            Ok(ByteSize::of(value, unit))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sizes::*, *};

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

        assert_eq!("1 MegaByte", format!("{:+}", repr_1));
        assert_eq!("2 MegaBytes", format!("{:+}", repr_2));
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
        // format specs take higher precedence over repr config
        let repr = ByteSize::of(1.59, MEGA_BYTE).repr(Mode::Decimal);

        assert_eq!("1.59 MegaBytes", format!("{:+}", repr));

        assert_eq!("1 MegaByte", format!("{:+}", repr.with(Format::NoFraction)));

        assert_eq!(
            "1 MegaBytes",
            format!("{:+#}", repr.with(Format::NoFraction))
        );

        assert_eq!(
            "1.59 MegaByte",
            format!("{:+}", repr.with(Format::NoPlural))
        );

        // the format spec's `plural (+#)` took precedence over repr config's `NoPlural`
        assert_eq!(
            "1.59 MegaBytes",
            format!("{:+#}", repr.with(Format::NoPlural))
        );

        assert_eq!(
            "1 MegaBytes",
            format!("{:+}", repr.with(Format::NoFraction | Format::ForcePlural))
        );

        // the format spec's `condensed (-#)` took precedence over repr config's `Long`
        assert_eq!(
            "1.59M",
            format!("{:-#}", repr.with(Format::Long | Format::NoSpace))
        );
    }
}
