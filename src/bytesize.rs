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

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct ReprFormat {
    flags: Format,
    precision: usize,
    thousands_separator: &'static str,
}

impl ReprFormat {
    const fn default() -> Self {
        Self {
            flags: Format::Default,
            precision: 2,
            thousands_separator: ",",
        }
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
        fmt::Display::fmt(&self.repr(Mode::Default), f)
    }
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub struct ByteSizeRepr(Float, Unit, ReprFormat);

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

pub trait ReprConfig {
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat;
}

impl ReprConfig for Format {
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat {
        ReprFormat {
            flags: bitflags_const_or!(Format::{r_fmt.flags, self}),
            ..*r_fmt
        }
    }
}

pub enum ReprConfigVariant {
    Separator(&'static str),
    Precision(usize),
}

use ReprConfigVariant::*;

impl ReprConfig for ReprConfigVariant {
    fn apply(&self, r_fmt: &ReprFormat) -> ReprFormat {
        let mut new = *r_fmt;
        match *self {
            Precision(precision) => new.precision = precision,
            Separator(sep) => new.thousands_separator = sep,
        }
        new
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

// thousands separator
// thsep("503") -> (3, 0, ['503'])
// thsep("405503") -> (6, 1, ['405', '503'])
// thsep("1234567") -> (7, 2, ['1', '234', '567'])
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
        let thousands_separator = self.2.thousands_separator;
        let mut value = self.0;

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
            let (len, holes, mut parts) = thsep(whole);
            let mut whole = String::with_capacity(len + holes);
            whole.extend(parts.next().into_iter().chain(
                parts.flat_map(|s| std::iter::once(thousands_separator).chain(std::iter::once(s))),
            ));
            value_part = format!("{}{}", whole, fract);
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
        assert_ne!(l.with(Format::Long), r); // 104.5 TebiBytes != 104.50 TiB
        assert_eq!(l.with(Format::NoMultiCaps), r.with(Format::NoMultiCaps)); // 104.50 Tebibytes == 104.50 Tebibytes
        assert_eq!(
            l.with(Format::Initials | Format::NoFraction),
            r.with(Format::Condensed | Format::NoSpace)
        ); // 104 TB != 104.50T
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
}
