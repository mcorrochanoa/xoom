use crate::types::DataType;
use bytes::{Buf, BufMut};
use std::io::{Error, ErrorKind, Result};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl VarInt {
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> i32 {
        self.0
    }

    pub fn encoded_size(&self) -> usize {
        let mut value = self.0 as u32;
        let mut size = 0;

        loop {
            size += 1;
            value >>= 7;
            if value == 0 {
                break;
            }
        }

        size
    }
}

impl DataType for VarInt {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        let mut result = 0i32;
        let mut shift = 0;
        let mut bytes_read = 0;

        loop {
            if !buf.has_remaining() {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "Not enough data for VarInt",
                ));
            }

            let byte = buf.get_u8();
            bytes_read += 1;

            result |= ((byte & 0x7F) as i32) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;

            if bytes_read > 5 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "VarInt is too long (max 5 bytes)",
                ));
            }

            if shift >= 32 {
                return Err(Error::new(ErrorKind::InvalidData, "VarInt is too big"));
            }
        }

        Ok(VarInt(result))
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        let mut value = self.0 as u32;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0x80;
            }

            buf.put_u8(byte);

            if value == 0 {
                break;
            }
        }

        Ok(())
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl From<VarInt> for i32 {
    fn from(varint: VarInt) -> Self {
        varint.0
    }
}

impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        VarInt(value as i32)
    }
}
