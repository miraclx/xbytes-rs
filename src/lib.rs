use std::fmt;

#[cfg(feature = "u128")]
pub type Int = u128;
#[cfg(not(feature = "u128"))]
pub type Int = u64;

#[cfg(not(feature = "lossless"))]
pub type Float = f64;
#[cfg(feature = "lossless")]
pub type Float = fraction::GenericFraction<Int>;

/// converts a value to a potentially lossless float
macro_rules! f {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let val = Float::from($value);
        #[cfg(not(feature = "lossless"))]
        let val = $value as Float;
        val
    }};
}

/// converts a value to the appropriate unsigned integer
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

/// checks if the potentially lossless float value is equal to zero
macro_rules! f_is_zero {
    ($value:expr) => {{
        #[cfg(feature = "lossless")]
        let res = fraction::Zero::is_zero(&$value);
        #[cfg(not(feature = "lossless"))]
        let res = $value == 0.0;
        res
    }};
}

/// checks if the potentially lossless float value is equal to one
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

/// Allows the provision of alternate execution paths for the same logic.
///
/// ### `bits` / `nobits`
///
/// The `bits` and `nobits` variants are used to provide alternate execution paths depending on whether
/// the "bits" feature flag is set.
///
/// ### `safely` / `unsafe`
///
/// Provides an alternative "safe" implementation for an otherwise potentially panickable execution.
///
/// Set the `no-panic` feature flag to use the "safely" variant.
///
/// For example, arithmetic overflows that could simply just saturate the result to the maximum value
///
/// This takes in two branches, that should always return the same result except in the case of where
/// one panics and the other implements a fallback.
///
///  - Without "no-panic", the "unsafe" branch is executed, meaning there's a possibility of panicing
///    naturally we only expect to panic from arithmetic over/underflows
///  - With "no-panic", the "safely" branch is executed, meaning the contained code must not panic on
///    the same conditions but implements a fallback, like saturating or returning `Default` if applicable.
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

mod bytesize;
mod prefix;
mod unit;

pub mod prelude {
    pub use super::sizes::all::*;
    pub use super::{ByteSize, Format, Mode, ReprConfigVariant::*, ReprFormat};
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
