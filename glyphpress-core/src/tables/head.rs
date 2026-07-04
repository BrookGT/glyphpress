//! head — OpenType table parser.

//! head — font header table.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;

pub const HEAD_MAGIC: u32 = 0x5F0F_3CF5;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub font_revision: i32,
    pub checksum_adjustment: u32,
    pub magic_number: u32,
    pub flags: u16,
    pub units_per_em: u16,
    pub created: u64,
    pub modified: u64,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}

impl HeadTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let major_version = r.read_u16()?;
        let minor_version = r.read_u16()?;
        let font_revision = r.read_i32()?;
        let checksum_adjustment = r.read_u32()?;
        let magic_number = r.read_u32()?;
        if magic_number != HEAD_MAGIC {
            return Err(GlyphError::BadMagic { found: magic_number });
        }
        let flags = r.read_u16()?;
        let units_per_em = r.read_u16()?;
        if units_per_em == 0 {
            return Err(GlyphError::OutOfRange {
                field: "unitsPerEm",
                value: 0,
            });
        }
        let created = r.read_longdatetime()?;
        let modified = r.read_longdatetime()?;
        let x_min = r.read_fword()?;
        let y_min = r.read_fword()?;
        let x_max = r.read_fword()?;
        let y_max = r.read_fword()?;
        let mac_style = r.read_u16()?;
        let lowest_rec_ppem = r.read_u16()?;
        let font_direction_hint = r.read_i16()?;
        let index_to_loc_format = r.read_i16()?;
        let glyph_data_format = r.read_i16()?;
        Ok(Self {
            major_version,
            minor_version,
            font_revision,
            checksum_adjustment,
            magic_number,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format,
        })
    }

    pub fn uses_long_loca(&self) -> bool {
        self.index_to_loc_format == 1
    }

    pub fn validate_bbox(&self) -> GlyphResult<()> {
        if self.x_min > self.x_max || self.y_min > self.y_max {
            return Err(GlyphError::Consistency {
                detail: "head bbox inverted",
            });
        }
        Ok(())
    }

    pub fn validate_loca_format(&self) -> GlyphResult<()> {
        if self.index_to_loc_format != 0 && self.index_to_loc_format != 1 {
            return Err(GlyphError::Unsupported {
                detail: "indexToLocFormat must be 0 or 1",
            });
        }
        Ok(())
    }

    pub fn validate_glyph_data_format(&self) -> GlyphResult<()> {
        if self.glyph_data_format != 0 {
            return Err(GlyphError::Unsupported {
                detail: "only glyf outlines supported",
            });
        }
        Ok(())
    }

    pub fn full_validate(&self) -> GlyphResult<()> {
        self.validate_bbox()?;
        self.validate_loca_format()?;
        self.validate_glyph_data_format()?;
        Ok(())
    }
}

/// OpenType 'head' table field reference.
pub mod field_docs {
    /// Major version of the head table
    pub const MAJOR_VERSION: &str = "Major version of the head table";
    /// Minor version of the head table
    pub const MINOR_VERSION: &str = "Minor version of the head table";
    /// Font revision fixed-point
    pub const FONT_REVISION: &str = "Font revision fixed-point";
    /// Checksum adjustment to ignore
    pub const CHECKSUM_ADJUSTMENT: &str = "Checksum adjustment to ignore";
    /// Must be 0x5F0F3CF5
    pub const MAGIC_NUMBER: &str = "Must be 0x5F0F3CF5";
    /// Font-wide flag bits
    pub const FLAGS: &str = "Font-wide flag bits";
    /// Units per em square
    pub const UNITS_PER_EM: &str = "Units per em square";
    /// 0=short loca, 1=long loca
    pub const INDEX_TO_LOC_FORMAT: &str = "0=short loca, 1=long loca";
}
impl HeadTable {
    pub fn flags_baseline_at_y0(&self) -> bool { self.flags & 0x01 != 0 }
    pub fn flags_sidebearing_left(&self) -> bool { self.flags & 0x02 != 0 }
    pub fn flags_instructions_depend_on_size(&self) -> bool { self.flags & 0x04 != 0 }
    pub fn mac_style_bold(&self) -> bool { self.mac_style & 0x01 != 0 }
    pub fn mac_style_italic(&self) -> bool { self.mac_style & 0x02 != 0 }
}

/* depth:head */

impl HeadTable {
    pub fn compute_bbox_from_glyphs(&self, boxes: &[crate::outline::bbox::BoundingBox]) -> crate::outline::bbox::BoundingBox {
        boxes.iter().copied().fold(crate::outline::bbox::BoundingBox::empty(), |a, b| a.union(b))
    }
    pub fn created_seconds(&self) -> u64 { self.created / (1u64 << 32) }
    pub fn modified_seconds(&self) -> u64 { self.modified / (1u64 << 32) }
}

/* field_matrix:head */
pub mod field_readers_head {
pub fn read_major_version(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[0], data[1]]))
}
pub fn read_minor_version(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[2], data[3]]))
}
pub fn read_font_revision(data: &[u8]) -> crate::GlyphResult<i32> {
    crate::limits::check_len(2, data.len())?;
    Ok(i32::from_be_bytes([data[4], data[5], data[6], data[7]]))
}
pub fn read_checksum_adjustment(data: &[u8]) -> crate::GlyphResult<u32> {
    crate::limits::check_len(2, data.len())?;
    Ok(u32::from_be_bytes([data[8], data[9], data[10], data[11]]))
}
pub fn read_magic(data: &[u8]) -> crate::GlyphResult<u32> {
    crate::limits::check_len(2, data.len())?;
    Ok(u32::from_be_bytes([data[12], data[13], data[14], data[15]]))
}
pub fn read_flags(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[16], data[17]]))
}
pub fn read_units_per_em(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[18], data[19]]))
}
pub fn read_x_min(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[36], data[37]]))
}
pub fn read_y_min(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[38], data[39]]))
}
pub fn read_x_max(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[40], data[41]]))
}
pub fn read_y_max(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[42], data[43]]))
}
pub fn read_mac_style(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[44], data[45]]))
}
pub fn read_lowest_ppem(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[46], data[47]]))
}
pub fn read_index_to_loc_format(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[50], data[51]]))
}
pub fn read_glyph_data_format(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[52], data[53]]))
}
}

/* field_checks:head */
impl HeadTable {
pub fn check_units_per_em(&self) -> crate::GlyphResult<()> {
    if self.units_per_em < 16 {
        return Err(crate::GlyphError::Consistency { detail: "unitsPerEm too small" });
    }
    Ok(())
}
pub fn check_units_per_em_max(&self) -> crate::GlyphResult<()> {
    if self.units_per_em > 16384 {
        return Err(crate::GlyphError::Consistency { detail: "unitsPerEm too large" });
    }
    Ok(())
}
pub fn check_magic(&self) -> crate::GlyphResult<()> {
    if self.magic_number != crate::tables::head::HEAD_MAGIC {
        return Err(crate::GlyphError::Consistency { detail: "bad head magic" });
    }
    Ok(())
}
pub fn check_bbox_x(&self) -> crate::GlyphResult<()> {
    if self.x_min > self.x_max {
        return Err(crate::GlyphError::Consistency { detail: "head x bbox" });
    }
    Ok(())
}
pub fn check_bbox_y(&self) -> crate::GlyphResult<()> {
    if self.y_min > self.y_max {
        return Err(crate::GlyphError::Consistency { detail: "head y bbox" });
    }
    Ok(())
}
pub fn check_loca_fmt(&self) -> crate::GlyphResult<()> {
    if self.index_to_loc_format != 0 && self.index_to_loc_format != 1 {
        return Err(crate::GlyphError::Consistency { detail: "bad loca fmt" });
    }
    Ok(())
}
pub fn check_glyph_fmt(&self) -> crate::GlyphResult<()> {
    if self.glyph_data_format != 0 {
        return Err(crate::GlyphError::Consistency { detail: "bad glyph fmt" });
    }
    Ok(())
}
pub fn check_mac_bold_italic(&self) -> crate::GlyphResult<()> {
    if self.mac_style > 0xFF {
        return Err(crate::GlyphError::Consistency { detail: "macStyle overflow" });
    }
    Ok(())
}
pub fn check_lowest_ppem(&self) -> crate::GlyphResult<()> {
    if self.lowest_rec_ppem == 0 {
        return Err(crate::GlyphError::Consistency { detail: "lowestRecPPEM zero" });
    }
    Ok(())
}
pub fn check_font_direction(&self) -> crate::GlyphResult<()> {
    if self.font_direction_hint != 2 && self.font_direction_hint != 0 {
        return Err(crate::GlyphError::Consistency { detail: "fontDirectionHint" });
    }
    Ok(())
}
}

/* walker:head */

impl HeadTable {
    pub fn serialize_fields<W: FnMut(u16) -> crate::GlyphResult<()>>(&self, mut write_u16: W) -> crate::GlyphResult<()> {
        write_u16(self.major_version)?;
        write_u16(self.minor_version)?;
        write_u16(self.units_per_em)?;
        write_u16(self.flags)?;
        Ok(())
    }
}
