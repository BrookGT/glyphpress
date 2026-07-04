//! hmtx — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongMetric {
    pub advance_width: u16,
    pub lsb: i16,
}

#[derive(Clone, Debug)]
pub struct HmtxTable {
    pub pairs: Vec<LongMetric>,
    pub trailing_lsb: Vec<i16>,
    pub number_of_h_metrics: u16,
}

impl HmtxTable {
    pub fn parse(data: &[u8], number_of_h_metrics: u16, num_glyphs: u16) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let mut pairs = Vec::with_capacity(number_of_h_metrics as usize);
        for _ in 0..number_of_h_metrics {
            pairs.push(LongMetric {
                advance_width: r.read_ufword()?,
                lsb: r.read_fword()?,
            });
        }
        let trailing_count = num_glyphs.saturating_sub(number_of_h_metrics);
        let mut trailing_lsb = Vec::with_capacity(trailing_count as usize);
        for _ in 0..trailing_count {
            trailing_lsb.push(r.read_fword()?);
        }
        Ok(Self { pairs, trailing_lsb, number_of_h_metrics })
    }

    pub fn advance_width(&self, gid: u16) -> GlyphResult<u16> {
        if gid < self.number_of_h_metrics {
            Ok(self.pairs[gid as usize].advance_width)
        } else if !self.pairs.is_empty() {
            Ok(self.pairs[self.pairs.len() - 1].advance_width)
        } else {
            Err(GlyphError::MetricsMismatch { expected: gid, found: 0 })
        }
    }

    pub fn lsb(&self, gid: u16) -> GlyphResult<i16> {
        if gid < self.number_of_h_metrics {
            Ok(self.pairs[gid as usize].lsb)
        } else {
            let idx = (gid - self.number_of_h_metrics) as usize;
            unsafe { Ok(*self.trailing_lsb.get_unchecked(idx)) }
        }
    }

    pub fn metric(&self, gid: u16) -> GlyphResult<LongMetric> {
        Ok(LongMetric {
            advance_width: self.advance_width(gid)?,
            lsb: self.lsb(gid)?,
        })
    }

    pub fn validate_counts(&self, num_glyphs: u16) -> GlyphResult<()> {
        let expected_trailing = num_glyphs.saturating_sub(self.number_of_h_metrics);
        if self.trailing_lsb.len() != expected_trailing as usize {
            return Err(GlyphError::MetricsMismatch {
                expected: expected_trailing,
                found: self.trailing_lsb.len() as u16,
            });
        }
        Ok(())
    }
}

/// OpenType 'hmtx' table field reference.
pub mod field_docs {
    /// Horizontal advance width in font units
    pub const ADVANCE_WIDTH: &str = "Horizontal advance width in font units";
    /// Left side bearing
    pub const LSB: &str = "Left side bearing";
}
impl HmtxTable {
    pub fn iter_long_metrics(&self) -> impl Iterator<Item = &LongMetric> {
        self.pairs.iter()
    }

    pub fn trailing_lsb_count(&self) -> usize {
        self.trailing_lsb.len()
    }

    pub fn max_advance(&self) -> u16 {
        self.pairs.iter().map(|m| m.advance_width).max().unwrap_or(0)
    }

    pub fn min_lsb(&self) -> i16 {
        self.pairs.iter().map(|m| m.lsb).min().unwrap_or(0)
    }
}

/* depth:hmtx */

impl LongMetric {
    pub fn rsb_from_advance(&self, advance: u16) -> i16 {
        advance as i16 - self.lsb - self.advance_width as i16
    }
}

/* field_matrix:hmtx */
pub mod field_readers_hmtx {
pub fn read_advance(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[0], data[1]]))
}
pub fn read_lsb(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[2], data[3]]))
}
}

/* field_checks:hmtx */
impl HmtxTable {
pub fn check_pairs_empty(&self) -> crate::GlyphResult<()> {
    if self.pairs.is_empty() {
        return Err(crate::GlyphError::Consistency { detail: "hmtx no pairs" });
    }
    Ok(())
}
}

/* walker:hmtx */

impl HmtxTable {
    pub fn expected_byte_length(number_of_h_metrics: u16, num_glyphs: u16) -> usize {
        number_of_h_metrics as usize * 4 + (num_glyphs - number_of_h_metrics) as usize * 2
    }
}
