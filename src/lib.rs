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
        #[cfg(feature = "lossless")]
        let val = if let fraction::GenericFraction::Rational(fraction::Sign::Plus, r) = $value {
            r.numer() / r.denom()
        } else {
            panic!("conversion to Int failed: expected unsigned rational float")
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

#[inline]
#[cfg(feature = "lossless")]
fn get_max_saturate<T: fraction::Bounded>(_value: Option<T>) -> T {
    T::max_value()
}

#[cfg(feature = "lossless")]
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
    pub use super::bytesize::{ByteSize, ByteSizeFormatter, ByteSizeOptions, Format, Mode};
    pub use super::prefix::UnitPrefix;
    pub use super::unit::{
        sizes::{self, binary::*, decimal::*},
        SizeVariant, Unit,
    };
    pub use super::ParseError;
}

pub use {
    bytesize::{ByteSize, ByteSizeOptions, Format, Mode},
    prefix::UnitPrefix,
    unit::{sizes, SizeVariant, Unit},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    MissingUnit,
    InvalidValue,
    MissingValue,
    InvalidPrefix,
    ValueOverflow,
    InvalidSizeVariant,
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
