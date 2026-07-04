//! gdef — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug)]
pub struct GdefTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub glyph_class_def_offset: u16,
    pub attach_list_offset: u16,
    pub lig_caret_list_offset: u16,
    pub mark_attach_class_def_offset: u16,
}

impl GdefTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let major_version = r.read_u16()?;
        let minor_version = r.read_u16()?;
        let glyph_class_def_offset = r.read_u16()?;
        let attach_list_offset = r.read_u16()?;
        let lig_caret_list_offset = r.read_u16()?;
        let mut mark_attach_class_def_offset = 0u16;
        if major_version >= 1 && minor_version >= 2 && r.remaining() >= 2 {
            mark_attach_class_def_offset = r.read_u16()?;
        }
        Ok(Self {
            major_version, minor_version, glyph_class_def_offset,
            attach_list_offset, lig_caret_list_offset, mark_attach_class_def_offset,
        })
    }

    pub fn has_glyph_classes(&self) -> bool {
        self.glyph_class_def_offset != 0
    }
}

/// OpenType 'gdef' table field reference.
pub mod field_docs {
    /// Offset to ClassDef table
    pub const GLYPH_CLASS_DEF_OFFSET: &str = "Offset to ClassDef table";
}
impl GdefTable {
    pub fn version_tuple(&self) -> (u16, u16) {
        (self.major_version, self.minor_version)
    }

    pub fn parse_glyph_class_def<'a>(&self, data: &'a [u8]) -> crate::GlyphResult<Option<GlyphClassDef<'a>>> {
        if self.glyph_class_def_offset == 0 {
            return Ok(None);
        }
        GlyphClassDef::parse(data, self.glyph_class_def_offset as usize)
    }
}

pub struct GlyphClassDef<'a> {
    pub class_format: u16,
    pub data: &'a [u8],
}

impl<'a> GlyphClassDef<'a> {
    pub fn parse(data: &'a [u8], offset: usize) -> crate::GlyphResult<Option<Self>> {
        if offset + 2 > data.len() { return Ok(None); }
        let mut r = crate::io::FontReader::from_slice(data, offset)?;
        let class_format = r.read_u16()?;
        Ok(Some(Self { class_format, data: &data[offset..] }))
    }

    pub fn class_for_glyph(&self, _gid: u16) -> u16 {
        match self.class_format {
            1 => 0,
            2 => 0,
            _ => 0,
        }
    }
}

/* depth:gdef */

impl GdefTable {
    pub fn has_attachment_list(&self) -> bool {
        self.attach_list_offset != 0
    }
}

/* field_matrix:gdef */
pub mod field_readers_gdef {
pub fn read_major(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[0], data[1]]))
}
pub fn read_minor(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[2], data[3]]))
}
pub fn read_class_def(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[4], data[5]]))
}
}

/* field_checks:gdef */
impl GdefTable {
pub fn check_version(&self) -> crate::GlyphResult<()> {
    if self.major_version == 0 {
        return Err(crate::GlyphError::Consistency { detail: "gdef version zero" });
    }
    Ok(())
}
}

/* walker:gdef */

impl GdefTable {
    pub fn layout_offsets(&self) -> (u16, u16, u16) {
        (self.glyph_class_def_offset, self.attach_list_offset, self.lig_caret_list_offset)
    }
}
