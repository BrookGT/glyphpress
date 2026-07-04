//! Hard limits for embedded-device font parsing.


/// Maximum SFNT file size accepted by glyphpress on constrained targets.
pub const MAX_FONT_BYTES: usize = 16 * 1024 * 1024;

/// Maximum number of tables in a single font.
pub const MAX_TABLE_COUNT: usize = 64;

/// Maximum glyph count for subsetting.
pub const MAX_GLYPH_COUNT: u16 = 65535;

/// Maximum points per simple glyph outline.
pub const MAX_POINTS_PER_GLYPH: usize = 8192;

/// Maximum contours per glyph.
pub const MAX_CONTOURS_PER_GLYPH: usize = 512;

/// Maximum cmap subtables.
pub const MAX_CMAP_SUBTABLES: usize = 32;

/// Maximum name records.
pub const MAX_NAME_RECORDS: usize = 256;

/// Maximum kern subtables.
pub const MAX_KERN_SUBTABLES: usize = 16;

/// Maximum composite glyph component depth.
pub const MAX_COMPOSITE_DEPTH: usize = 32;

/// Maximum bytes for a single table payload during emit.
pub const MAX_TABLE_EMIT_BYTES: usize = 8 * 1024 * 1024;

/// Scratch arena default capacity for outline walks.
pub const SCRATCH_DEFAULT_CAP: usize = 4096;

pub fn check_glyph_index(gid: u16, num_glyphs: u16) -> crate::GlyphResult<()> {
    if gid >= num_glyphs {
        return Err(crate::GlyphError::GlyphIndexOutOfRange {
            gid,
            max: num_glyphs.saturating_sub(1),
        });
    }
    Ok(())
}

pub fn check_len(need: usize, have: usize) -> crate::GlyphResult<()> {
    if have < need {
        return Err(crate::GlyphError::truncated(need, have));
    }
    Ok(())
}
