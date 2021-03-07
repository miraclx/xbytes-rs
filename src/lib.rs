mod prefix;
pub use prefix::UnitPrefix;

use UnitPrefix::*;

pub struct ByteSize<T>(T);

impl<T> ByteSize<T> {
    pub fn new() -> Self {
        todo!()
    }
}
