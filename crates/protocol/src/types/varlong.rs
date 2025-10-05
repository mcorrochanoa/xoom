use crate::types::DataType;
use bytes::{Buf, BufMut};
use std::io::{Error, ErrorKind, Result};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl VarLong {
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> i64 {
        self.0
    }

    pub fn encoded_size(&self) -> usize {
        let mut value = self.0 as u64;
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

impl DataType for VarLong {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        let mut result = 0i64;
        let mut shift = 0;
        let mut bytes_read = 0;

        loop {
            if !buf.has_remaining() {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "Not enough data for VarLong",
                ));
            }

            let byte = buf.get_u8();
            bytes_read += 1;

            result |= ((byte & 0x7F) as i64) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;

            if bytes_read > 10 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "VarLong is too long (max 10 bytes)",
                ));
            }

            if shift >= 64 {
                return Err(Error::new(ErrorKind::InvalidData, "VarLong is too big"));
            }
        }

        Ok(VarLong(result))
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        let mut value = self.0 as u64;

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

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        VarLong(value)
    }
}

impl From<VarLong> for i64 {
    fn from(varlong: VarLong) -> Self {
        varlong.0
    }
}
