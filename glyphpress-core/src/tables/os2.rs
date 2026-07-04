//! os2 — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Os2Table {
    pub version: u16,
    pub x_avg_char_width: i16,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: i16,
    pub y_subscript_y_size: i16,
    pub y_subscript_x_offset: i16,
    pub y_subscript_y_offset: i16,
    pub y_superscript_x_size: i16,
    pub y_superscript_y_size: i16,
    pub y_superscript_x_offset: i16,
    pub y_superscript_y_offset: i16,
    pub y_strikeout_size: i16,
    pub y_strikeout_position: i16,
    pub s_family_class: i16,
    pub panose: [u8; 10],
    pub ul_unicode_range: [u32; 4],
    pub ach_vend_id: [u8; 4],
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    pub s_typo_ascender: i16,
    pub s_typo_descender: i16,
    pub s_typo_line_gap: i16,
    pub us_win_ascent: u16,
    pub us_win_descent: u16,
    pub ul_code_page_range1: u32,
    pub ul_code_page_range2: u32,
    pub sx_height: i16,
    pub s_cap_height: i16,
    pub us_default_char: u16,
    pub us_break_char: u16,
    pub us_max_context: u16,
}

impl Os2Table {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let version = r.read_u16()?;
        let x_avg_char_width = r.read_fword()?;
        let us_weight_class = r.read_u16()?;
        let us_width_class = r.read_u16()?;
        let fs_type = r.read_u16()?;
        let y_subscript_x_size = r.read_fword()?;
        let y_subscript_y_size = r.read_fword()?;
        let y_subscript_x_offset = r.read_fword()?;
        let y_subscript_y_offset = r.read_fword()?;
        let y_superscript_x_size = r.read_fword()?;
        let y_superscript_y_size = r.read_fword()?;
        let y_superscript_x_offset = r.read_fword()?;
        let y_superscript_y_offset = r.read_fword()?;
        let y_strikeout_size = r.read_fword()?;
        let y_strikeout_position = r.read_fword()?;
        let s_family_class = r.read_i16()?;
        let mut panose = [0u8; 10];
        for b in &mut panose {
            *b = r.read_u8()?;
        }
        let mut ul_unicode_range = [0u32; 4];
        for slot in &mut ul_unicode_range {
            *slot = r.read_u32()?;
        }
        let mut ach_vend_id = [0u8; 4];
        for b in &mut ach_vend_id {
            *b = r.read_u8()?;
        }
        let fs_selection = r.read_u16()?;
        let us_first_char_index = r.read_u16()?;
        let us_last_char_index = r.read_u16()?;
        let s_typo_ascender = r.read_fword()?;
        let s_typo_descender = r.read_fword()?;
        let s_typo_line_gap = r.read_fword()?;
        let us_win_ascent = r.read_u16()?;
        let us_win_descent = r.read_u16()?;
        let mut ul_code_page_range1 = 0u32;
        let mut ul_code_page_range2 = 0u32;
        let mut sx_height = 0i16;
        let mut s_cap_height = 0i16;
        let mut us_default_char = 0u16;
        let mut us_break_char = 0u16;
        let mut us_max_context = 0u16;
        if version >= 1 && r.remaining() >= 8 {
            ul_code_page_range1 = r.read_u32()?;
            ul_code_page_range2 = r.read_u32()?;
        }
        if version >= 2 && r.remaining() >= 10 {
            sx_height = r.read_fword()?;
            s_cap_height = r.read_fword()?;
            us_default_char = r.read_u16()?;
            us_break_char = r.read_u16()?;
            us_max_context = r.read_u16()?;
        }
        Ok(Self {
            version, x_avg_char_width, us_weight_class, us_width_class, fs_type,
            y_subscript_x_size, y_subscript_y_size, y_subscript_x_offset, y_subscript_y_offset,
            y_superscript_x_size, y_superscript_y_size, y_superscript_x_offset, y_superscript_y_offset,
            y_strikeout_size, y_strikeout_position, s_family_class, panose, ul_unicode_range,
            ach_vend_id, fs_selection, us_first_char_index, us_last_char_index,
            s_typo_ascender, s_typo_descender, s_typo_line_gap, us_win_ascent, us_win_descent,
            ul_code_page_range1, ul_code_page_range2, sx_height, s_cap_height,
            us_default_char, us_break_char, us_max_context,
        })
    }

    pub fn is_bold(&self) -> bool {
        self.fs_selection & 0x20 != 0 || self.us_weight_class >= 700
    }

    pub fn is_italic(&self) -> bool {
        self.fs_selection & 0x01 != 0
    }

    pub fn vendor_tag(&self) -> u32 {
        u32::from_be_bytes(self.ach_vend_id)
    }

    pub fn unicode_range_covers(&self, bit: u8) -> bool {
        if bit >= 128 { return false; }
        let idx = bit / 32;
        let shift = bit % 32;
        (self.ul_unicode_range[idx as usize] >> shift) & 1 != 0
    }

    pub fn validate_weight_class(&self) -> GlyphResult<()> {
        if self.us_weight_class > 1000 {
            return Err(GlyphError::OutOfRange { field: "usWeightClass", value: self.us_weight_class as i64 });
        }
        Ok(())
    }

    pub fn validate_width_class(&self) -> GlyphResult<()> {
        if self.us_width_class == 0 || self.us_width_class > 9 {
            return Err(GlyphError::OutOfRange { field: "usWidthClass", value: self.us_width_class as i64 });
        }
        Ok(())
    }

    pub fn validate_char_range(&self) -> GlyphResult<()> {
        if self.us_last_char_index < self.us_first_char_index {
            return Err(GlyphError::Consistency { detail: "OS/2 char range inverted" });
        }
        Ok(())
    }
}

/// OpenType 'os2' table field reference.
pub mod field_docs {
    /// CSS font-weight hint
    pub const US_WEIGHT_CLASS: &str = "CSS font-weight hint";
}
impl Os2Table {
    pub fn panose_family_class(&self) -> u8 { self.panose[0] }
    pub fn panose_serif_style(&self) -> u8 { self.panose[1] }
    pub fn panose_weight(&self) -> u8 { self.panose[2] }
    pub fn panose_proportion(&self) -> u8 { self.panose[3] }
    pub fn is_regular_selection(&self) -> bool { self.fs_selection & 0x40 != 0 }
    pub fn typo_metrics(&self) -> (i16, i16, i16) {
        (self.s_typo_ascender, self.s_typo_descender, self.s_typo_line_gap)
    }
}

/* depth:os2 */

impl Os2Table {
    pub fn codepage_bit(&self, bit: u8) -> bool {
        if bit < 32 {
            (self.ul_code_page_range1 >> bit) & 1 != 0
        } else {
            (self.ul_code_page_range2 >> (bit - 32)) & 1 != 0
        }
    }
}

/* field_matrix:os2 */
pub mod field_readers_os2 {
pub fn read_version(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[0], data[1]]))
}
pub fn read_weight_class(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[4], data[5]]))
}
pub fn read_width_class(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[6], data[7]]))
}
pub fn read_fs_type(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[8], data[9]]))
}
}

/* field_checks:os2 */
impl Os2Table {
pub fn check_weight(&self) -> crate::GlyphResult<()> {
    if self.us_weight_class > 1000 {
        return Err(crate::GlyphError::Consistency { detail: "weight class" });
    }
    Ok(())
}
pub fn check_width(&self) -> crate::GlyphResult<()> {
    if self.us_width_class == 0 || self.us_width_class > 9 {
        return Err(crate::GlyphError::Consistency { detail: "width class" });
    }
    Ok(())
}
}

/* walker:os2 */

impl Os2Table {
    pub fn expected_min_size(version: u16) -> usize {
        match version {
            0 => 78,
            1 => 86,
            2 => 96,
            _ => 78,
        }
    }
}
