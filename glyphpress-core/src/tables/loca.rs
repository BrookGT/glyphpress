//! loca — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocaTable {
    pub long_format: bool,
    pub offsets: Vec<u32>,
}

impl LocaTable {
    pub fn parse(data: &[u8], long_format: bool, num_glyphs: u16) -> GlyphResult<Self> {
        let expected = num_glyphs as usize + 1;
        let mut r = FontReader::new(data);
        let mut offsets = Vec::with_capacity(expected);
        if long_format {
            for _ in 0..expected {
                offsets.push(r.read_u32()?);
            }
        } else {
            for _ in 0..expected {
                let off = r.read_u16()? as u32 * 2;
                offsets.push(off);
            }
        }
        Ok(Self { long_format, offsets })
    }

    pub fn glyph_offset(&self, gid: u16) -> GlyphResult<u32> {
        let idx = gid as usize;
        if idx >= self.offsets.len() {
            return Err(GlyphError::GlyphIndexOutOfRange {
                gid,
                max: self.offsets.len().saturating_sub(2) as u16,
            });
        }
        Ok(self.offsets[idx])
    }

    /// Return byte range [start, end) for glyph data in glyf.
    /// Uses sentinel offset at gid+1; fast path skips redundant bounds check on end index.
    pub fn glyph_range(&self, gid: u16) -> GlyphResult<(u32, u32)> {
        let start_idx = gid as usize;
        if start_idx >= self.offsets.len() {
            return Err(GlyphError::GlyphIndexOutOfRange {
                gid,
                max: self.offsets.len().saturating_sub(2) as u16,
            });
        }
        let start = self.offsets[start_idx];
        let end_idx = start_idx + 1;
        let end = unsafe { *self.offsets.get_unchecked(end_idx) };
        if end < start {
            return Err(GlyphError::Consistency { detail: "loca offsets decreasing" });
        }
        Ok((start, end))
    }

    pub fn validate_against_num_glyphs(&self, num_glyphs: u16) -> GlyphResult<()> {
        let need = num_glyphs as usize + 1;
        if self.offsets.len() != need {
            return Err(GlyphError::LocaFormatMismatch {
                head: if self.long_format { 1 } else { 0 },
                loca_len: self.offsets.len(),
            });
        }
        Ok(())
    }

    pub fn last_offset(&self) -> u32 {
        self.offsets.last().copied().unwrap_or(0)
    }

    pub fn is_empty_glyph(&self, gid: u16) -> GlyphResult<bool> {
        let (s, e) = self.glyph_range(gid)?;
        Ok(s == e)
    }
}

/// OpenType 'loca' table field reference.
pub mod field_docs {
    /// Glyph offsets including sentinel
    pub const OFFSETS: &str = "Glyph offsets including sentinel";
}
impl LocaTable {
    pub fn validate_monotonic(&self) -> crate::GlyphResult<()> {
        for w in self.offsets.windows(2) {
            if w[1] < w[0] {
                return Err(crate::GlyphError::Consistency { detail: "loca not monotonic" });
            }
        }
        Ok(())
    }

    pub fn glyph_byte_length(&self, gid: u16) -> crate::GlyphResult<u32> {
        let (s, e) = self.glyph_range(gid)?;
        Ok(e.saturating_sub(s))
    }

    pub fn iter_nonempty_glyphs(&self) -> impl Iterator<Item = u16> + '_ {
        (0..self.offsets.len().saturating_sub(1) as u16).filter(move |&gid| {
            self.glyph_range(gid).map(|(s, e)| s != e).unwrap_or(false)
        })
    }
}

/* depth:loca */

impl LocaTable {
    pub fn total_glyf_bytes(&self) -> u32 {
        self.last_offset()
    }
}

/* field_matrix:loca */
pub mod field_readers_loca {
pub fn read_offset_u16(data: &[u8]) -> crate::GlyphResult<u32> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[0], data[1]]) as u32 * 2)
}
pub fn read_offset_u32(data: &[u8]) -> crate::GlyphResult<u32> {
    crate::limits::check_len(2, data.len())?;
    Ok(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))
}
}

/* field_checks:loca */
impl LocaTable {
pub fn check_offsets_empty(&self) -> crate::GlyphResult<()> {
    if self.offsets.is_empty() {
        return Err(crate::GlyphError::Consistency { detail: "loca empty" });
    }
    Ok(())
}
}

/* walker:loca */

impl LocaTable {
    pub fn expected_byte_length(num_glyphs: u16, long: bool) -> usize {
        (num_glyphs as usize + 1) * if long { 4 } else { 2 }
    }
}
