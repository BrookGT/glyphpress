//! Table record entry in SFNT directory.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableRecord {
    pub tag: u32,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

impl TableRecord {
    pub fn parse(reader: &mut FontReader<'_>) -> GlyphResult<Self> {
        Ok(Self {
            tag: reader.read_tag()?,
            checksum: reader.read_u32()?,
            offset: reader.read_u32()?,
            length: reader.read_u32()?,
        })
    }

    pub fn tag_str(&self) -> [u8; 4] {
        self.tag.to_be_bytes()
    }

    pub fn end_offset(&self) -> u64 {
        self.offset as u64 + self.length as u64
    }

    pub fn validate_bounds(&self, file_len: usize) -> GlyphResult<()> {
        let end = self.end_offset();
        if end > file_len as u64 {
            return Err(GlyphError::truncated(end as usize, file_len));
        }
        Ok(())
    }
}
