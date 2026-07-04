//! Big-endian binary writer for SFNT emission.


use crate::error::{GlyphError, GlyphResult};
use crate::limits;

#[derive(Clone, Debug, Default)]
pub struct FontWriter {
    buf: Vec<u8>,
}

impl FontWriter {
    pub fn with_capacity(cap: usize) -> Self {
        Self { buf: Vec::with_capacity(cap) }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    fn ensure_room(&self, extra: usize) -> GlyphResult<()> {
        if self.buf.len().saturating_add(extra) > limits::MAX_TABLE_EMIT_BYTES {
            return Err(GlyphError::EmitOverflow { table: "writer" });
        }
        Ok(())
    }

    pub fn write_u8(&mut self, v: u8) -> GlyphResult<()> {
        self.ensure_room(1)?;
        self.buf.push(v);
        Ok(())
    }

    pub fn write_i8(&mut self, v: i8) -> GlyphResult<()> {
        self.write_u8(v as u8)
    }

    pub fn write_u16(&mut self, v: u16) -> GlyphResult<()> {
        self.ensure_room(2)?;
        self.buf.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    pub fn write_i16(&mut self, v: i16) -> GlyphResult<()> {
        self.write_u16(v as u16)
    }

    pub fn write_u32(&mut self, v: u32) -> GlyphResult<()> {
        self.ensure_room(4)?;
        self.buf.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    pub fn write_i32(&mut self, v: i32) -> GlyphResult<()> {
        self.write_u32(v as u32)
    }

    pub fn write_u64(&mut self, v: u64) -> GlyphResult<()> {
        self.ensure_room(8)?;
        self.buf.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    pub fn write_tag(&mut self, tag: u32) -> GlyphResult<()> {
        self.write_u32(tag)
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> GlyphResult<()> {
        self.ensure_room(data.len())?;
        self.buf.extend_from_slice(data);
        Ok(())
    }

    pub fn write_zeros(&mut self, count: usize) -> GlyphResult<()> {
        self.ensure_room(count)?;
        self.buf.extend(std::iter::repeat_n(0u8, count));
        Ok(())
    }

    pub fn pad_to_4(&mut self) -> GlyphResult<()> {
        let rem = self.buf.len() % 4;
        if rem != 0 {
            self.write_zeros(4 - rem)?;
        }
        Ok(())
    }

    pub fn patch_u16(&mut self, offset: usize, value: u16) -> GlyphResult<()> {
        crate::limits::check_len(offset + 2, self.buf.len())?;
        self.buf[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
        Ok(())
    }

    pub fn patch_u32(&mut self, offset: usize, value: u32) -> GlyphResult<()> {
        crate::limits::check_len(offset + 4, self.buf.len())?;
        self.buf[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
        Ok(())
    }
}
