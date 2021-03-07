mod prefix;
pub mod prelude;
mod unit;
pub use prelude::*;

pub struct ByteSize<T>(T);

impl<T> ByteSize<T> {
    pub fn new() -> Self {
        todo!()
    }
}
