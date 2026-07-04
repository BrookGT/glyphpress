//! Validate loca/glyf/hmtx/maxp relationships.


use crate::error::{GlyphError, GlyphResult};
use crate::tables::head::HeadTable;
use crate::tables::hhea::HheaTable;
use crate::tables::hmtx::HmtxTable;
use crate::tables::loca::LocaTable;
use crate::tables::maxp::MaxpTable;

pub fn validate_font_consistency(
    head: &HeadTable,
    maxp: &MaxpTable,
    hhea: &HheaTable,
    hmtx: &HmtxTable,
    loca: &LocaTable,
) -> GlyphResult<()> {
    head.full_validate()?;
    maxp.validate_limits()?;
    hhea.validate_metrics_count(maxp.num_glyphs)?;
    hmtx.validate_counts(maxp.num_glyphs)?;
    loca.validate_against_num_glyphs(maxp.num_glyphs)?;
    if head.uses_long_loca() != loca.long_format {
        return Err(GlyphError::LocaFormatMismatch {
            head: head.index_to_loc_format,
            loca_len: loca.offsets.len(),
        });
    }
    Ok(())
}

/* volume */

pub fn check_glyph_count_agreement(maxp: u16, loca_len: usize) -> crate::GlyphResult<()> {
    if loca_len != maxp as usize + 1 {
        return Err(crate::GlyphError::Consistency { detail: "loca/maxp glyph count" });
    }
    Ok(())
}
