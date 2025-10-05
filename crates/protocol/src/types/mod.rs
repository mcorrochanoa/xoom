use bytes::{Buf, BufMut};
use std::io::Result;

pub mod primitives;
pub mod varint;
pub mod varlong;

pub trait DataType: Sized {
    fn read<B: Buf>(buf: &mut B) -> Result<Self>;
    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()>;
}
