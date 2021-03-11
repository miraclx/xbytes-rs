use std::fmt;

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
    pub use super::bytesize::{ByteSize, ByteSizeOptions, Format, Mode};
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
type Int = u128;
#[cfg(not(feature = "u128"))]
type Int = u64;

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
