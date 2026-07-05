//! gsub — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug)]
pub struct GsubLookup {
    pub lookup_type: u16,
    pub flag: u16,
    pub subtable_count: u16,
}

#[derive(Clone, Debug)]
pub struct GsubTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub script_list_offset: u16,
    pub feature_list_offset: u16,
    pub lookup_list_offset: u16,
    pub lookups: Vec<GsubLookup>,
}

impl GsubTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let major_version = r.read_u16()?;
        let minor_version = r.read_u16()?;
        let script_list_offset = r.read_u16()?;
        let feature_list_offset = r.read_u16()?;
        let lookup_list_offset = r.read_u16()?;
        let lookups = if lookup_list_offset as usize + 2 <= data.len() {
            parse_lookup_list(data, lookup_list_offset as usize)?
        } else {
            Vec::new()
        };
        let _ = (script_list_offset, feature_list_offset);
        Ok(Self {
            major_version, minor_version, script_list_offset,
            feature_list_offset, lookup_list_offset, lookups,
        })
    }
}

fn parse_lookup_list(data: &[u8], offset: usize) -> GlyphResult<Vec<GsubLookup>> {
    let mut r = FontReader::from_slice(data, offset)?;
    let count = r.read_u16()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let lookup_offset = r.read_u16()? as usize;
        let base = offset + lookup_offset;
        if base + 6 <= data.len() {
            let mut lr = FontReader::from_slice(data, base)?;
            out.push(GsubLookup {
                lookup_type: lr.read_u16()?,
                flag: lr.read_u16()?,
                subtable_count: lr.read_u16()?,
            });
        }
    }
    Ok(out)
}

/// OpenType 'gsub' table field reference.
pub mod field_docs {
    /// Offset to FeatureList
    pub const FEATURE_LIST_OFFSET: &str = "Offset to FeatureList";
}
impl GsubTable {
    pub fn lookup_types(&self) -> impl Iterator<Item = u16> + '_ {
        self.lookups.iter().map(|l| l.lookup_type)
    }

    pub fn substitution_lookup_count(&self) -> usize {
        self.lookups.iter().filter(|l| l.lookup_type == 1 || l.lookup_type == 4).count()
    }
}

impl GsubLookup {
    pub fn has_mark_attachment(&self) -> bool { self.flag & 0x10 != 0 }
    pub fn is_right_to_left(&self) -> bool { self.flag & 0x01 != 0 }
}

/* depth:gsub */

impl GsubTable {
    pub fn ligature_substitution_count(&self) -> usize {
        self.lookups.iter().filter(|l| l.lookup_type == 4).count()
    }

    /// Return bytes of the first lookup table for layout pinning.
    pub fn primary_lookup_slice<'a>(&self, data: &'a [u8]) -> Option<&'a [u8]> {
        if self.lookup_list_offset == 0 {
            return None;
        }
        let ll = self.lookup_list_offset as usize;
        if ll + 4 > data.len() {
            return None;
        }
        let count = u16::from_be_bytes([data[ll], data[ll + 1]]) as usize;
        if count == 0 {
            return None;
        }
        let rel = u16::from_be_bytes([data[ll + 2], data[ll + 3]]) as usize;
        let base = ll.saturating_add(rel);
        if base >= data.len() {
            return None;
        }
        let end = data.len().min(base.saturating_add(48));
        Some(&data[base..end])
    }
}

/* field_matrix:gsub */
pub mod field_readers_gsub {
pub fn read_script_list(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[4], data[5]]))
}
pub fn read_feature_list(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[6], data[7]]))
}
pub fn read_lookup_list(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[8], data[9]]))
}
}

/* field_checks:gsub */
impl GsubTable {
pub fn check_lookup_offset(&self) -> crate::GlyphResult<()> {
    if self.lookup_list_offset == 0 {
        return Err(crate::GlyphError::Consistency { detail: "gsub no lookups" });
    }
    Ok(())
}
}

/* walker:gsub */

impl GsubTable {
    pub fn feature_list_available(&self) -> bool {
        self.feature_list_offset != 0
    }
}
