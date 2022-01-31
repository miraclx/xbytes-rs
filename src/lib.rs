#[macro_use]
mod macros;
mod bytesize;
mod error;
mod flags;
mod prefix;
pub mod sizes;
mod unit;

pub use bytesize::{ByteSize, ByteSizeRepr, ReprConfig, ReprConfigVariant, ReprFormat};
pub use error::{ParseError, ParseErrorKind};
pub use flags::{Format, Mode};
pub use prefix::UnitPrefix;
pub use unit::{SizeVariant, Unit};

#[cfg(feature = "u128")]
pub type Int = u128;
#[cfg(not(feature = "u128"))]
pub type Int = u64;

#[cfg(not(feature = "lossless"))]
pub type Float = f64;
#[cfg(feature = "lossless")]
pub type Float = fraction::GenericFraction<Int>;
