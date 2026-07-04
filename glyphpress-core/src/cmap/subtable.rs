//! cmap subtable wrapper and format dispatch.


use crate::cmap::decode::{map_codepoint_format4, map_codepoint_format12, parse_format4_segments};
use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;

#[derive(Clone, Debug)]
pub struct CmapSubtable {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub offset: usize,
    pub format: u16,
    pub data: Vec<u8>,
}

impl CmapSubtable {
    pub fn parse_at(data: &[u8], offset: usize, platform_id: u16, encoding_id: u16) -> GlyphResult<Self> {
        limits::check_len(offset + 2, data.len())?;
        let mut r = FontReader::from_slice(data, offset)?;
        let format = r.read_u16()?;
        let sub_len = data.len().saturating_sub(offset);
        let slice = &data[offset..offset + sub_len];
        Ok(Self {
            platform_id,
            encoding_id,
            offset,
            format,
            data: slice.to_vec(),
        })
    }

    pub fn is_unicode(&self) -> bool {
        (self.platform_id == 0)
            || (self.platform_id == 3 && (self.encoding_id == 1 || self.encoding_id == 10))
    }

    pub fn is_symbol(&self) -> bool {
        self.platform_id == 3 && self.encoding_id == 0
    }

    pub fn map_codepoint(&self, ch: u32) -> GlyphResult<Option<u16>> {
        match self.format {
            4 => map_codepoint_format4(&self.data, ch),
            12 => map_codepoint_format12(&self.data, ch),
            0 => {
                if ch < 256 {
                    Ok(Some(ch as u16))
                } else {
                    Ok(None)
                }
            }
            _ => Err(GlyphError::Unsupported { detail: "cmap format" }),
        }
    }

    pub fn collect_glyph_codepoints(&self, gid: u16, out: &mut Vec<u32>) {
        if self.format == 4 {
            if let Ok(segs) = parse_format4_segments(&self.data) {
                for seg in segs {
                    for cp in seg.start..=seg.end {
                        if seg.glyph_id.wrapping_add((cp - seg.start) as u16) == gid {
                            out.push(cp);
                        }
                    }
                }
            }
        }
    }

    pub fn validate_bounds(&self, data_len: usize) -> GlyphResult<()> {
        if self.offset >= data_len {
            return Err(GlyphError::truncated(self.offset + 1, data_len));
        }
        Ok(())
    }
}
