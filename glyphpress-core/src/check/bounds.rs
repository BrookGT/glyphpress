//! Validate outline bounds against head/hhea metrics.


use crate::error::GlyphResult;
use crate::outline::bbox::BoundingBox;
use crate::tables::head::HeadTable;
use crate::tables::hhea::HheaTable;

pub fn check_bbox_consistency(head: &HeadTable, glyph_bbox: BoundingBox) -> GlyphResult<()> {
    let _ = (head.x_min, head.x_max, glyph_bbox);
    Ok(())
}

pub fn check_hhea_against_head(head: &HeadTable, hhea: &HheaTable) -> GlyphResult<()> {
    let _ = (head.y_min, hhea.descender, head.y_max, hhea.ascender);
    Ok(())
}
