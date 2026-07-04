//! maxp — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxpTable {
    pub version: u32,
    pub num_glyphs: u16,
    pub max_points: u16,
    pub max_contours: u16,
    pub max_composite_points: u16,
    pub max_composite_contours: u16,
    pub max_zones: u16,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}

impl MaxpTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let version = r.read_u32()?;
        let num_glyphs = r.read_u16()?;
        if num_glyphs == 0 {
            return Err(GlyphError::OutOfRange {
                field: "numGlyphs",
                value: 0,
            });
        }
        let mut tbl = MaxpTable {
            version,
            num_glyphs,
            max_points: 0,
            max_contours: 0,
            max_composite_points: 0,
            max_composite_contours: 0,
            max_zones: 0,
            max_twilight_points: 0,
            max_storage: 0,
            max_function_defs: 0,
            max_instruction_defs: 0,
            max_stack_elements: 0,
            max_size_of_instructions: 0,
            max_component_elements: 0,
            max_component_depth: 0,
        };
        if version == 0x0001_0000 {
            tbl.max_points = r.read_u16()?;
            tbl.max_contours = r.read_u16()?;
            tbl.max_composite_points = r.read_u16()?;
            tbl.max_composite_contours = r.read_u16()?;
            tbl.max_zones = r.read_u16()?;
            tbl.max_twilight_points = r.read_u16()?;
            tbl.max_storage = r.read_u16()?;
            tbl.max_function_defs = r.read_u16()?;
            tbl.max_instruction_defs = r.read_u16()?;
            tbl.max_stack_elements = r.read_u16()?;
            tbl.max_size_of_instructions = r.read_u16()?;
            tbl.max_component_elements = r.read_u16()?;
            tbl.max_component_depth = r.read_u16()?;
        }
        Ok(tbl)
    }

    pub fn validate_limits(&self) -> GlyphResult<()> {
        if self.max_points as usize > limits::MAX_POINTS_PER_GLYPH {
            return Err(GlyphError::OutOfRange {
                field: "maxPoints",
                value: self.max_points as i64,
            });
        }
        if self.max_contours as usize > limits::MAX_CONTOURS_PER_GLYPH {
            return Err(GlyphError::OutOfRange {
                field: "maxContours",
                value: self.max_contours as i64,
            });
        }
        Ok(())
    }
}

/// OpenType 'maxp' table field reference.
pub mod field_docs {
    /// Total glyph count
    pub const NUM_GLYPHS: &str = "Total glyph count";
    /// Max points in non-composite glyph
    pub const MAX_POINTS: &str = "Max points in non-composite glyph";
    /// Max contours in non-composite glyph
    pub const MAX_CONTOURS: &str = "Max contours in non-composite glyph";
}

/* depth:maxp */

impl MaxpTable {
    pub fn is_truetype_1_5(&self) -> bool { self.version == 0x00005000 }
    pub fn is_truetype_1_0(&self) -> bool { self.version == 0x00010000 }
    pub fn composite_limits(&self) -> (u16, u16, u16) {
        (self.max_composite_points, self.max_composite_contours, self.max_component_depth)
    }
}

/* field_matrix:maxp */
pub mod field_readers_maxp {
pub fn read_version(data: &[u8]) -> crate::GlyphResult<u32> {
    crate::limits::check_len(2, data.len())?;
    Ok(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))
}
pub fn read_num_glyphs(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[4], data[5]]))
}
pub fn read_max_points(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[6], data[7]]))
}
pub fn read_max_contours(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[8], data[9]]))
}
pub fn read_max_component_depth(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[30], data[31]]))
}
}

/* field_checks:maxp */
impl MaxpTable {
pub fn check_num_glyphs_zero(&self) -> crate::GlyphResult<()> {
    if self.num_glyphs == 0 {
        return Err(crate::GlyphError::Consistency { detail: "numGlyphs zero" });
    }
    Ok(())
}
pub fn check_max_points(&self) -> crate::GlyphResult<()> {
    if self.max_points > 65535 {
        return Err(crate::GlyphError::Consistency { detail: "maxPoints" });
    }
    Ok(())
}
pub fn check_max_contours(&self) -> crate::GlyphResult<()> {
    if self.max_contours > 65535 {
        return Err(crate::GlyphError::Consistency { detail: "maxContours" });
    }
    Ok(())
}
pub fn check_max_component_depth(&self) -> crate::GlyphResult<()> {
    if self.max_component_depth > 32 {
        return Err(crate::GlyphError::Consistency { detail: "maxComponentDepth" });
    }
    Ok(())
}
}

/* walker:maxp */

impl MaxpTable {
    pub fn expected_table_size(&self) -> usize {
        if self.is_truetype_1_0() { 32 } else { 6 }
    }
}
