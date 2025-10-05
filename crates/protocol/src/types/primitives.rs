use crate::types::{DataType, varint::VarInt};
use bytes::{Buf, BufMut};
use std::io::{Error, ErrorKind, Result};

impl DataType for bool {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if !buf.has_remaining() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for bool",
            ));
        }
        Ok(buf.get_u8() != 0)
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_u8(if *self { 1 } else { 0 });
        Ok(())
    }
}

impl DataType for u8 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if !buf.has_remaining() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for u8",
            ));
        }
        Ok(buf.get_u8())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_u8(*self);
        Ok(())
    }
}

impl DataType for i8 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if !buf.has_remaining() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for i8",
            ));
        }
        Ok(buf.get_i8())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_i8(*self);
        Ok(())
    }
}

impl DataType for u16 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 2 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for u16",
            ));
        }
        Ok(buf.get_u16())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_u16(*self);
        Ok(())
    }
}

impl DataType for i16 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 2 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for i16",
            ));
        }
        Ok(buf.get_i16())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_i16(*self);
        Ok(())
    }
}

impl DataType for i32 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 4 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for i32",
            ));
        }
        Ok(buf.get_i32())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_i32(*self);
        Ok(())
    }
}

impl DataType for i64 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 8 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for i64",
            ));
        }
        Ok(buf.get_i64())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_i64(*self);
        Ok(())
    }
}

impl DataType for f32 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 4 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for f32",
            ));
        }
        Ok(buf.get_f32())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_f32(*self);
        Ok(())
    }
}

impl DataType for f64 {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 8 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for f64",
            ));
        }
        Ok(buf.get_f64())
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        buf.put_f64(*self);
        Ok(())
    }
}

impl DataType for String {
    fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        let lenght = VarInt::read(buf)?.0;

        if lenght < 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "String lenght is negative",
            ));
        }

        let lenght = lenght as usize;

        if buf.remaining() < lenght {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Not enough data for string",
            ));
        }

        let mut bytes = vec![0u8; lenght];
        buf.copy_to_slice(&mut bytes);

        String::from_utf8(bytes).map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8"))
    }

    fn write<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        let bytes = self.as_bytes();
        VarInt(bytes.len() as i32).write(buf)?;
        buf.put_slice(bytes);
        Ok(())
    }
}
