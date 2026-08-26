#[cfg(feature = "u128")]
pub type Int = u128;
#[cfg(not(feature = "u128"))]
pub type Int = u64;

#[cfg(not(feature = "lossless"))]
pub type Float = f64;
#[cfg(feature = "lossless")]
pub type Float = fraction::GenericFraction<Int>;

macro_rules! bitflags_const_or {
    ($flag:ident::{$($variant:ident)|+}) => {
        bitflags_const_or!($flag::{$($flag::$variant),+})
    };
    ($flag:ident::{$($variant:expr),+}) => {
        $flag::from_bits_truncate($($variant.bits())|+)
    }
}

mod bytesize;
mod numeric;
mod prefix;
mod unit;

pub mod prelude {
    pub use super::ReprConfigVariant::*;
    pub use super::sizes::all::*;
    pub use super::{ByteSize, Format, Mode, ReprFormat};
}

pub use bytesize::{ByteSize, ByteSizeRepr, Format, Mode, ReprConfigVariant, ReprFormat};
pub use prefix::UnitPrefix;
pub use unit::{SizeVariant, Unit, sizes};

/// The error raised when a string cannot be parsed into a [`ByteSize`],
/// [`Unit`], [`UnitPrefix`], or [`SizeVariant`].
///
/// The variant set is stable across the feature matrix: the two case-format
/// variants are only produced under the default (case-sensitive) parser, but
/// they exist unconditionally so a downstream `match` compiles against every
/// feature set.
///
/// ```
/// use xbytes::{ByteSize, ParseError};
///
/// assert_eq!("".parse::<ByteSize>().unwrap_err(), ParseError::EmptyInput);
/// assert_eq!("10".parse::<ByteSize>().unwrap_err(), ParseError::MissingUnit);
/// ```
#[derive(thiserror::Error, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// the input was empty
    #[error("empty input")]
    EmptyInput,
    /// a value was given with no trailing unit
    #[error("missing unit")]
    MissingUnit,
    /// the value component was not a parseable number
    #[error("invalid value")]
    InvalidValue,
    /// a unit was given with no leading value
    #[error("missing value")]
    MissingValue,
    /// the prefix component did not name a known prefix
    #[error("invalid prefix")]
    InvalidPrefix,
    /// converting between bits and bytes overflowed the backing integer
    #[error("value overflow")]
    ValueOverflow,
    /// the size variant was neither a bit nor a byte
    #[error("invalid size variant")]
    InvalidSizeVariant,
    /// the thousands separators were not aligned on three-digit groups
    #[error("invalid thousands format")]
    InvalidThousandsFormat,
    /// the unit was spelled with a case the sensitive parser rejects
    #[error("invalid case: expected format like 'kB', 'Kb', 'KiB', 'Mb', 'MiB'")]
    InvalidUnitCaseFormat,
    /// the prefix was spelled with a case the sensitive parser rejects
    #[error("invalid case: expected format like 'k', 'K', 'Ki', 'M', 'Mi'")]
    InvalidPrefixCaseFormat,
}
