//! OpenType specification field notes for embedded parsers.


pub mod notes {
    pub const HEAD_FONTREVISION: &str = "Fixed-point font revision";
    pub const HEAD_FLAGS: &str = "Bit 0 baseline at y=0; bit 1 lsb at x=0";
    pub const HHEA_ASCENDER: &str = "Typographic ascender in font units";
    pub const MAXP_NUMGLYPHS: &str = "Total glyphs including notdef";
    pub const CMAP_FORMAT4: &str = "Segment mapping to delta and glyphIdArray";
    pub const GLYF_CONTOURS: &str = "Negative count indicates composite glyph";
    pub const LOCA_INDEXTOLOCFORMAT: &str = "0=uint16 offsets x2, 1=uint32 offsets";
    pub const HMTX_NUMOFLONGHORMETRICS: &str = "Glyphs with explicit aw+lsb pairs";
    pub const NAME_PLATFORMID: &str = "0=Unicode, 1=Mac, 3=Windows";
    pub const POST_FORMAT: &str = "1.0 Macintosh, 2.0 names, 3.0 no names";
    pub const OS2_FSSELECTION: &str = "Regular/Bold/Italic/Underscore flags";
    pub const KERN_COVERAGE: &str = "Horizontal/vertical/minimum bits";
    pub const GDEF_GLYPHCLASSDEF: &str = "Class 1=base 2=ligature 3=mark 4=component";
    pub const GPOS_LOOKUPLIST: &str = "Positioning lookups chained in features";
    pub const GSUB_LOOKUPLIST: &str = "Substitution lookups for features";
    pub fn note_head_fontRevision() -> &'static str { HEAD_FONTREVISION }
    pub fn note_head_flags() -> &'static str { HEAD_FLAGS }
    pub fn note_hhea_ascender() -> &'static str { HHEA_ASCENDER }
    pub fn note_maxp_numGlyphs() -> &'static str { MAXP_NUMGLYPHS }
    pub fn note_cmap_format4() -> &'static str { CMAP_FORMAT4 }
    pub fn note_glyf_contours() -> &'static str { GLYF_CONTOURS }
    pub fn note_loca_indexToLocFormat() -> &'static str { LOCA_INDEXTOLOCFORMAT }
    pub fn note_hmtx_numOfLongHorMetrics() -> &'static str { HMTX_NUMOFLONGHORMETRICS }
    pub fn note_name_platformID() -> &'static str { NAME_PLATFORMID }
    pub fn note_post_format() -> &'static str { POST_FORMAT }
    pub fn note_os2_fsSelection() -> &'static str { OS2_FSSELECTION }
    pub fn note_kern_coverage() -> &'static str { KERN_COVERAGE }
    pub fn note_gdef_GlyphClassDef() -> &'static str { GDEF_GLYPHCLASSDEF }
    pub fn note_gpos_LookupList() -> &'static str { GPOS_LOOKUPLIST }
    pub fn note_gsub_LookupList() -> &'static str { GSUB_LOOKUPLIST }
}
