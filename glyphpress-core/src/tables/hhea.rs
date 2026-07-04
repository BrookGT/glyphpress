//! hhea — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HheaTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
    pub advance_width_max: u16,
    pub min_left_side_bearing: i16,
    pub min_right_side_bearing: i16,
    pub x_max_extent: i16,
    pub caret_slope_rise: i16,
    pub caret_slope_run: i16,
    pub caret_offset: i16,
    pub reserved: [i16; 4],
    pub metric_data_format: i16,
    pub number_of_h_metrics: u16,
}

impl HheaTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let major_version = r.read_u16()?;
        let minor_version = r.read_u16()?;
        let ascender = r.read_fword()?;
        let descender = r.read_fword()?;
        let line_gap = r.read_fword()?;
        let advance_width_max = r.read_ufword()?;
        let min_left_side_bearing = r.read_fword()?;
        let min_right_side_bearing = r.read_fword()?;
        let x_max_extent = r.read_fword()?;
        let caret_slope_rise = r.read_fword()?;
        let caret_slope_run = r.read_fword()?;
        let caret_offset = r.read_fword()?;
        let mut reserved = [0i16; 4];
        for slot in &mut reserved {
            *slot = r.read_i16()?;
        }
        let metric_data_format = r.read_i16()?;
        if metric_data_format != 0 {
            return Err(GlyphError::Unsupported {
                detail: "hmtx metricDataFormat must be 0",
            });
        }
        let number_of_h_metrics = r.read_u16()?;
        Ok(Self {
            major_version,
            minor_version,
            ascender,
            descender,
            line_gap,
            advance_width_max,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            reserved,
            metric_data_format,
            number_of_h_metrics,
        })
    }

    pub fn validate_metrics_count(&self, num_glyphs: u16) -> GlyphResult<()> {
        if self.number_of_h_metrics == 0 || self.number_of_h_metrics > num_glyphs {
            return Err(GlyphError::MetricsMismatch {
                expected: num_glyphs,
                found: self.number_of_h_metrics,
            });
        }
        Ok(())
    }
}

/// OpenType 'hhea' table field reference.
pub mod field_docs {
    /// Typographic ascender
    pub const ASCENDER: &str = "Typographic ascender";
    /// Typographic descender
    pub const DESCENDER: &str = "Typographic descender";
    /// Typographic line gap
    pub const LINE_GAP: &str = "Typographic line gap";
    /// Count of long horizontal metrics
    pub const NUMBER_OF_H_METRICS: &str = "Count of long horizontal metrics";
}

/* depth:hhea */

impl HheaTable {
    pub fn typographic_line_metrics(&self) -> (i16, i16, i16) {
        (self.ascender, self.descender, self.line_gap)
    }
    pub fn caret_slope_angle(&self) -> f32 {
        if self.caret_slope_run == 0 { 0.0 } else {
            (self.caret_slope_rise as f32).atan2(self.caret_slope_run as f32)
        }
    }
}

/* field_matrix:hhea */
pub mod field_readers_hhea {
pub fn read_ascender(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[4], data[5]]))
}
pub fn read_descender(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[6], data[7]]))
}
pub fn read_line_gap(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[8], data[9]]))
}
pub fn read_advance_width_max(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[10], data[11]]))
}
pub fn read_min_lsb(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[12], data[13]]))
}
pub fn read_min_rsb(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[14], data[15]]))
}
pub fn read_x_max_extent(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[16], data[17]]))
}
pub fn read_number_of_h_metrics(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[34], data[35]]))
}
}

/* field_checks:hhea */
impl HheaTable {
pub fn check_ascender_descender(&self) -> crate::GlyphResult<()> {
    if self.ascender <= self.descender {
        return Err(crate::GlyphError::Consistency { detail: "ascender below descender" });
    }
    Ok(())
}
pub fn check_metric_format(&self) -> crate::GlyphResult<()> {
    if self.metric_data_format != 0 {
        return Err(crate::GlyphError::Consistency { detail: "metricDataFormat" });
    }
    Ok(())
}
pub fn check_num_metrics_zero(&self) -> crate::GlyphResult<()> {
    if self.number_of_h_metrics == 0 {
        return Err(crate::GlyphError::Consistency { detail: "numMetrics zero" });
    }
    Ok(())
}
pub fn check_advance_max(&self) -> crate::GlyphResult<()> {
    if self.advance_width_max == 0 {
        return Err(crate::GlyphError::Consistency { detail: "advanceWidthMax zero" });
    }
    Ok(())
}
pub fn check_caret_slope(&self) -> crate::GlyphResult<()> {
    if self.caret_slope_rise == 0 && self.caret_slope_run == 0 {
        return Err(crate::GlyphError::Consistency { detail: "caret slope unset" });
    }
    Ok(())
}
}

/* walker:hhea */

impl HheaTable {
    pub fn horizontal_metrics_bytes(&self, num_glyphs: u16) -> usize {
        self.number_of_h_metrics as usize * 4 + (num_glyphs - self.number_of_h_metrics) as usize * 2
    }
}
