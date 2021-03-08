mod prefix;
pub mod prelude;
mod unit;
pub use prelude::*;

pub struct ByteSize<T>(T);

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    PrefixParseError,
    SizeVariantParseError,
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
