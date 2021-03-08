mod prefix;
pub mod prelude;
mod unit;
pub use prelude::*;
use std::fmt;

pub struct ByteSize<T>(T);

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    InvalidPrefix,
    InvalidSizeVariant,
    InvalidUnitCaseFormat,
    InvalidPrefixCaseFormat,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            ParseError::EmptyInput => "empty input",
            ParseError::InvalidPrefix => "invalid prefix",
            ParseError::InvalidSizeVariant => "invalid size variant",
            ParseError::InvalidUnitCaseFormat => {
                "invalid case: expected format like 'kB', 'Kb', 'KiB', 'Mb', 'MiB'"
            }
            ParseError::InvalidPrefixCaseFormat => {
                "invalid case: expected format like 'k', 'K', 'Ki', 'M', 'Mi'"
            }
        })
    }
}

#[cfg(has_u128)]
type Int = u128;
#[cfg(not(has_u128))]
type Int = u64;

impl<T> ByteSize<T> {
    pub fn new(t: T) -> Self {
        ByteSize(t)
    }
}
