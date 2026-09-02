//! Convert between a raw byte count and a human-readable size, in both directions.
//!
//! xbytes turns a byte count into a size string (`1536 KiB` renders as `1.50 MiB`) and parses one back
//! (`"1.5 KiB"` reads as `1536`), with fine control over how the size is written:
//!
//! - **Exact-fraction arithmetic.** Conversions run through exact rationals
//!   rather than `f64` (with the default `lossless` feature), so repeated math
//!   never drifts and the rendered digits are exact.
//! - **A fully-typed [`Unit`].** Prefix (SI or IEC) and variant (bit or byte)
//!   are enum-backed values, not strings: `KiB` and `KB` are distinct,
//!   comparable, round-trippable units.
//! - **A broad formatting vocabulary:** thousands separators, arbitrary
//!   precision, long unit words, pluralization and capitalization control, bits
//!   or bytes, SI or IEC, all composable.
//!
//! # The fluent front door
//!
//! Build a [`ByteSize`] from a number and a [`Unit`], then reach for the fluent
//! formatter. [`iec`](ByteSize::iec) / [`si`](ByteSize::si) /
//! [`bits`](ByteSize::bits) choose the number system, and the refiners on the
//! returned [`ByteSizeRepr`] ([`long`](ByteSizeRepr::long),
//! [`precision`](ByteSizeRepr::precision),
//! [`separator`](ByteSizeRepr::separator), ...) chain in any order:
//!
//! ```
//! use xbytes::prelude::*;
//!
//! let size = ByteSize::of(1536, KIBI_BYTE);
//!
//! assert_eq!(size.to_string(), "1.50 MiB");         // Display default: IEC
//! assert_eq!(size.si().to_string(), "1.57 MB");     // decimal, auto-prefixed
//! assert_eq!(size.bits().to_string(), "12 Mib");    // denominated in bits
//! assert_eq!(size.iec().long().precision(3).to_string(), "1.500 MebiBytes");
//!
//! // and back again, exactly
//! let a: ByteSize = "1,024 MiB".parse().unwrap();
//! let b: ByteSize = "1 GiB".parse().unwrap();
//! assert_eq!(a, b);
//! assert_eq!(a.byte_count_lossy(), 1024 * 1024 * 1024); // pull the raw count back out
//! ```
//!
//! The [`Format`] bitflags and [`ReprConfigVariant`] knobs remain available via
//! [`with`](ByteSizeRepr::with) for anything the refiners do not name; the
//! refiners are the primary, documented path.
//!
//! # Fine-grained formatting
//!
//! ```
//! use xbytes::prelude::*;
//!
//! let size = ByteSize::of(58375, MEBI_BYTE);
//!
//! // pin a unit and group the thousands, with any separator
//! assert_eq!(size.iec().pin(MEBI_BYTE).thousands().to_string(), "58,375 MiB");
//! assert_eq!(size.iec().pin(MEBI_BYTE).separator("_").to_string(), "58_375 MiB");
//!
//! // arbitrary precision with no float noise, because the value is exact
//! assert_eq!(
//!     ByteSize::of(1536, KIBI_BYTE).iec().precision(20).to_string(),
//!     "1.50000000000000000000 MiB",
//! );
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
//!   [`bit_count`](ByteSize::bit_count) and [`byte_count`](ByteSize::byte_count)
//!   between an infallible `Self`/[`Int`] and a fallible [`Result`], since only
//!   one direction can overflow. Pick one shape per dependency graph.
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
