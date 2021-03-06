mod unit;
pub use unit::Unit;

use Unit::*;

pub struct ByteSize<T>(T);

impl<T> ByteSize<T> {
    pub fn new() -> Self {
        todo!()
    }
}
