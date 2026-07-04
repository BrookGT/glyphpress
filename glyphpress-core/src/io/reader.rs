//! Big-endian binary reader over font bytes.


use crate::error::{GlyphError, GlyphResult};
use crate::limits;

#[derive(Clone, Debug)]
pub struct FontReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> FontReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn from_slice(data: &'a [u8], offset: usize) -> GlyphResult<Self> {
        limits::check_len(offset, data.len())?;
        Ok(Self { data, pos: offset })
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn set_position(&mut self, pos: usize) -> GlyphResult<()> {
        limits::check_len(pos, self.data.len())?;
        self.pos = pos;
        Ok(())
    }

    pub fn skip(&mut self, n: usize) -> GlyphResult<()> {
        limits::check_len(self.pos.saturating_add(n), self.data.len())?;
        self.pos += n;
        Ok(())
    }

    pub fn peek_u8(&self) -> GlyphResult<u8> {
        limits::check_len(self.pos + 1, self.data.len())?;
        Ok(self.data[self.pos])
    }

    pub fn read_u8(&mut self) -> GlyphResult<u8> {
        let v = self.peek_u8()?;
        self.pos += 1;
        Ok(v)
    }

    pub fn read_i8(&mut self) -> GlyphResult<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u16(&mut self) -> GlyphResult<u16> {
        limits::check_len(self.pos + 2, self.data.len())?;
        let v = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(v)
    }

    pub fn read_i16(&mut self) -> GlyphResult<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub fn read_u32(&mut self) -> GlyphResult<u32> {
        limits::check_len(self.pos + 4, self.data.len())?;
        let b = &self.data[self.pos..self.pos + 4];
        self.pos += 4;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn read_i32(&mut self) -> GlyphResult<i32> {
        Ok(self.read_u32()? as i32)
    }

    pub fn read_u64(&mut self) -> GlyphResult<u64> {
        limits::check_len(self.pos + 8, self.data.len())?;
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.data[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_tag(&mut self) -> GlyphResult<u32> {
        limits::check_len(self.pos + 4, self.data.len())?;
        let mut tag = [0u8; 4];
        tag.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(u32::from_be_bytes(tag))
    }

    pub fn read_bytes(&mut self, len: usize) -> GlyphResult<&'a [u8]> {
        limits::check_len(self.pos + len, self.data.len())?;
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    pub fn read_fixed(&mut self) -> GlyphResult<i32> {
        self.read_i32()
    }

    pub fn read_fword(&mut self) -> GlyphResult<i16> {
        self.read_i16()
    }

    pub fn read_ufword(&mut self) -> GlyphResult<u16> {
        self.read_u16()
    }

    pub fn read_longdatetime(&mut self) -> GlyphResult<u64> {
        self.read_u64()
    }

    pub fn slice_from(&self, offset: usize, len: usize) -> GlyphResult<&'a [u8]> {
        limits::check_len(offset.saturating_add(len), self.data.len())?;
        Ok(&self.data[offset..offset + len])
    }

    pub fn entire_slice(&self) -> &'a [u8] {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_be_integers() {
        let data = [0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE];
        let mut r = FontReader::new(&data);
        assert_eq!(r.read_u16().unwrap(), 0x0001);
        assert_eq!(r.read_u16().unwrap(), 0x0203);
        assert_eq!(r.read_i16().unwrap(), -2);
    }
}

/* volume */

impl<'a> FontReader<'a> {
    pub fn peek_tag_at(&self, offset: usize) -> crate::GlyphResult<u32> {
        crate::limits::check_len(offset + 4, self.data.len())?;
        Ok(u32::from_be_bytes([
            self.data[offset], self.data[offset + 1], self.data[offset + 2], self.data[offset + 3]
        ]))
    }

    pub fn remaining_at(&self, offset: usize) -> usize {
        self.data.len().saturating_sub(offset)
    }
}
