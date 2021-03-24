use std::fmt;

macro_rules! bitflags_const_or {
    ($flag:ident::{$($variant:ident)|+}) => {
        bitflags_const_or!($flag::{$($flag::$variant),+})
    };
    ($flag:ident::{$($variant:expr),+}) => {
        $flag::from_bits_truncate($($variant.bits())|+)
    }
}

#[cfg(not(feature = "lossless"))]
pub type Float = f64;
#[cfg(all(feature = "lossless", not(feature = "u128")))]
pub type Float = fraction::Fraction;
#[cfg(all(feature = "lossless", feature = "u128"))]
pub type Float = fraction::GenericFraction<u128>;

macro_rules! f {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let val = Float::from($value);
        #[cfg(not(feature = "lossless"))]
        let val = $value as Float;
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

#[cfg(feature = "u128")]
pub type Int = u128;
#[cfg(not(feature = "u128"))]
pub type Int = u64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
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
