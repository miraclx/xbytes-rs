#[macro_use]
mod macros;
mod bytesize;
mod error;
mod prefix;
mod unit;

#[cfg(feature = "u128")]
pub type Int = u128;
#[cfg(not(feature = "u128"))]
pub type Int = u64;

#[cfg(not(feature = "lossless"))]
pub type Float = f64;
#[cfg(feature = "lossless")]
pub type Float = fraction::GenericFraction<Int>;

pub mod prelude {
    pub use super::sizes::all::*;
    pub use super::{ByteSize, Format, Mode, ReprConfigVariant::*, ReprFormat};
}

pub use bytesize::{ByteSize, ByteSizeRepr, Format, Mode, ReprConfigVariant, ReprFormat};
pub use error::{ParseError, ParseErrorKind};
pub use prefix::UnitPrefix;
pub use unit::{sizes, SizeVariant, Unit};
