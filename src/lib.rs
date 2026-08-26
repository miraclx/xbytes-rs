//! Convert between raw byte counts and human-readable sizes, both ways.
//!
//! Build a [`ByteSize`] from a number and a [`Unit`], then render it: either
//! let [`repr`](ByteSize::repr) pick the largest fitting prefix, or pin an
//! explicit unit with [`repr_as`](ByteSize::repr_as). Parsing runs the other
//! direction through [`FromStr`](core::str::FromStr).
//!
//! ```
//! use xbytes::prelude::*;
//!
//! // number + unit -> size, auto-prefixed on the way out
//! let size = ByteSize::of(1536, KIBI_BYTE);
//! assert_eq!(size.to_string(), "1.50 MiB");
//! assert_eq!(size.repr(Mode::Decimal).to_string(), "1.57 MB");
//!
//! // and back again
//! let parsed: ByteSize = "1.5 MiB".parse().unwrap();
//! assert_eq!(parsed, size);
//! ```
//!
//! # Feature flags
//!
//! - `u128` (default): back the count with `u128` instead of `u64`, unlocking
//!   the zetta/yotta prefixes.
//! - `lossless` (default): compute in exact fractions rather than `f64`, so
//!   repeated conversions do not drift.
//! - `no-panic` (default): saturate arithmetic on overflow instead of relying
//!   on `f64`'s infinities (implies `lossless`).
//! - `bits`: store the count in bits rather than bytes. This flips the return
//!   types of [`ByteSize::from_bits`], [`from_bytes`](ByteSize::from_bytes),
//!   [`bits`](ByteSize::bits) and [`bytes`](ByteSize::bytes) between an
//!   infallible `Self`/[`Int`] and a fallible [`Result`], since only one
//!   direction can overflow. Pick one shape per dependency graph.
//! - `case-insensitive`: accept units in any case (`kib`, `MB`, `gIb`) rather
//!   than the stricter default parser.

#![warn(missing_docs)]

/// The integer backing a [`ByteSize`]: `u128` with the `u128` feature (the
/// default), `u64` otherwise.
#[cfg(feature = "u128")]
pub type Int = u128;
/// The integer backing a [`ByteSize`]: `u128` with the `u128` feature (the
/// default), `u64` otherwise.
#[cfg(not(feature = "u128"))]
pub type Int = u64;

/// The scalar used for intermediate arithmetic: an exact fraction with the
/// `lossless` feature (the default), plain `f64` otherwise.
#[cfg(not(feature = "lossless"))]
pub type Float = f64;
/// The scalar used for intermediate arithmetic: an exact fraction with the
/// `lossless` feature (the default), plain `f64` otherwise.
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

/// The everyday surface: [`ByteSize`], the [`Format`]/[`Mode`] flags, the
/// [`ReprConfigVariant`] builders, and every named size constant. Glob-import
/// it to write `ByteSize::of(10, MEBI_BYTE)` with no further imports.
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
