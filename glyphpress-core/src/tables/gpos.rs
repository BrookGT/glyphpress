//! gpos — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug)]
pub struct ScriptRecord {
    pub tag: u32,
    pub offset: u16,
}

#[derive(Clone, Debug)]
pub struct GposTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub script_list_offset: u16,
    pub feature_list_offset: u16,
    pub lookup_list_offset: u16,
    pub scripts: Vec<ScriptRecord>,
}

impl GposTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let major_version = r.read_u16()?;
        let minor_version = r.read_u16()?;
        let script_list_offset = r.read_u16()?;
        let feature_list_offset = r.read_u16()?;
        let lookup_list_offset = r.read_u16()?;
        let scripts = if script_list_offset as usize + 2 <= data.len() {
            parse_script_list(data, script_list_offset as usize)?
        } else {
            Vec::new()
        };
        Ok(Self {
            major_version, minor_version, script_list_offset,
            feature_list_offset, lookup_list_offset, scripts,
        })
    }
}

fn parse_script_list(data: &[u8], offset: usize) -> GlyphResult<Vec<ScriptRecord>> {
    let mut r = FontReader::from_slice(data, offset)?;
    let count = r.read_u16()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(ScriptRecord { tag: r.read_tag()?, offset: r.read_u16()? });
    }
    Ok(out)
}

/// OpenType 'gpos' table field reference.
pub mod field_docs {
    /// Offset to LookupList
    pub const LOOKUP_LIST_OFFSET: &str = "Offset to LookupList";
}
impl GposTable {
    pub fn script_tags(&self) -> impl Iterator<Item = u32> + '_ {
        self.scripts.iter().map(|s| s.tag)
    }

    pub fn lookup_count_hint(&self) -> usize {
        self.scripts.len()
    }
}

impl ScriptRecord {
    pub fn tag_str(&self) -> [u8; 4] { self.tag.to_be_bytes() }
}

/* depth:gpos */

impl GposTable {
    pub fn has_scripts(&self) -> bool {
        !self.scripts.is_empty()
    }
}

/* field_matrix:gpos */
pub mod field_readers_gpos {
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

/* field_checks:gpos */
impl GposTable {
pub fn check_lookup_offset(&self) -> crate::GlyphResult<()> {
    if self.lookup_list_offset == 0 {
        return Err(crate::GlyphError::Consistency { detail: "gpos no lookups" });
    }
    Ok(())
}
}

/* walker:gpos */

impl GposTable {
    pub fn feature_list_available(&self) -> bool {
        self.feature_list_offset != 0
    }
}
