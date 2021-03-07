mod prefix;
pub mod prelude;
mod unit;
pub use prelude::*;

pub struct ByteSize<T>(T);

#[cfg(has_u128)]
type int = u128;
#[cfg(not(has_u128))]
type int = u64;

impl<T> ByteSize<T> {
    pub fn new() -> Self {
        todo!()
    }
}
