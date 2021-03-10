use std::fmt;

mod bytesize;
mod prefix;
mod unit;

pub mod prelude {
    pub use super::bytesize::{ByteSize, Format, Mode};
    pub use super::prefix::UnitPrefix;
    pub use super::unit::{
        sizes::{self, binary::*, decimal::*},
        SizeVariant, Unit,
    };
    pub use super::ParseError;
}

pub use {
    bytesize::{ByteSize, Format, Mode},
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
