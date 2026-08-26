use std::fmt;

#[cfg(feature = "u128")]
pub type Int = u128;
#[cfg(not(feature = "u128"))]
pub type Int = u64;

#[cfg(not(feature = "lossless"))]
pub type Float = f64;
#[cfg(feature = "lossless")]
pub type Float = fraction::GenericFraction<Int>;

macro_rules! f {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let val = Float::from($value);
        #[cfg(not(feature = "lossless"))]
        let val = $value as Float;
        val
    }};
}

macro_rules! i {
    ($value:expr) => {{
        // Collapse a Float back to a whole byte count. Non-finite or negative
        // states (overflow to infinity, NaN, an underflowed subtraction) are
        // saturated rather than panicked on, per the no-panic-on-values rule:
        // positive infinity to the maximum, everything else to zero.
        #[cfg(feature = "lossless")]
        let val = match $value {
            fraction::GenericFraction::Rational(fraction::Sign::Plus, r) => r.numer() / r.denom(),
            fraction::GenericFraction::Infinity(fraction::Sign::Plus) => Int::MAX,
            _ => 0,
        };
        #[cfg(not(feature = "lossless"))]
        let val = $value as Int;
        val
    }};
}

macro_rules! f_is_zero {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let res = fraction::Zero::is_zero(&$value);
        #[cfg(not(feature = "lossless"))]
        let res = $value == 0.0;
        res
    }};
}

macro_rules! f_is_one {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let res = fraction::One::is_one(&$value);
        #[cfg(not(feature = "lossless"))]
        let res = $value == 1.0;
        res
    }};
}

#[inline]
#[cfg(feature = "no-panic")]
fn get_max_saturate<T: fraction::Bounded>(_value: Option<T>) -> T {
    T::max_value()
}

#[cfg(feature = "no-panic")]
macro_rules! saturate {
    ($value:expr) => {
        match $value {
            Some(value) => value,
            None => $crate::get_max_saturate(None),
        }
    };
}

macro_rules! exec {
    (@ safely $expr:block) => {
        #[cfg(all(feature = "no-panic", feature = "lossless"))] {
            #[allow(unused_imports)] use fraction::{CheckedDiv, CheckedMul};
            break $expr
        }
    };
    (@ unsafe $expr:block) => {
        #[cfg(any(not(feature = "no-panic"), not(feature = "lossless")))]
        break $expr
    };
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

macro_rules! bitflags_const_or {
    ($flag:ident::{$($variant:ident)|+}) => {
        bitflags_const_or!($flag::{$($flag::$variant),+})
    };
    ($flag:ident::{$($variant:expr),+}) => {
        $flag::from_bits_truncate($($variant.bits())|+)
    }
}

mod bytesize;
mod prefix;
mod unit;

pub mod prelude {
    pub use super::sizes::all::*;
    pub use super::ReprConfigVariant::*;
    pub use super::{ByteSize, Format, Mode, ReprFormat};
}

pub use bytesize::{ByteSize, ByteSizeRepr, Format, Mode, ReprConfigVariant, ReprFormat};
pub use prefix::UnitPrefix;
pub use unit::{sizes, SizeVariant, Unit};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    MissingUnit,
    InvalidValue,
    MissingValue,
    InvalidPrefix,
    ValueOverflow,
    InvalidSizeVariant,
    InvalidThousandsFormat,
    #[cfg(not(feature = "case-insensitive"))]
    InvalidUnitCaseFormat,
    #[cfg(not(feature = "case-insensitive"))]
    InvalidPrefixCaseFormat,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            ParseError::EmptyInput => "empty input",
            ParseError::MissingUnit => "missing unit",
            ParseError::InvalidValue => "invalid value",
            ParseError::MissingValue => "missing value",
            ParseError::InvalidPrefix => "invalid prefix",
            ParseError::InvalidSizeVariant => "invalid size variant",
            ParseError::InvalidThousandsFormat => "invalid thousands format",
            ParseError::ValueOverflow => "value overflow",
            #[cfg(not(feature = "case-insensitive"))]
            ParseError::InvalidUnitCaseFormat => {
                "invalid case: expected format like 'kB', 'Kb', 'KiB', 'Mb', 'MiB'"
            }
            #[cfg(not(feature = "case-insensitive"))]
            ParseError::InvalidPrefixCaseFormat => {
                "invalid case: expected format like 'k', 'K', 'Ki', 'M', 'Mi'"
            }
        })
    }
}

impl std::error::Error for ParseError {}
