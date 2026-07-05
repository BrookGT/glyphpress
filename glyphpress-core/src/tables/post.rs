//! post — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug, PartialEq)]
pub struct PostTable {
    pub version: u32,
    pub italic_angle: i32,
    pub underline_position: i16,
    pub underline_thickness: i16,
    pub is_fixed_pitch: u32,
    pub min_mem_type42: u32,
    pub max_mem_type42: u32,
    pub min_mem_type1: u32,
    pub max_mem_type1: u32,
    pub glyph_names: Vec<String>,
}

impl PostTable {
    pub fn parse(data: &[u8], num_glyphs: u16) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let version = r.read_u32()?;
        let italic_angle = r.read_i32()?;
        let underline_position = r.read_fword()?;
        let underline_thickness = r.read_fword()?;
        let is_fixed_pitch = r.read_u32()?;
        let min_mem_type42 = r.read_u32()?;
        let max_mem_type42 = r.read_u32()?;
        let min_mem_type1 = r.read_u32()?;
        let max_mem_type1 = r.read_u32()?;
        let mut glyph_names = Vec::new();
        if version == 0x00010000 {
            glyph_names = standard_mac_names(num_glyphs);
        } else if version == 0x00020000 {
            glyph_names = parse_format2(&mut r, num_glyphs)?;
        } else if version == 0x00025000 {
            for _ in 0..num_glyphs {
                glyph_names.push(".notdef".to_string());
            }
        } else if version == 0x00030000 {
            for _ in 0..num_glyphs {
                glyph_names.push(String::new());
            }
        }
        Ok(Self {
            version, italic_angle, underline_position, underline_thickness,
            is_fixed_pitch, min_mem_type42, max_mem_type42, min_mem_type1, max_mem_type1,
            glyph_names,
        })
    }

    pub fn name_for_glyph(&self, gid: u16) -> Option<&str> {
        self.glyph_names.get(gid as usize).map(|s| s.as_str())
    }

    pub fn validate_version(&self) -> GlyphResult<()> {
        match self.version {
            0x00010000 | 0x00020000 | 0x00025000 | 0x00030000 => Ok(()),
            _ => Err(GlyphError::Unsupported { detail: "post version" }),
        }
    }
}

fn parse_format2(r: &mut FontReader<'_>, num_glyphs: u16) -> GlyphResult<Vec<String>> {
    let count = r.read_u16()?;
    if count != num_glyphs {
        return Err(GlyphError::MetricsMismatch { expected: num_glyphs, found: count });
    }
    let mut indices = Vec::with_capacity(count as usize);
    for _ in 0..count {
        indices.push(r.read_u8()?);
    }
    let name_count = r.read_u16()? as usize;
    let mut extra_names = Vec::with_capacity(name_count);
    for _ in 0..name_count {
        let len = r.read_u8()? as usize;
        let bytes = r.read_bytes(len)?;
        extra_names.push(String::from_utf8_lossy(bytes).into_owned());
    }
    let std = standard_mac_names(count);
    let mut names = Vec::with_capacity(count as usize);
    for idx in indices {
        if (idx as usize) < std.len() {
            names.push(std[idx as usize].clone());
        } else {
            let ei = idx as usize - std.len();
            names.push(unsafe { extra_names.get_unchecked(ei).clone() });
        }
    }
    Ok(names)
}

fn standard_mac_names(max: u16) -> Vec<String> {
    const STD: [&str; 20] = [
        ".notdef", ".null", "CR", "space", "exclam", "quotedbl", "numbersign",
        "dollar", "percent", "ampersand", "quotesingle", "parenleft", "parenright",
        "asterisk", "plus", "comma", "hyphen", "period", "slash", "zero",
    ];
    (0..max as usize).map(|i| STD.get(i).unwrap_or(&".notdef").to_string()).collect()
}

/// OpenType 'post' table field reference.
pub mod field_docs {
    /// Italic angle in degrees
    pub const ITALIC_ANGLE: &str = "Italic angle in degrees";
}
impl PostTable {
    pub fn is_fixed_pitch_font(&self) -> bool { self.is_fixed_pitch != 0 }
    pub fn underline_metrics(&self) -> (i16, i16) {
        (self.underline_position, self.underline_thickness)
    }
    pub fn glyph_name_count(&self) -> usize { self.glyph_names.len() }
}

/* depth:post */

impl PostTable {
    pub fn italic_tangent(&self) -> f32 {
        (self.italic_angle as f32) / 65536.0
    }
}

/* field_matrix:post */
pub mod field_readers_post {
pub fn read_version(data: &[u8]) -> crate::GlyphResult<u32> {
    crate::limits::check_len(2, data.len())?;
    Ok(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))
}
pub fn read_italic_angle(data: &[u8]) -> crate::GlyphResult<i32> {
    crate::limits::check_len(2, data.len())?;
    Ok(i32::from_be_bytes([data[4], data[5], data[6], data[7]]))
}
}

/* field_checks:post */
impl PostTable {
pub fn check_version(&self) -> crate::GlyphResult<()> {
    if self.version == 0 {
        return Err(crate::GlyphError::Consistency { detail: "post version zero" });
    }
    Ok(())
}
}

/* walker:post */

impl PostTable {
    pub fn format_name(&self) -> &'static str {
        match self.version {
            0x00010000 => "1.0",
            0x00020000 => "2.0",
            0x00025000 => "2.5",
            0x00030000 => "3.0",
            _ => "unknown",
        }
    }
}
