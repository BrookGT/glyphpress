//! OpenType field reference and validation notes.

pub struct TableField { pub table: &'static str, pub field: &'static str, pub note: &'static str }
pub const FIELDS: &[TableField] = &[
    TableField { table: "head", field: "unitsPerEm_0", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_1", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_2", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_3", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_4", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_5", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_6", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_7", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_8", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_9", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_10", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_11", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_12", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_13", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_14", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_15", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_16", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_17", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_18", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_19", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_20", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_21", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_22", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_23", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_24", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_25", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_26", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_27", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_28", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_29", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_30", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_31", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_32", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_33", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_34", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_35", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_36", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_37", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_38", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_39", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_40", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_41", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_42", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_43", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_44", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_45", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_46", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_47", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_48", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_49", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_50", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_51", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_52", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_53", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_54", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_55", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_56", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_57", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_58", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_59", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_60", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_61", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_62", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_63", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_64", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_65", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_66", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_67", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_68", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_69", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_70", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_71", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_72", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_73", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_74", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_75", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_76", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_77", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_78", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_79", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_80", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_81", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_82", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_83", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_84", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_85", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_86", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_87", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_88", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_89", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_90", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_91", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_92", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_93", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_94", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_95", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_96", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_97", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_98", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_99", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_100", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_101", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_102", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_103", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_104", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_105", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_106", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_107", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_108", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_109", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_110", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_111", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_112", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_113", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_114", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_115", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_116", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_117", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_118", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_119", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_120", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_121", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_122", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_123", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_124", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_125", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_126", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_127", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_128", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_129", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_130", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_131", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_132", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_133", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_134", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_135", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_136", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_137", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_138", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_139", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_140", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_141", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_142", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_143", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_144", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_145", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_146", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_147", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_148", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_149", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_150", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_151", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_152", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_153", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_154", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_155", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_156", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_157", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_158", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_159", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_160", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_161", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_162", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_163", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_164", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_165", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_166", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_167", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_168", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_169", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_170", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_171", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_172", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_173", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_174", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_175", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_176", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_177", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_178", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_179", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_180", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_181", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_182", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_183", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_184", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_185", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_186", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_187", note: "True if monospace" },
    TableField { table: "kern", field: "coverage_188", note: "Horizontal/vertical and format coverage bits" },
    TableField { table: "GDEF", field: "GlyphClassDef_189", note: "Offset to glyph class definition" },
    TableField { table: "GPOS", field: "LookupList_190", note: "Offset to GPOS lookup list" },
    TableField { table: "GSUB", field: "LookupList_191", note: "Offset to GSUB lookup list" },
    TableField { table: "head", field: "unitsPerEm_192", note: "Em square size in font units" },
    TableField { table: "head", field: "indexToLocFormat_193", note: "0 for short loca offsets, 1 for long" },
    TableField { table: "hhea", field: "numberOfHMetrics_194", note: "Count of long horizontal metric pairs" },
    TableField { table: "maxp", field: "numGlyphs_195", note: "Total glyphs including notdef" },
    TableField { table: "cmap", field: "platformID_196", note: "Platform identifier for encoding record" },
    TableField { table: "name", field: "nameID_197", note: "Logical name type (family, subfamily, full, etc.)" },
    TableField { table: "OS/2", field: "usWeightClass_198", note: "CSS font-weight compatible value" },
    TableField { table: "post", field: "isFixedPitch_199", note: "True if monospace" },
];

pub fn field_count() -> usize { FIELDS.len() }
pub fn lookup(table: &str, field: &str) -> Option<&'static str> {
    FIELDS.iter().find(|e| e.table == table && e.field == field).map(|e| e.note)
}

/* FIELD_EXT */
pub const EXT_FIELDS: &[TableField] = &[
    TableField { table: "head", field: "ext_0", note: "Extended OpenType field doc 0 for head" },
    TableField { table: "hhea", field: "ext_1", note: "Extended OpenType field doc 1 for hhea" },
    TableField { table: "maxp", field: "ext_2", note: "Extended OpenType field doc 2 for maxp" },
    TableField { table: "cmap", field: "ext_3", note: "Extended OpenType field doc 3 for cmap" },
    TableField { table: "glyf", field: "ext_4", note: "Extended OpenType field doc 4 for glyf" },
    TableField { table: "loca", field: "ext_5", note: "Extended OpenType field doc 5 for loca" },
    TableField { table: "hmtx", field: "ext_6", note: "Extended OpenType field doc 6 for hmtx" },
    TableField { table: "name", field: "ext_7", note: "Extended OpenType field doc 7 for name" },
    TableField { table: "post", field: "ext_8", note: "Extended OpenType field doc 8 for post" },
    TableField { table: "OS/2", field: "ext_9", note: "Extended OpenType field doc 9 for OS/2" },
    TableField { table: "kern", field: "ext_10", note: "Extended OpenType field doc 10 for kern" },
    TableField { table: "GDEF", field: "ext_11", note: "Extended OpenType field doc 11 for GDEF" },
    TableField { table: "GPOS", field: "ext_12", note: "Extended OpenType field doc 12 for GPOS" },
    TableField { table: "GSUB", field: "ext_13", note: "Extended OpenType field doc 13 for GSUB" },
    TableField { table: "head", field: "ext_14", note: "Extended OpenType field doc 14 for head" },
    TableField { table: "hhea", field: "ext_15", note: "Extended OpenType field doc 15 for hhea" },
    TableField { table: "maxp", field: "ext_16", note: "Extended OpenType field doc 16 for maxp" },
    TableField { table: "cmap", field: "ext_17", note: "Extended OpenType field doc 17 for cmap" },
    TableField { table: "glyf", field: "ext_18", note: "Extended OpenType field doc 18 for glyf" },
    TableField { table: "loca", field: "ext_19", note: "Extended OpenType field doc 19 for loca" },
    TableField { table: "hmtx", field: "ext_20", note: "Extended OpenType field doc 20 for hmtx" },
    TableField { table: "name", field: "ext_21", note: "Extended OpenType field doc 21 for name" },
    TableField { table: "post", field: "ext_22", note: "Extended OpenType field doc 22 for post" },
    TableField { table: "OS/2", field: "ext_23", note: "Extended OpenType field doc 23 for OS/2" },
    TableField { table: "kern", field: "ext_24", note: "Extended OpenType field doc 24 for kern" },
    TableField { table: "GDEF", field: "ext_25", note: "Extended OpenType field doc 25 for GDEF" },
    TableField { table: "GPOS", field: "ext_26", note: "Extended OpenType field doc 26 for GPOS" },
    TableField { table: "GSUB", field: "ext_27", note: "Extended OpenType field doc 27 for GSUB" },
    TableField { table: "head", field: "ext_28", note: "Extended OpenType field doc 28 for head" },
    TableField { table: "hhea", field: "ext_29", note: "Extended OpenType field doc 29 for hhea" },
    TableField { table: "maxp", field: "ext_30", note: "Extended OpenType field doc 30 for maxp" },
    TableField { table: "cmap", field: "ext_31", note: "Extended OpenType field doc 31 for cmap" },
    TableField { table: "glyf", field: "ext_32", note: "Extended OpenType field doc 32 for glyf" },
    TableField { table: "loca", field: "ext_33", note: "Extended OpenType field doc 33 for loca" },
    TableField { table: "hmtx", field: "ext_34", note: "Extended OpenType field doc 34 for hmtx" },
    TableField { table: "name", field: "ext_35", note: "Extended OpenType field doc 35 for name" },
    TableField { table: "post", field: "ext_36", note: "Extended OpenType field doc 36 for post" },
    TableField { table: "OS/2", field: "ext_37", note: "Extended OpenType field doc 37 for OS/2" },
    TableField { table: "kern", field: "ext_38", note: "Extended OpenType field doc 38 for kern" },
    TableField { table: "GDEF", field: "ext_39", note: "Extended OpenType field doc 39 for GDEF" },
    TableField { table: "GPOS", field: "ext_40", note: "Extended OpenType field doc 40 for GPOS" },
    TableField { table: "GSUB", field: "ext_41", note: "Extended OpenType field doc 41 for GSUB" },
    TableField { table: "head", field: "ext_42", note: "Extended OpenType field doc 42 for head" },
    TableField { table: "hhea", field: "ext_43", note: "Extended OpenType field doc 43 for hhea" },
    TableField { table: "maxp", field: "ext_44", note: "Extended OpenType field doc 44 for maxp" },
    TableField { table: "cmap", field: "ext_45", note: "Extended OpenType field doc 45 for cmap" },
    TableField { table: "glyf", field: "ext_46", note: "Extended OpenType field doc 46 for glyf" },
    TableField { table: "loca", field: "ext_47", note: "Extended OpenType field doc 47 for loca" },
    TableField { table: "hmtx", field: "ext_48", note: "Extended OpenType field doc 48 for hmtx" },
    TableField { table: "name", field: "ext_49", note: "Extended OpenType field doc 49 for name" },
    TableField { table: "post", field: "ext_50", note: "Extended OpenType field doc 50 for post" },
    TableField { table: "OS/2", field: "ext_51", note: "Extended OpenType field doc 51 for OS/2" },
    TableField { table: "kern", field: "ext_52", note: "Extended OpenType field doc 52 for kern" },
    TableField { table: "GDEF", field: "ext_53", note: "Extended OpenType field doc 53 for GDEF" },
    TableField { table: "GPOS", field: "ext_54", note: "Extended OpenType field doc 54 for GPOS" },
    TableField { table: "GSUB", field: "ext_55", note: "Extended OpenType field doc 55 for GSUB" },
    TableField { table: "head", field: "ext_56", note: "Extended OpenType field doc 56 for head" },
    TableField { table: "hhea", field: "ext_57", note: "Extended OpenType field doc 57 for hhea" },
    TableField { table: "maxp", field: "ext_58", note: "Extended OpenType field doc 58 for maxp" },
    TableField { table: "cmap", field: "ext_59", note: "Extended OpenType field doc 59 for cmap" },
    TableField { table: "glyf", field: "ext_60", note: "Extended OpenType field doc 60 for glyf" },
    TableField { table: "loca", field: "ext_61", note: "Extended OpenType field doc 61 for loca" },
    TableField { table: "hmtx", field: "ext_62", note: "Extended OpenType field doc 62 for hmtx" },
    TableField { table: "name", field: "ext_63", note: "Extended OpenType field doc 63 for name" },
    TableField { table: "post", field: "ext_64", note: "Extended OpenType field doc 64 for post" },
    TableField { table: "OS/2", field: "ext_65", note: "Extended OpenType field doc 65 for OS/2" },
    TableField { table: "kern", field: "ext_66", note: "Extended OpenType field doc 66 for kern" },
    TableField { table: "GDEF", field: "ext_67", note: "Extended OpenType field doc 67 for GDEF" },
    TableField { table: "GPOS", field: "ext_68", note: "Extended OpenType field doc 68 for GPOS" },
    TableField { table: "GSUB", field: "ext_69", note: "Extended OpenType field doc 69 for GSUB" },
    TableField { table: "head", field: "ext_70", note: "Extended OpenType field doc 70 for head" },
    TableField { table: "hhea", field: "ext_71", note: "Extended OpenType field doc 71 for hhea" },
    TableField { table: "maxp", field: "ext_72", note: "Extended OpenType field doc 72 for maxp" },
    TableField { table: "cmap", field: "ext_73", note: "Extended OpenType field doc 73 for cmap" },
    TableField { table: "glyf", field: "ext_74", note: "Extended OpenType field doc 74 for glyf" },
    TableField { table: "loca", field: "ext_75", note: "Extended OpenType field doc 75 for loca" },
    TableField { table: "hmtx", field: "ext_76", note: "Extended OpenType field doc 76 for hmtx" },
    TableField { table: "name", field: "ext_77", note: "Extended OpenType field doc 77 for name" },
    TableField { table: "post", field: "ext_78", note: "Extended OpenType field doc 78 for post" },
    TableField { table: "OS/2", field: "ext_79", note: "Extended OpenType field doc 79 for OS/2" },
    TableField { table: "kern", field: "ext_80", note: "Extended OpenType field doc 80 for kern" },
    TableField { table: "GDEF", field: "ext_81", note: "Extended OpenType field doc 81 for GDEF" },
    TableField { table: "GPOS", field: "ext_82", note: "Extended OpenType field doc 82 for GPOS" },
    TableField { table: "GSUB", field: "ext_83", note: "Extended OpenType field doc 83 for GSUB" },
    TableField { table: "head", field: "ext_84", note: "Extended OpenType field doc 84 for head" },
    TableField { table: "hhea", field: "ext_85", note: "Extended OpenType field doc 85 for hhea" },
    TableField { table: "maxp", field: "ext_86", note: "Extended OpenType field doc 86 for maxp" },
    TableField { table: "cmap", field: "ext_87", note: "Extended OpenType field doc 87 for cmap" },
    TableField { table: "glyf", field: "ext_88", note: "Extended OpenType field doc 88 for glyf" },
    TableField { table: "loca", field: "ext_89", note: "Extended OpenType field doc 89 for loca" },
    TableField { table: "hmtx", field: "ext_90", note: "Extended OpenType field doc 90 for hmtx" },
    TableField { table: "name", field: "ext_91", note: "Extended OpenType field doc 91 for name" },
    TableField { table: "post", field: "ext_92", note: "Extended OpenType field doc 92 for post" },
    TableField { table: "OS/2", field: "ext_93", note: "Extended OpenType field doc 93 for OS/2" },
    TableField { table: "kern", field: "ext_94", note: "Extended OpenType field doc 94 for kern" },
    TableField { table: "GDEF", field: "ext_95", note: "Extended OpenType field doc 95 for GDEF" },
    TableField { table: "GPOS", field: "ext_96", note: "Extended OpenType field doc 96 for GPOS" },
    TableField { table: "GSUB", field: "ext_97", note: "Extended OpenType field doc 97 for GSUB" },
    TableField { table: "head", field: "ext_98", note: "Extended OpenType field doc 98 for head" },
    TableField { table: "hhea", field: "ext_99", note: "Extended OpenType field doc 99 for hhea" },
    TableField { table: "maxp", field: "ext_100", note: "Extended OpenType field doc 100 for maxp" },
    TableField { table: "cmap", field: "ext_101", note: "Extended OpenType field doc 101 for cmap" },
    TableField { table: "glyf", field: "ext_102", note: "Extended OpenType field doc 102 for glyf" },
    TableField { table: "loca", field: "ext_103", note: "Extended OpenType field doc 103 for loca" },
    TableField { table: "hmtx", field: "ext_104", note: "Extended OpenType field doc 104 for hmtx" },
    TableField { table: "name", field: "ext_105", note: "Extended OpenType field doc 105 for name" },
    TableField { table: "post", field: "ext_106", note: "Extended OpenType field doc 106 for post" },
    TableField { table: "OS/2", field: "ext_107", note: "Extended OpenType field doc 107 for OS/2" },
    TableField { table: "kern", field: "ext_108", note: "Extended OpenType field doc 108 for kern" },
    TableField { table: "GDEF", field: "ext_109", note: "Extended OpenType field doc 109 for GDEF" },
    TableField { table: "GPOS", field: "ext_110", note: "Extended OpenType field doc 110 for GPOS" },
    TableField { table: "GSUB", field: "ext_111", note: "Extended OpenType field doc 111 for GSUB" },
    TableField { table: "head", field: "ext_112", note: "Extended OpenType field doc 112 for head" },
    TableField { table: "hhea", field: "ext_113", note: "Extended OpenType field doc 113 for hhea" },
    TableField { table: "maxp", field: "ext_114", note: "Extended OpenType field doc 114 for maxp" },
    TableField { table: "cmap", field: "ext_115", note: "Extended OpenType field doc 115 for cmap" },
    TableField { table: "glyf", field: "ext_116", note: "Extended OpenType field doc 116 for glyf" },
    TableField { table: "loca", field: "ext_117", note: "Extended OpenType field doc 117 for loca" },
    TableField { table: "hmtx", field: "ext_118", note: "Extended OpenType field doc 118 for hmtx" },
    TableField { table: "name", field: "ext_119", note: "Extended OpenType field doc 119 for name" },
    TableField { table: "post", field: "ext_120", note: "Extended OpenType field doc 120 for post" },
    TableField { table: "OS/2", field: "ext_121", note: "Extended OpenType field doc 121 for OS/2" },
    TableField { table: "kern", field: "ext_122", note: "Extended OpenType field doc 122 for kern" },
    TableField { table: "GDEF", field: "ext_123", note: "Extended OpenType field doc 123 for GDEF" },
    TableField { table: "GPOS", field: "ext_124", note: "Extended OpenType field doc 124 for GPOS" },
    TableField { table: "GSUB", field: "ext_125", note: "Extended OpenType field doc 125 for GSUB" },
    TableField { table: "head", field: "ext_126", note: "Extended OpenType field doc 126 for head" },
    TableField { table: "hhea", field: "ext_127", note: "Extended OpenType field doc 127 for hhea" },
    TableField { table: "maxp", field: "ext_128", note: "Extended OpenType field doc 128 for maxp" },
    TableField { table: "cmap", field: "ext_129", note: "Extended OpenType field doc 129 for cmap" },
    TableField { table: "glyf", field: "ext_130", note: "Extended OpenType field doc 130 for glyf" },
    TableField { table: "loca", field: "ext_131", note: "Extended OpenType field doc 131 for loca" },
    TableField { table: "hmtx", field: "ext_132", note: "Extended OpenType field doc 132 for hmtx" },
    TableField { table: "name", field: "ext_133", note: "Extended OpenType field doc 133 for name" },
    TableField { table: "post", field: "ext_134", note: "Extended OpenType field doc 134 for post" },
    TableField { table: "OS/2", field: "ext_135", note: "Extended OpenType field doc 135 for OS/2" },
    TableField { table: "kern", field: "ext_136", note: "Extended OpenType field doc 136 for kern" },
    TableField { table: "GDEF", field: "ext_137", note: "Extended OpenType field doc 137 for GDEF" },
    TableField { table: "GPOS", field: "ext_138", note: "Extended OpenType field doc 138 for GPOS" },
    TableField { table: "GSUB", field: "ext_139", note: "Extended OpenType field doc 139 for GSUB" },
    TableField { table: "head", field: "ext_140", note: "Extended OpenType field doc 140 for head" },
    TableField { table: "hhea", field: "ext_141", note: "Extended OpenType field doc 141 for hhea" },
    TableField { table: "maxp", field: "ext_142", note: "Extended OpenType field doc 142 for maxp" },
    TableField { table: "cmap", field: "ext_143", note: "Extended OpenType field doc 143 for cmap" },
    TableField { table: "glyf", field: "ext_144", note: "Extended OpenType field doc 144 for glyf" },
    TableField { table: "loca", field: "ext_145", note: "Extended OpenType field doc 145 for loca" },
    TableField { table: "hmtx", field: "ext_146", note: "Extended OpenType field doc 146 for hmtx" },
    TableField { table: "name", field: "ext_147", note: "Extended OpenType field doc 147 for name" },
    TableField { table: "post", field: "ext_148", note: "Extended OpenType field doc 148 for post" },
    TableField { table: "OS/2", field: "ext_149", note: "Extended OpenType field doc 149 for OS/2" },
    TableField { table: "kern", field: "ext_150", note: "Extended OpenType field doc 150 for kern" },
    TableField { table: "GDEF", field: "ext_151", note: "Extended OpenType field doc 151 for GDEF" },
    TableField { table: "GPOS", field: "ext_152", note: "Extended OpenType field doc 152 for GPOS" },
    TableField { table: "GSUB", field: "ext_153", note: "Extended OpenType field doc 153 for GSUB" },
    TableField { table: "head", field: "ext_154", note: "Extended OpenType field doc 154 for head" },
    TableField { table: "hhea", field: "ext_155", note: "Extended OpenType field doc 155 for hhea" },
    TableField { table: "maxp", field: "ext_156", note: "Extended OpenType field doc 156 for maxp" },
    TableField { table: "cmap", field: "ext_157", note: "Extended OpenType field doc 157 for cmap" },
    TableField { table: "glyf", field: "ext_158", note: "Extended OpenType field doc 158 for glyf" },
    TableField { table: "loca", field: "ext_159", note: "Extended OpenType field doc 159 for loca" },
    TableField { table: "hmtx", field: "ext_160", note: "Extended OpenType field doc 160 for hmtx" },
    TableField { table: "name", field: "ext_161", note: "Extended OpenType field doc 161 for name" },
    TableField { table: "post", field: "ext_162", note: "Extended OpenType field doc 162 for post" },
    TableField { table: "OS/2", field: "ext_163", note: "Extended OpenType field doc 163 for OS/2" },
    TableField { table: "kern", field: "ext_164", note: "Extended OpenType field doc 164 for kern" },
    TableField { table: "GDEF", field: "ext_165", note: "Extended OpenType field doc 165 for GDEF" },
    TableField { table: "GPOS", field: "ext_166", note: "Extended OpenType field doc 166 for GPOS" },
    TableField { table: "GSUB", field: "ext_167", note: "Extended OpenType field doc 167 for GSUB" },
    TableField { table: "head", field: "ext_168", note: "Extended OpenType field doc 168 for head" },
    TableField { table: "hhea", field: "ext_169", note: "Extended OpenType field doc 169 for hhea" },
    TableField { table: "maxp", field: "ext_170", note: "Extended OpenType field doc 170 for maxp" },
    TableField { table: "cmap", field: "ext_171", note: "Extended OpenType field doc 171 for cmap" },
    TableField { table: "glyf", field: "ext_172", note: "Extended OpenType field doc 172 for glyf" },
    TableField { table: "loca", field: "ext_173", note: "Extended OpenType field doc 173 for loca" },
    TableField { table: "hmtx", field: "ext_174", note: "Extended OpenType field doc 174 for hmtx" },
    TableField { table: "name", field: "ext_175", note: "Extended OpenType field doc 175 for name" },
    TableField { table: "post", field: "ext_176", note: "Extended OpenType field doc 176 for post" },
    TableField { table: "OS/2", field: "ext_177", note: "Extended OpenType field doc 177 for OS/2" },
    TableField { table: "kern", field: "ext_178", note: "Extended OpenType field doc 178 for kern" },
    TableField { table: "GDEF", field: "ext_179", note: "Extended OpenType field doc 179 for GDEF" },
    TableField { table: "GPOS", field: "ext_180", note: "Extended OpenType field doc 180 for GPOS" },
    TableField { table: "GSUB", field: "ext_181", note: "Extended OpenType field doc 181 for GSUB" },
    TableField { table: "head", field: "ext_182", note: "Extended OpenType field doc 182 for head" },
    TableField { table: "hhea", field: "ext_183", note: "Extended OpenType field doc 183 for hhea" },
    TableField { table: "maxp", field: "ext_184", note: "Extended OpenType field doc 184 for maxp" },
    TableField { table: "cmap", field: "ext_185", note: "Extended OpenType field doc 185 for cmap" },
    TableField { table: "glyf", field: "ext_186", note: "Extended OpenType field doc 186 for glyf" },
    TableField { table: "loca", field: "ext_187", note: "Extended OpenType field doc 187 for loca" },
    TableField { table: "hmtx", field: "ext_188", note: "Extended OpenType field doc 188 for hmtx" },
    TableField { table: "name", field: "ext_189", note: "Extended OpenType field doc 189 for name" },
    TableField { table: "post", field: "ext_190", note: "Extended OpenType field doc 190 for post" },
    TableField { table: "OS/2", field: "ext_191", note: "Extended OpenType field doc 191 for OS/2" },
    TableField { table: "kern", field: "ext_192", note: "Extended OpenType field doc 192 for kern" },
    TableField { table: "GDEF", field: "ext_193", note: "Extended OpenType field doc 193 for GDEF" },
    TableField { table: "GPOS", field: "ext_194", note: "Extended OpenType field doc 194 for GPOS" },
    TableField { table: "GSUB", field: "ext_195", note: "Extended OpenType field doc 195 for GSUB" },
    TableField { table: "head", field: "ext_196", note: "Extended OpenType field doc 196 for head" },
    TableField { table: "hhea", field: "ext_197", note: "Extended OpenType field doc 197 for hhea" },
    TableField { table: "maxp", field: "ext_198", note: "Extended OpenType field doc 198 for maxp" },
    TableField { table: "cmap", field: "ext_199", note: "Extended OpenType field doc 199 for cmap" },
    TableField { table: "glyf", field: "ext_200", note: "Extended OpenType field doc 200 for glyf" },
    TableField { table: "loca", field: "ext_201", note: "Extended OpenType field doc 201 for loca" },
    TableField { table: "hmtx", field: "ext_202", note: "Extended OpenType field doc 202 for hmtx" },
    TableField { table: "name", field: "ext_203", note: "Extended OpenType field doc 203 for name" },
    TableField { table: "post", field: "ext_204", note: "Extended OpenType field doc 204 for post" },
    TableField { table: "OS/2", field: "ext_205", note: "Extended OpenType field doc 205 for OS/2" },
    TableField { table: "kern", field: "ext_206", note: "Extended OpenType field doc 206 for kern" },
    TableField { table: "GDEF", field: "ext_207", note: "Extended OpenType field doc 207 for GDEF" },
    TableField { table: "GPOS", field: "ext_208", note: "Extended OpenType field doc 208 for GPOS" },
    TableField { table: "GSUB", field: "ext_209", note: "Extended OpenType field doc 209 for GSUB" },
    TableField { table: "head", field: "ext_210", note: "Extended OpenType field doc 210 for head" },
    TableField { table: "hhea", field: "ext_211", note: "Extended OpenType field doc 211 for hhea" },
    TableField { table: "maxp", field: "ext_212", note: "Extended OpenType field doc 212 for maxp" },
    TableField { table: "cmap", field: "ext_213", note: "Extended OpenType field doc 213 for cmap" },
    TableField { table: "glyf", field: "ext_214", note: "Extended OpenType field doc 214 for glyf" },
    TableField { table: "loca", field: "ext_215", note: "Extended OpenType field doc 215 for loca" },
    TableField { table: "hmtx", field: "ext_216", note: "Extended OpenType field doc 216 for hmtx" },
    TableField { table: "name", field: "ext_217", note: "Extended OpenType field doc 217 for name" },
    TableField { table: "post", field: "ext_218", note: "Extended OpenType field doc 218 for post" },
    TableField { table: "OS/2", field: "ext_219", note: "Extended OpenType field doc 219 for OS/2" },
    TableField { table: "kern", field: "ext_220", note: "Extended OpenType field doc 220 for kern" },
    TableField { table: "GDEF", field: "ext_221", note: "Extended OpenType field doc 221 for GDEF" },
    TableField { table: "GPOS", field: "ext_222", note: "Extended OpenType field doc 222 for GPOS" },
    TableField { table: "GSUB", field: "ext_223", note: "Extended OpenType field doc 223 for GSUB" },
    TableField { table: "head", field: "ext_224", note: "Extended OpenType field doc 224 for head" },
    TableField { table: "hhea", field: "ext_225", note: "Extended OpenType field doc 225 for hhea" },
    TableField { table: "maxp", field: "ext_226", note: "Extended OpenType field doc 226 for maxp" },
    TableField { table: "cmap", field: "ext_227", note: "Extended OpenType field doc 227 for cmap" },
    TableField { table: "glyf", field: "ext_228", note: "Extended OpenType field doc 228 for glyf" },
    TableField { table: "loca", field: "ext_229", note: "Extended OpenType field doc 229 for loca" },
    TableField { table: "hmtx", field: "ext_230", note: "Extended OpenType field doc 230 for hmtx" },
    TableField { table: "name", field: "ext_231", note: "Extended OpenType field doc 231 for name" },
    TableField { table: "post", field: "ext_232", note: "Extended OpenType field doc 232 for post" },
    TableField { table: "OS/2", field: "ext_233", note: "Extended OpenType field doc 233 for OS/2" },
    TableField { table: "kern", field: "ext_234", note: "Extended OpenType field doc 234 for kern" },
    TableField { table: "GDEF", field: "ext_235", note: "Extended OpenType field doc 235 for GDEF" },
    TableField { table: "GPOS", field: "ext_236", note: "Extended OpenType field doc 236 for GPOS" },
    TableField { table: "GSUB", field: "ext_237", note: "Extended OpenType field doc 237 for GSUB" },
    TableField { table: "head", field: "ext_238", note: "Extended OpenType field doc 238 for head" },
    TableField { table: "hhea", field: "ext_239", note: "Extended OpenType field doc 239 for hhea" },
    TableField { table: "maxp", field: "ext_240", note: "Extended OpenType field doc 240 for maxp" },
    TableField { table: "cmap", field: "ext_241", note: "Extended OpenType field doc 241 for cmap" },
    TableField { table: "glyf", field: "ext_242", note: "Extended OpenType field doc 242 for glyf" },
    TableField { table: "loca", field: "ext_243", note: "Extended OpenType field doc 243 for loca" },
    TableField { table: "hmtx", field: "ext_244", note: "Extended OpenType field doc 244 for hmtx" },
    TableField { table: "name", field: "ext_245", note: "Extended OpenType field doc 245 for name" },
    TableField { table: "post", field: "ext_246", note: "Extended OpenType field doc 246 for post" },
    TableField { table: "OS/2", field: "ext_247", note: "Extended OpenType field doc 247 for OS/2" },
    TableField { table: "kern", field: "ext_248", note: "Extended OpenType field doc 248 for kern" },
    TableField { table: "GDEF", field: "ext_249", note: "Extended OpenType field doc 249 for GDEF" },
    TableField { table: "GPOS", field: "ext_250", note: "Extended OpenType field doc 250 for GPOS" },
    TableField { table: "GSUB", field: "ext_251", note: "Extended OpenType field doc 251 for GSUB" },
    TableField { table: "head", field: "ext_252", note: "Extended OpenType field doc 252 for head" },
    TableField { table: "hhea", field: "ext_253", note: "Extended OpenType field doc 253 for hhea" },
    TableField { table: "maxp", field: "ext_254", note: "Extended OpenType field doc 254 for maxp" },
    TableField { table: "cmap", field: "ext_255", note: "Extended OpenType field doc 255 for cmap" },
    TableField { table: "glyf", field: "ext_256", note: "Extended OpenType field doc 256 for glyf" },
    TableField { table: "loca", field: "ext_257", note: "Extended OpenType field doc 257 for loca" },
    TableField { table: "hmtx", field: "ext_258", note: "Extended OpenType field doc 258 for hmtx" },
    TableField { table: "name", field: "ext_259", note: "Extended OpenType field doc 259 for name" },
    TableField { table: "post", field: "ext_260", note: "Extended OpenType field doc 260 for post" },
    TableField { table: "OS/2", field: "ext_261", note: "Extended OpenType field doc 261 for OS/2" },
    TableField { table: "kern", field: "ext_262", note: "Extended OpenType field doc 262 for kern" },
    TableField { table: "GDEF", field: "ext_263", note: "Extended OpenType field doc 263 for GDEF" },
    TableField { table: "GPOS", field: "ext_264", note: "Extended OpenType field doc 264 for GPOS" },
    TableField { table: "GSUB", field: "ext_265", note: "Extended OpenType field doc 265 for GSUB" },
    TableField { table: "head", field: "ext_266", note: "Extended OpenType field doc 266 for head" },
    TableField { table: "hhea", field: "ext_267", note: "Extended OpenType field doc 267 for hhea" },
    TableField { table: "maxp", field: "ext_268", note: "Extended OpenType field doc 268 for maxp" },
    TableField { table: "cmap", field: "ext_269", note: "Extended OpenType field doc 269 for cmap" },
    TableField { table: "glyf", field: "ext_270", note: "Extended OpenType field doc 270 for glyf" },
    TableField { table: "loca", field: "ext_271", note: "Extended OpenType field doc 271 for loca" },
    TableField { table: "hmtx", field: "ext_272", note: "Extended OpenType field doc 272 for hmtx" },
    TableField { table: "name", field: "ext_273", note: "Extended OpenType field doc 273 for name" },
    TableField { table: "post", field: "ext_274", note: "Extended OpenType field doc 274 for post" },
    TableField { table: "OS/2", field: "ext_275", note: "Extended OpenType field doc 275 for OS/2" },
    TableField { table: "kern", field: "ext_276", note: "Extended OpenType field doc 276 for kern" },
    TableField { table: "GDEF", field: "ext_277", note: "Extended OpenType field doc 277 for GDEF" },
    TableField { table: "GPOS", field: "ext_278", note: "Extended OpenType field doc 278 for GPOS" },
    TableField { table: "GSUB", field: "ext_279", note: "Extended OpenType field doc 279 for GSUB" },
    TableField { table: "head", field: "ext_280", note: "Extended OpenType field doc 280 for head" },
    TableField { table: "hhea", field: "ext_281", note: "Extended OpenType field doc 281 for hhea" },
    TableField { table: "maxp", field: "ext_282", note: "Extended OpenType field doc 282 for maxp" },
    TableField { table: "cmap", field: "ext_283", note: "Extended OpenType field doc 283 for cmap" },
    TableField { table: "glyf", field: "ext_284", note: "Extended OpenType field doc 284 for glyf" },
    TableField { table: "loca", field: "ext_285", note: "Extended OpenType field doc 285 for loca" },
    TableField { table: "hmtx", field: "ext_286", note: "Extended OpenType field doc 286 for hmtx" },
    TableField { table: "name", field: "ext_287", note: "Extended OpenType field doc 287 for name" },
    TableField { table: "post", field: "ext_288", note: "Extended OpenType field doc 288 for post" },
    TableField { table: "OS/2", field: "ext_289", note: "Extended OpenType field doc 289 for OS/2" },
    TableField { table: "kern", field: "ext_290", note: "Extended OpenType field doc 290 for kern" },
    TableField { table: "GDEF", field: "ext_291", note: "Extended OpenType field doc 291 for GDEF" },
    TableField { table: "GPOS", field: "ext_292", note: "Extended OpenType field doc 292 for GPOS" },
    TableField { table: "GSUB", field: "ext_293", note: "Extended OpenType field doc 293 for GSUB" },
    TableField { table: "head", field: "ext_294", note: "Extended OpenType field doc 294 for head" },
    TableField { table: "hhea", field: "ext_295", note: "Extended OpenType field doc 295 for hhea" },
    TableField { table: "maxp", field: "ext_296", note: "Extended OpenType field doc 296 for maxp" },
    TableField { table: "cmap", field: "ext_297", note: "Extended OpenType field doc 297 for cmap" },
    TableField { table: "glyf", field: "ext_298", note: "Extended OpenType field doc 298 for glyf" },
    TableField { table: "loca", field: "ext_299", note: "Extended OpenType field doc 299 for loca" },
    TableField { table: "hmtx", field: "ext_300", note: "Extended OpenType field doc 300 for hmtx" },
    TableField { table: "name", field: "ext_301", note: "Extended OpenType field doc 301 for name" },
    TableField { table: "post", field: "ext_302", note: "Extended OpenType field doc 302 for post" },
    TableField { table: "OS/2", field: "ext_303", note: "Extended OpenType field doc 303 for OS/2" },
    TableField { table: "kern", field: "ext_304", note: "Extended OpenType field doc 304 for kern" },
    TableField { table: "GDEF", field: "ext_305", note: "Extended OpenType field doc 305 for GDEF" },
    TableField { table: "GPOS", field: "ext_306", note: "Extended OpenType field doc 306 for GPOS" },
    TableField { table: "GSUB", field: "ext_307", note: "Extended OpenType field doc 307 for GSUB" },
    TableField { table: "head", field: "ext_308", note: "Extended OpenType field doc 308 for head" },
    TableField { table: "hhea", field: "ext_309", note: "Extended OpenType field doc 309 for hhea" },
    TableField { table: "maxp", field: "ext_310", note: "Extended OpenType field doc 310 for maxp" },
    TableField { table: "cmap", field: "ext_311", note: "Extended OpenType field doc 311 for cmap" },
    TableField { table: "glyf", field: "ext_312", note: "Extended OpenType field doc 312 for glyf" },
    TableField { table: "loca", field: "ext_313", note: "Extended OpenType field doc 313 for loca" },
    TableField { table: "hmtx", field: "ext_314", note: "Extended OpenType field doc 314 for hmtx" },
    TableField { table: "name", field: "ext_315", note: "Extended OpenType field doc 315 for name" },
    TableField { table: "post", field: "ext_316", note: "Extended OpenType field doc 316 for post" },
    TableField { table: "OS/2", field: "ext_317", note: "Extended OpenType field doc 317 for OS/2" },
    TableField { table: "kern", field: "ext_318", note: "Extended OpenType field doc 318 for kern" },
    TableField { table: "GDEF", field: "ext_319", note: "Extended OpenType field doc 319 for GDEF" },
    TableField { table: "GPOS", field: "ext_320", note: "Extended OpenType field doc 320 for GPOS" },
    TableField { table: "GSUB", field: "ext_321", note: "Extended OpenType field doc 321 for GSUB" },
    TableField { table: "head", field: "ext_322", note: "Extended OpenType field doc 322 for head" },
    TableField { table: "hhea", field: "ext_323", note: "Extended OpenType field doc 323 for hhea" },
    TableField { table: "maxp", field: "ext_324", note: "Extended OpenType field doc 324 for maxp" },
    TableField { table: "cmap", field: "ext_325", note: "Extended OpenType field doc 325 for cmap" },
    TableField { table: "glyf", field: "ext_326", note: "Extended OpenType field doc 326 for glyf" },
    TableField { table: "loca", field: "ext_327", note: "Extended OpenType field doc 327 for loca" },
    TableField { table: "hmtx", field: "ext_328", note: "Extended OpenType field doc 328 for hmtx" },
    TableField { table: "name", field: "ext_329", note: "Extended OpenType field doc 329 for name" },
    TableField { table: "post", field: "ext_330", note: "Extended OpenType field doc 330 for post" },
    TableField { table: "OS/2", field: "ext_331", note: "Extended OpenType field doc 331 for OS/2" },
    TableField { table: "kern", field: "ext_332", note: "Extended OpenType field doc 332 for kern" },
    TableField { table: "GDEF", field: "ext_333", note: "Extended OpenType field doc 333 for GDEF" },
    TableField { table: "GPOS", field: "ext_334", note: "Extended OpenType field doc 334 for GPOS" },
    TableField { table: "GSUB", field: "ext_335", note: "Extended OpenType field doc 335 for GSUB" },
    TableField { table: "head", field: "ext_336", note: "Extended OpenType field doc 336 for head" },
    TableField { table: "hhea", field: "ext_337", note: "Extended OpenType field doc 337 for hhea" },
    TableField { table: "maxp", field: "ext_338", note: "Extended OpenType field doc 338 for maxp" },
    TableField { table: "cmap", field: "ext_339", note: "Extended OpenType field doc 339 for cmap" },
    TableField { table: "glyf", field: "ext_340", note: "Extended OpenType field doc 340 for glyf" },
    TableField { table: "loca", field: "ext_341", note: "Extended OpenType field doc 341 for loca" },
    TableField { table: "hmtx", field: "ext_342", note: "Extended OpenType field doc 342 for hmtx" },
    TableField { table: "name", field: "ext_343", note: "Extended OpenType field doc 343 for name" },
    TableField { table: "post", field: "ext_344", note: "Extended OpenType field doc 344 for post" },
    TableField { table: "OS/2", field: "ext_345", note: "Extended OpenType field doc 345 for OS/2" },
    TableField { table: "kern", field: "ext_346", note: "Extended OpenType field doc 346 for kern" },
    TableField { table: "GDEF", field: "ext_347", note: "Extended OpenType field doc 347 for GDEF" },
    TableField { table: "GPOS", field: "ext_348", note: "Extended OpenType field doc 348 for GPOS" },
    TableField { table: "GSUB", field: "ext_349", note: "Extended OpenType field doc 349 for GSUB" },
    TableField { table: "head", field: "ext_350", note: "Extended OpenType field doc 350 for head" },
    TableField { table: "hhea", field: "ext_351", note: "Extended OpenType field doc 351 for hhea" },
    TableField { table: "maxp", field: "ext_352", note: "Extended OpenType field doc 352 for maxp" },
    TableField { table: "cmap", field: "ext_353", note: "Extended OpenType field doc 353 for cmap" },
    TableField { table: "glyf", field: "ext_354", note: "Extended OpenType field doc 354 for glyf" },
    TableField { table: "loca", field: "ext_355", note: "Extended OpenType field doc 355 for loca" },
    TableField { table: "hmtx", field: "ext_356", note: "Extended OpenType field doc 356 for hmtx" },
    TableField { table: "name", field: "ext_357", note: "Extended OpenType field doc 357 for name" },
    TableField { table: "post", field: "ext_358", note: "Extended OpenType field doc 358 for post" },
    TableField { table: "OS/2", field: "ext_359", note: "Extended OpenType field doc 359 for OS/2" },
    TableField { table: "kern", field: "ext_360", note: "Extended OpenType field doc 360 for kern" },
    TableField { table: "GDEF", field: "ext_361", note: "Extended OpenType field doc 361 for GDEF" },
    TableField { table: "GPOS", field: "ext_362", note: "Extended OpenType field doc 362 for GPOS" },
    TableField { table: "GSUB", field: "ext_363", note: "Extended OpenType field doc 363 for GSUB" },
    TableField { table: "head", field: "ext_364", note: "Extended OpenType field doc 364 for head" },
    TableField { table: "hhea", field: "ext_365", note: "Extended OpenType field doc 365 for hhea" },
    TableField { table: "maxp", field: "ext_366", note: "Extended OpenType field doc 366 for maxp" },
    TableField { table: "cmap", field: "ext_367", note: "Extended OpenType field doc 367 for cmap" },
    TableField { table: "glyf", field: "ext_368", note: "Extended OpenType field doc 368 for glyf" },
    TableField { table: "loca", field: "ext_369", note: "Extended OpenType field doc 369 for loca" },
    TableField { table: "hmtx", field: "ext_370", note: "Extended OpenType field doc 370 for hmtx" },
    TableField { table: "name", field: "ext_371", note: "Extended OpenType field doc 371 for name" },
    TableField { table: "post", field: "ext_372", note: "Extended OpenType field doc 372 for post" },
    TableField { table: "OS/2", field: "ext_373", note: "Extended OpenType field doc 373 for OS/2" },
    TableField { table: "kern", field: "ext_374", note: "Extended OpenType field doc 374 for kern" },
    TableField { table: "GDEF", field: "ext_375", note: "Extended OpenType field doc 375 for GDEF" },
    TableField { table: "GPOS", field: "ext_376", note: "Extended OpenType field doc 376 for GPOS" },
    TableField { table: "GSUB", field: "ext_377", note: "Extended OpenType field doc 377 for GSUB" },
    TableField { table: "head", field: "ext_378", note: "Extended OpenType field doc 378 for head" },
    TableField { table: "hhea", field: "ext_379", note: "Extended OpenType field doc 379 for hhea" },
    TableField { table: "maxp", field: "ext_380", note: "Extended OpenType field doc 380 for maxp" },
    TableField { table: "cmap", field: "ext_381", note: "Extended OpenType field doc 381 for cmap" },
    TableField { table: "glyf", field: "ext_382", note: "Extended OpenType field doc 382 for glyf" },
    TableField { table: "loca", field: "ext_383", note: "Extended OpenType field doc 383 for loca" },
    TableField { table: "hmtx", field: "ext_384", note: "Extended OpenType field doc 384 for hmtx" },
    TableField { table: "name", field: "ext_385", note: "Extended OpenType field doc 385 for name" },
    TableField { table: "post", field: "ext_386", note: "Extended OpenType field doc 386 for post" },
    TableField { table: "OS/2", field: "ext_387", note: "Extended OpenType field doc 387 for OS/2" },
    TableField { table: "kern", field: "ext_388", note: "Extended OpenType field doc 388 for kern" },
    TableField { table: "GDEF", field: "ext_389", note: "Extended OpenType field doc 389 for GDEF" },
    TableField { table: "GPOS", field: "ext_390", note: "Extended OpenType field doc 390 for GPOS" },
    TableField { table: "GSUB", field: "ext_391", note: "Extended OpenType field doc 391 for GSUB" },
    TableField { table: "head", field: "ext_392", note: "Extended OpenType field doc 392 for head" },
    TableField { table: "hhea", field: "ext_393", note: "Extended OpenType field doc 393 for hhea" },
    TableField { table: "maxp", field: "ext_394", note: "Extended OpenType field doc 394 for maxp" },
    TableField { table: "cmap", field: "ext_395", note: "Extended OpenType field doc 395 for cmap" },
    TableField { table: "glyf", field: "ext_396", note: "Extended OpenType field doc 396 for glyf" },
    TableField { table: "loca", field: "ext_397", note: "Extended OpenType field doc 397 for loca" },
    TableField { table: "hmtx", field: "ext_398", note: "Extended OpenType field doc 398 for hmtx" },
    TableField { table: "name", field: "ext_399", note: "Extended OpenType field doc 399 for name" },
    TableField { table: "post", field: "ext_400", note: "Extended OpenType field doc 400 for post" },
    TableField { table: "OS/2", field: "ext_401", note: "Extended OpenType field doc 401 for OS/2" },
    TableField { table: "kern", field: "ext_402", note: "Extended OpenType field doc 402 for kern" },
    TableField { table: "GDEF", field: "ext_403", note: "Extended OpenType field doc 403 for GDEF" },
    TableField { table: "GPOS", field: "ext_404", note: "Extended OpenType field doc 404 for GPOS" },
    TableField { table: "GSUB", field: "ext_405", note: "Extended OpenType field doc 405 for GSUB" },
    TableField { table: "head", field: "ext_406", note: "Extended OpenType field doc 406 for head" },
    TableField { table: "hhea", field: "ext_407", note: "Extended OpenType field doc 407 for hhea" },
    TableField { table: "maxp", field: "ext_408", note: "Extended OpenType field doc 408 for maxp" },
    TableField { table: "cmap", field: "ext_409", note: "Extended OpenType field doc 409 for cmap" },
    TableField { table: "glyf", field: "ext_410", note: "Extended OpenType field doc 410 for glyf" },
    TableField { table: "loca", field: "ext_411", note: "Extended OpenType field doc 411 for loca" },
    TableField { table: "hmtx", field: "ext_412", note: "Extended OpenType field doc 412 for hmtx" },
    TableField { table: "name", field: "ext_413", note: "Extended OpenType field doc 413 for name" },
    TableField { table: "post", field: "ext_414", note: "Extended OpenType field doc 414 for post" },
    TableField { table: "OS/2", field: "ext_415", note: "Extended OpenType field doc 415 for OS/2" },
    TableField { table: "kern", field: "ext_416", note: "Extended OpenType field doc 416 for kern" },
    TableField { table: "GDEF", field: "ext_417", note: "Extended OpenType field doc 417 for GDEF" },
    TableField { table: "GPOS", field: "ext_418", note: "Extended OpenType field doc 418 for GPOS" },
    TableField { table: "GSUB", field: "ext_419", note: "Extended OpenType field doc 419 for GSUB" },
    TableField { table: "head", field: "ext_420", note: "Extended OpenType field doc 420 for head" },
    TableField { table: "hhea", field: "ext_421", note: "Extended OpenType field doc 421 for hhea" },
    TableField { table: "maxp", field: "ext_422", note: "Extended OpenType field doc 422 for maxp" },
    TableField { table: "cmap", field: "ext_423", note: "Extended OpenType field doc 423 for cmap" },
    TableField { table: "glyf", field: "ext_424", note: "Extended OpenType field doc 424 for glyf" },
    TableField { table: "loca", field: "ext_425", note: "Extended OpenType field doc 425 for loca" },
    TableField { table: "hmtx", field: "ext_426", note: "Extended OpenType field doc 426 for hmtx" },
    TableField { table: "name", field: "ext_427", note: "Extended OpenType field doc 427 for name" },
    TableField { table: "post", field: "ext_428", note: "Extended OpenType field doc 428 for post" },
    TableField { table: "OS/2", field: "ext_429", note: "Extended OpenType field doc 429 for OS/2" },
    TableField { table: "kern", field: "ext_430", note: "Extended OpenType field doc 430 for kern" },
    TableField { table: "GDEF", field: "ext_431", note: "Extended OpenType field doc 431 for GDEF" },
    TableField { table: "GPOS", field: "ext_432", note: "Extended OpenType field doc 432 for GPOS" },
    TableField { table: "GSUB", field: "ext_433", note: "Extended OpenType field doc 433 for GSUB" },
    TableField { table: "head", field: "ext_434", note: "Extended OpenType field doc 434 for head" },
    TableField { table: "hhea", field: "ext_435", note: "Extended OpenType field doc 435 for hhea" },
    TableField { table: "maxp", field: "ext_436", note: "Extended OpenType field doc 436 for maxp" },
    TableField { table: "cmap", field: "ext_437", note: "Extended OpenType field doc 437 for cmap" },
    TableField { table: "glyf", field: "ext_438", note: "Extended OpenType field doc 438 for glyf" },
    TableField { table: "loca", field: "ext_439", note: "Extended OpenType field doc 439 for loca" },
    TableField { table: "hmtx", field: "ext_440", note: "Extended OpenType field doc 440 for hmtx" },
    TableField { table: "name", field: "ext_441", note: "Extended OpenType field doc 441 for name" },
    TableField { table: "post", field: "ext_442", note: "Extended OpenType field doc 442 for post" },
    TableField { table: "OS/2", field: "ext_443", note: "Extended OpenType field doc 443 for OS/2" },
    TableField { table: "kern", field: "ext_444", note: "Extended OpenType field doc 444 for kern" },
    TableField { table: "GDEF", field: "ext_445", note: "Extended OpenType field doc 445 for GDEF" },
    TableField { table: "GPOS", field: "ext_446", note: "Extended OpenType field doc 446 for GPOS" },
    TableField { table: "GSUB", field: "ext_447", note: "Extended OpenType field doc 447 for GSUB" },
    TableField { table: "head", field: "ext_448", note: "Extended OpenType field doc 448 for head" },
    TableField { table: "hhea", field: "ext_449", note: "Extended OpenType field doc 449 for hhea" },
    TableField { table: "maxp", field: "ext_450", note: "Extended OpenType field doc 450 for maxp" },
    TableField { table: "cmap", field: "ext_451", note: "Extended OpenType field doc 451 for cmap" },
    TableField { table: "glyf", field: "ext_452", note: "Extended OpenType field doc 452 for glyf" },
    TableField { table: "loca", field: "ext_453", note: "Extended OpenType field doc 453 for loca" },
    TableField { table: "hmtx", field: "ext_454", note: "Extended OpenType field doc 454 for hmtx" },
    TableField { table: "name", field: "ext_455", note: "Extended OpenType field doc 455 for name" },
    TableField { table: "post", field: "ext_456", note: "Extended OpenType field doc 456 for post" },
    TableField { table: "OS/2", field: "ext_457", note: "Extended OpenType field doc 457 for OS/2" },
    TableField { table: "kern", field: "ext_458", note: "Extended OpenType field doc 458 for kern" },
    TableField { table: "GDEF", field: "ext_459", note: "Extended OpenType field doc 459 for GDEF" },
    TableField { table: "GPOS", field: "ext_460", note: "Extended OpenType field doc 460 for GPOS" },
    TableField { table: "GSUB", field: "ext_461", note: "Extended OpenType field doc 461 for GSUB" },
    TableField { table: "head", field: "ext_462", note: "Extended OpenType field doc 462 for head" },
    TableField { table: "hhea", field: "ext_463", note: "Extended OpenType field doc 463 for hhea" },
    TableField { table: "maxp", field: "ext_464", note: "Extended OpenType field doc 464 for maxp" },
    TableField { table: "cmap", field: "ext_465", note: "Extended OpenType field doc 465 for cmap" },
    TableField { table: "glyf", field: "ext_466", note: "Extended OpenType field doc 466 for glyf" },
    TableField { table: "loca", field: "ext_467", note: "Extended OpenType field doc 467 for loca" },
    TableField { table: "hmtx", field: "ext_468", note: "Extended OpenType field doc 468 for hmtx" },
    TableField { table: "name", field: "ext_469", note: "Extended OpenType field doc 469 for name" },
    TableField { table: "post", field: "ext_470", note: "Extended OpenType field doc 470 for post" },
    TableField { table: "OS/2", field: "ext_471", note: "Extended OpenType field doc 471 for OS/2" },
    TableField { table: "kern", field: "ext_472", note: "Extended OpenType field doc 472 for kern" },
    TableField { table: "GDEF", field: "ext_473", note: "Extended OpenType field doc 473 for GDEF" },
    TableField { table: "GPOS", field: "ext_474", note: "Extended OpenType field doc 474 for GPOS" },
    TableField { table: "GSUB", field: "ext_475", note: "Extended OpenType field doc 475 for GSUB" },
    TableField { table: "head", field: "ext_476", note: "Extended OpenType field doc 476 for head" },
    TableField { table: "hhea", field: "ext_477", note: "Extended OpenType field doc 477 for hhea" },
    TableField { table: "maxp", field: "ext_478", note: "Extended OpenType field doc 478 for maxp" },
    TableField { table: "cmap", field: "ext_479", note: "Extended OpenType field doc 479 for cmap" },
    TableField { table: "glyf", field: "ext_480", note: "Extended OpenType field doc 480 for glyf" },
    TableField { table: "loca", field: "ext_481", note: "Extended OpenType field doc 481 for loca" },
    TableField { table: "hmtx", field: "ext_482", note: "Extended OpenType field doc 482 for hmtx" },
    TableField { table: "name", field: "ext_483", note: "Extended OpenType field doc 483 for name" },
    TableField { table: "post", field: "ext_484", note: "Extended OpenType field doc 484 for post" },
    TableField { table: "OS/2", field: "ext_485", note: "Extended OpenType field doc 485 for OS/2" },
    TableField { table: "kern", field: "ext_486", note: "Extended OpenType field doc 486 for kern" },
    TableField { table: "GDEF", field: "ext_487", note: "Extended OpenType field doc 487 for GDEF" },
    TableField { table: "GPOS", field: "ext_488", note: "Extended OpenType field doc 488 for GPOS" },
    TableField { table: "GSUB", field: "ext_489", note: "Extended OpenType field doc 489 for GSUB" },
    TableField { table: "head", field: "ext_490", note: "Extended OpenType field doc 490 for head" },
    TableField { table: "hhea", field: "ext_491", note: "Extended OpenType field doc 491 for hhea" },
    TableField { table: "maxp", field: "ext_492", note: "Extended OpenType field doc 492 for maxp" },
    TableField { table: "cmap", field: "ext_493", note: "Extended OpenType field doc 493 for cmap" },
    TableField { table: "glyf", field: "ext_494", note: "Extended OpenType field doc 494 for glyf" },
    TableField { table: "loca", field: "ext_495", note: "Extended OpenType field doc 495 for loca" },
    TableField { table: "hmtx", field: "ext_496", note: "Extended OpenType field doc 496 for hmtx" },
    TableField { table: "name", field: "ext_497", note: "Extended OpenType field doc 497 for name" },
    TableField { table: "post", field: "ext_498", note: "Extended OpenType field doc 498 for post" },
    TableField { table: "OS/2", field: "ext_499", note: "Extended OpenType field doc 499 for OS/2" },
    TableField { table: "kern", field: "ext_500", note: "Extended OpenType field doc 500 for kern" },
    TableField { table: "GDEF", field: "ext_501", note: "Extended OpenType field doc 501 for GDEF" },
    TableField { table: "GPOS", field: "ext_502", note: "Extended OpenType field doc 502 for GPOS" },
    TableField { table: "GSUB", field: "ext_503", note: "Extended OpenType field doc 503 for GSUB" },
    TableField { table: "head", field: "ext_504", note: "Extended OpenType field doc 504 for head" },
    TableField { table: "hhea", field: "ext_505", note: "Extended OpenType field doc 505 for hhea" },
    TableField { table: "maxp", field: "ext_506", note: "Extended OpenType field doc 506 for maxp" },
    TableField { table: "cmap", field: "ext_507", note: "Extended OpenType field doc 507 for cmap" },
    TableField { table: "glyf", field: "ext_508", note: "Extended OpenType field doc 508 for glyf" },
    TableField { table: "loca", field: "ext_509", note: "Extended OpenType field doc 509 for loca" },
    TableField { table: "hmtx", field: "ext_510", note: "Extended OpenType field doc 510 for hmtx" },
    TableField { table: "name", field: "ext_511", note: "Extended OpenType field doc 511 for name" },
    TableField { table: "post", field: "ext_512", note: "Extended OpenType field doc 512 for post" },
    TableField { table: "OS/2", field: "ext_513", note: "Extended OpenType field doc 513 for OS/2" },
    TableField { table: "kern", field: "ext_514", note: "Extended OpenType field doc 514 for kern" },
    TableField { table: "GDEF", field: "ext_515", note: "Extended OpenType field doc 515 for GDEF" },
    TableField { table: "GPOS", field: "ext_516", note: "Extended OpenType field doc 516 for GPOS" },
    TableField { table: "GSUB", field: "ext_517", note: "Extended OpenType field doc 517 for GSUB" },
    TableField { table: "head", field: "ext_518", note: "Extended OpenType field doc 518 for head" },
    TableField { table: "hhea", field: "ext_519", note: "Extended OpenType field doc 519 for hhea" },
    TableField { table: "maxp", field: "ext_520", note: "Extended OpenType field doc 520 for maxp" },
    TableField { table: "cmap", field: "ext_521", note: "Extended OpenType field doc 521 for cmap" },
    TableField { table: "glyf", field: "ext_522", note: "Extended OpenType field doc 522 for glyf" },
    TableField { table: "loca", field: "ext_523", note: "Extended OpenType field doc 523 for loca" },
    TableField { table: "hmtx", field: "ext_524", note: "Extended OpenType field doc 524 for hmtx" },
    TableField { table: "name", field: "ext_525", note: "Extended OpenType field doc 525 for name" },
    TableField { table: "post", field: "ext_526", note: "Extended OpenType field doc 526 for post" },
    TableField { table: "OS/2", field: "ext_527", note: "Extended OpenType field doc 527 for OS/2" },
    TableField { table: "kern", field: "ext_528", note: "Extended OpenType field doc 528 for kern" },
    TableField { table: "GDEF", field: "ext_529", note: "Extended OpenType field doc 529 for GDEF" },
    TableField { table: "GPOS", field: "ext_530", note: "Extended OpenType field doc 530 for GPOS" },
    TableField { table: "GSUB", field: "ext_531", note: "Extended OpenType field doc 531 for GSUB" },
    TableField { table: "head", field: "ext_532", note: "Extended OpenType field doc 532 for head" },
    TableField { table: "hhea", field: "ext_533", note: "Extended OpenType field doc 533 for hhea" },
    TableField { table: "maxp", field: "ext_534", note: "Extended OpenType field doc 534 for maxp" },
    TableField { table: "cmap", field: "ext_535", note: "Extended OpenType field doc 535 for cmap" },
    TableField { table: "glyf", field: "ext_536", note: "Extended OpenType field doc 536 for glyf" },
    TableField { table: "loca", field: "ext_537", note: "Extended OpenType field doc 537 for loca" },
    TableField { table: "hmtx", field: "ext_538", note: "Extended OpenType field doc 538 for hmtx" },
    TableField { table: "name", field: "ext_539", note: "Extended OpenType field doc 539 for name" },
    TableField { table: "post", field: "ext_540", note: "Extended OpenType field doc 540 for post" },
    TableField { table: "OS/2", field: "ext_541", note: "Extended OpenType field doc 541 for OS/2" },
    TableField { table: "kern", field: "ext_542", note: "Extended OpenType field doc 542 for kern" },
    TableField { table: "GDEF", field: "ext_543", note: "Extended OpenType field doc 543 for GDEF" },
    TableField { table: "GPOS", field: "ext_544", note: "Extended OpenType field doc 544 for GPOS" },
    TableField { table: "GSUB", field: "ext_545", note: "Extended OpenType field doc 545 for GSUB" },
    TableField { table: "head", field: "ext_546", note: "Extended OpenType field doc 546 for head" },
    TableField { table: "hhea", field: "ext_547", note: "Extended OpenType field doc 547 for hhea" },
    TableField { table: "maxp", field: "ext_548", note: "Extended OpenType field doc 548 for maxp" },
    TableField { table: "cmap", field: "ext_549", note: "Extended OpenType field doc 549 for cmap" },
    TableField { table: "glyf", field: "ext_550", note: "Extended OpenType field doc 550 for glyf" },
    TableField { table: "loca", field: "ext_551", note: "Extended OpenType field doc 551 for loca" },
    TableField { table: "hmtx", field: "ext_552", note: "Extended OpenType field doc 552 for hmtx" },
    TableField { table: "name", field: "ext_553", note: "Extended OpenType field doc 553 for name" },
    TableField { table: "post", field: "ext_554", note: "Extended OpenType field doc 554 for post" },
    TableField { table: "OS/2", field: "ext_555", note: "Extended OpenType field doc 555 for OS/2" },
    TableField { table: "kern", field: "ext_556", note: "Extended OpenType field doc 556 for kern" },
    TableField { table: "GDEF", field: "ext_557", note: "Extended OpenType field doc 557 for GDEF" },
    TableField { table: "GPOS", field: "ext_558", note: "Extended OpenType field doc 558 for GPOS" },
    TableField { table: "GSUB", field: "ext_559", note: "Extended OpenType field doc 559 for GSUB" },
    TableField { table: "head", field: "ext_560", note: "Extended OpenType field doc 560 for head" },
    TableField { table: "hhea", field: "ext_561", note: "Extended OpenType field doc 561 for hhea" },
    TableField { table: "maxp", field: "ext_562", note: "Extended OpenType field doc 562 for maxp" },
    TableField { table: "cmap", field: "ext_563", note: "Extended OpenType field doc 563 for cmap" },
    TableField { table: "glyf", field: "ext_564", note: "Extended OpenType field doc 564 for glyf" },
    TableField { table: "loca", field: "ext_565", note: "Extended OpenType field doc 565 for loca" },
    TableField { table: "hmtx", field: "ext_566", note: "Extended OpenType field doc 566 for hmtx" },
    TableField { table: "name", field: "ext_567", note: "Extended OpenType field doc 567 for name" },
    TableField { table: "post", field: "ext_568", note: "Extended OpenType field doc 568 for post" },
    TableField { table: "OS/2", field: "ext_569", note: "Extended OpenType field doc 569 for OS/2" },
    TableField { table: "kern", field: "ext_570", note: "Extended OpenType field doc 570 for kern" },
    TableField { table: "GDEF", field: "ext_571", note: "Extended OpenType field doc 571 for GDEF" },
    TableField { table: "GPOS", field: "ext_572", note: "Extended OpenType field doc 572 for GPOS" },
    TableField { table: "GSUB", field: "ext_573", note: "Extended OpenType field doc 573 for GSUB" },
    TableField { table: "head", field: "ext_574", note: "Extended OpenType field doc 574 for head" },
    TableField { table: "hhea", field: "ext_575", note: "Extended OpenType field doc 575 for hhea" },
    TableField { table: "maxp", field: "ext_576", note: "Extended OpenType field doc 576 for maxp" },
    TableField { table: "cmap", field: "ext_577", note: "Extended OpenType field doc 577 for cmap" },
    TableField { table: "glyf", field: "ext_578", note: "Extended OpenType field doc 578 for glyf" },
    TableField { table: "loca", field: "ext_579", note: "Extended OpenType field doc 579 for loca" },
    TableField { table: "hmtx", field: "ext_580", note: "Extended OpenType field doc 580 for hmtx" },
    TableField { table: "name", field: "ext_581", note: "Extended OpenType field doc 581 for name" },
    TableField { table: "post", field: "ext_582", note: "Extended OpenType field doc 582 for post" },
    TableField { table: "OS/2", field: "ext_583", note: "Extended OpenType field doc 583 for OS/2" },
    TableField { table: "kern", field: "ext_584", note: "Extended OpenType field doc 584 for kern" },
    TableField { table: "GDEF", field: "ext_585", note: "Extended OpenType field doc 585 for GDEF" },
    TableField { table: "GPOS", field: "ext_586", note: "Extended OpenType field doc 586 for GPOS" },
    TableField { table: "GSUB", field: "ext_587", note: "Extended OpenType field doc 587 for GSUB" },
    TableField { table: "head", field: "ext_588", note: "Extended OpenType field doc 588 for head" },
    TableField { table: "hhea", field: "ext_589", note: "Extended OpenType field doc 589 for hhea" },
    TableField { table: "maxp", field: "ext_590", note: "Extended OpenType field doc 590 for maxp" },
    TableField { table: "cmap", field: "ext_591", note: "Extended OpenType field doc 591 for cmap" },
    TableField { table: "glyf", field: "ext_592", note: "Extended OpenType field doc 592 for glyf" },
    TableField { table: "loca", field: "ext_593", note: "Extended OpenType field doc 593 for loca" },
    TableField { table: "hmtx", field: "ext_594", note: "Extended OpenType field doc 594 for hmtx" },
    TableField { table: "name", field: "ext_595", note: "Extended OpenType field doc 595 for name" },
    TableField { table: "post", field: "ext_596", note: "Extended OpenType field doc 596 for post" },
    TableField { table: "OS/2", field: "ext_597", note: "Extended OpenType field doc 597 for OS/2" },
    TableField { table: "kern", field: "ext_598", note: "Extended OpenType field doc 598 for kern" },
    TableField { table: "GDEF", field: "ext_599", note: "Extended OpenType field doc 599 for GDEF" },
];
pub fn ext_field_count() -> usize { EXT_FIELDS.len() }

/* MORE_REF_0 */
pub fn ref_note_0() -> &'static str { "OpenType cross-table validation note 0" }

/* MORE_REF_1 */
pub fn ref_note_1() -> &'static str { "OpenType cross-table validation note 1" }

/* MORE_REF_2 */
pub fn ref_note_2() -> &'static str { "OpenType cross-table validation note 2" }

/* MORE_REF_3 */
pub fn ref_note_3() -> &'static str { "OpenType cross-table validation note 3" }

/* MORE_REF_4 */
pub fn ref_note_4() -> &'static str { "OpenType cross-table validation note 4" }

/* MORE_REF_5 */
pub fn ref_note_5() -> &'static str { "OpenType cross-table validation note 5" }

/* MORE_REF_6 */
pub fn ref_note_6() -> &'static str { "OpenType cross-table validation note 6" }

/* MORE_REF_7 */
pub fn ref_note_7() -> &'static str { "OpenType cross-table validation note 7" }

/* MORE_REF_8 */
pub fn ref_note_8() -> &'static str { "OpenType cross-table validation note 8" }

/* MORE_REF_9 */
pub fn ref_note_9() -> &'static str { "OpenType cross-table validation note 9" }

/* MORE_REF_10 */
pub fn ref_note_10() -> &'static str { "OpenType cross-table validation note 10" }

/* MORE_REF_11 */
pub fn ref_note_11() -> &'static str { "OpenType cross-table validation note 11" }

/* MORE_REF_12 */
pub fn ref_note_12() -> &'static str { "OpenType cross-table validation note 12" }

/* MORE_REF_13 */
pub fn ref_note_13() -> &'static str { "OpenType cross-table validation note 13" }

/* MORE_REF_14 */
pub fn ref_note_14() -> &'static str { "OpenType cross-table validation note 14" }

/* MORE_REF_15 */
pub fn ref_note_15() -> &'static str { "OpenType cross-table validation note 15" }

/* MORE_REF_16 */
pub fn ref_note_16() -> &'static str { "OpenType cross-table validation note 16" }

/* MORE_REF_17 */
pub fn ref_note_17() -> &'static str { "OpenType cross-table validation note 17" }

/* MORE_REF_18 */
pub fn ref_note_18() -> &'static str { "OpenType cross-table validation note 18" }

/* MORE_REF_19 */
pub fn ref_note_19() -> &'static str { "OpenType cross-table validation note 19" }

/* MORE_REF_20 */
pub fn ref_note_20() -> &'static str { "OpenType cross-table validation note 20" }

/* MORE_REF_21 */
pub fn ref_note_21() -> &'static str { "OpenType cross-table validation note 21" }

/* MORE_REF_22 */
pub fn ref_note_22() -> &'static str { "OpenType cross-table validation note 22" }

/* MORE_REF_23 */
pub fn ref_note_23() -> &'static str { "OpenType cross-table validation note 23" }

/* MORE_REF_24 */
pub fn ref_note_24() -> &'static str { "OpenType cross-table validation note 24" }

/* MORE_REF_25 */
pub fn ref_note_25() -> &'static str { "OpenType cross-table validation note 25" }

/* MORE_REF_26 */
pub fn ref_note_26() -> &'static str { "OpenType cross-table validation note 26" }

/* MORE_REF_27 */
pub fn ref_note_27() -> &'static str { "OpenType cross-table validation note 27" }

/* MORE_REF_28 */
pub fn ref_note_28() -> &'static str { "OpenType cross-table validation note 28" }

/* MORE_REF_29 */
pub fn ref_note_29() -> &'static str { "OpenType cross-table validation note 29" }

/* MORE_REF_30 */
pub fn ref_note_30() -> &'static str { "OpenType cross-table validation note 30" }

/* MORE_REF_31 */
pub fn ref_note_31() -> &'static str { "OpenType cross-table validation note 31" }

/* MORE_REF_32 */
pub fn ref_note_32() -> &'static str { "OpenType cross-table validation note 32" }

/* MORE_REF_33 */
pub fn ref_note_33() -> &'static str { "OpenType cross-table validation note 33" }

/* MORE_REF_34 */
pub fn ref_note_34() -> &'static str { "OpenType cross-table validation note 34" }

/* MORE_REF_35 */
pub fn ref_note_35() -> &'static str { "OpenType cross-table validation note 35" }

/* MORE_REF_36 */
pub fn ref_note_36() -> &'static str { "OpenType cross-table validation note 36" }

/* MORE_REF_37 */
pub fn ref_note_37() -> &'static str { "OpenType cross-table validation note 37" }

/* MORE_REF_38 */
pub fn ref_note_38() -> &'static str { "OpenType cross-table validation note 38" }

/* MORE_REF_39 */
pub fn ref_note_39() -> &'static str { "OpenType cross-table validation note 39" }

/* MORE_REF_40 */
pub fn ref_note_40() -> &'static str { "OpenType cross-table validation note 40" }

/* MORE_REF_41 */
pub fn ref_note_41() -> &'static str { "OpenType cross-table validation note 41" }

/* MORE_REF_42 */
pub fn ref_note_42() -> &'static str { "OpenType cross-table validation note 42" }

/* MORE_REF_43 */
pub fn ref_note_43() -> &'static str { "OpenType cross-table validation note 43" }

/* MORE_REF_44 */
pub fn ref_note_44() -> &'static str { "OpenType cross-table validation note 44" }

/* MORE_REF_45 */
pub fn ref_note_45() -> &'static str { "OpenType cross-table validation note 45" }

/* MORE_REF_46 */
pub fn ref_note_46() -> &'static str { "OpenType cross-table validation note 46" }

/* MORE_REF_47 */
pub fn ref_note_47() -> &'static str { "OpenType cross-table validation note 47" }

/* MORE_REF_48 */
pub fn ref_note_48() -> &'static str { "OpenType cross-table validation note 48" }

/* MORE_REF_49 */
pub fn ref_note_49() -> &'static str { "OpenType cross-table validation note 49" }

/* MORE_REF_50 */
pub fn ref_note_50() -> &'static str { "OpenType cross-table validation note 50" }

/* MORE_REF_51 */
pub fn ref_note_51() -> &'static str { "OpenType cross-table validation note 51" }

/* MORE_REF_52 */
pub fn ref_note_52() -> &'static str { "OpenType cross-table validation note 52" }

/* MORE_REF_53 */
pub fn ref_note_53() -> &'static str { "OpenType cross-table validation note 53" }

/* MORE_REF_54 */
pub fn ref_note_54() -> &'static str { "OpenType cross-table validation note 54" }

/* MORE_REF_55 */
pub fn ref_note_55() -> &'static str { "OpenType cross-table validation note 55" }

/* MORE_REF_56 */
pub fn ref_note_56() -> &'static str { "OpenType cross-table validation note 56" }

/* MORE_REF_57 */
pub fn ref_note_57() -> &'static str { "OpenType cross-table validation note 57" }

/* MORE_REF_58 */
pub fn ref_note_58() -> &'static str { "OpenType cross-table validation note 58" }

/* MORE_REF_59 */
pub fn ref_note_59() -> &'static str { "OpenType cross-table validation note 59" }

/* MORE_REF_60 */
pub fn ref_note_60() -> &'static str { "OpenType cross-table validation note 60" }

/* MORE_REF_61 */
pub fn ref_note_61() -> &'static str { "OpenType cross-table validation note 61" }

/* MORE_REF_62 */
pub fn ref_note_62() -> &'static str { "OpenType cross-table validation note 62" }

/* MORE_REF_63 */
pub fn ref_note_63() -> &'static str { "OpenType cross-table validation note 63" }

/* MORE_REF_64 */
pub fn ref_note_64() -> &'static str { "OpenType cross-table validation note 64" }

/* MORE_REF_65 */
pub fn ref_note_65() -> &'static str { "OpenType cross-table validation note 65" }

/* MORE_REF_66 */
pub fn ref_note_66() -> &'static str { "OpenType cross-table validation note 66" }

/* MORE_REF_67 */
pub fn ref_note_67() -> &'static str { "OpenType cross-table validation note 67" }

/* MORE_REF_68 */
pub fn ref_note_68() -> &'static str { "OpenType cross-table validation note 68" }

/* MORE_REF_69 */
pub fn ref_note_69() -> &'static str { "OpenType cross-table validation note 69" }

/* MORE_REF_70 */
pub fn ref_note_70() -> &'static str { "OpenType cross-table validation note 70" }

/* MORE_REF_71 */
pub fn ref_note_71() -> &'static str { "OpenType cross-table validation note 71" }

/* MORE_REF_72 */
pub fn ref_note_72() -> &'static str { "OpenType cross-table validation note 72" }

/* MORE_REF_73 */
pub fn ref_note_73() -> &'static str { "OpenType cross-table validation note 73" }

/* MORE_REF_74 */
pub fn ref_note_74() -> &'static str { "OpenType cross-table validation note 74" }

/* MORE_REF_75 */
pub fn ref_note_75() -> &'static str { "OpenType cross-table validation note 75" }

/* MORE_REF_76 */
pub fn ref_note_76() -> &'static str { "OpenType cross-table validation note 76" }

/* MORE_REF_77 */
pub fn ref_note_77() -> &'static str { "OpenType cross-table validation note 77" }

/* MORE_REF_78 */
pub fn ref_note_78() -> &'static str { "OpenType cross-table validation note 78" }

/* MORE_REF_79 */
pub fn ref_note_79() -> &'static str { "OpenType cross-table validation note 79" }

/* MORE_REF_80 */
pub fn ref_note_80() -> &'static str { "OpenType cross-table validation note 80" }

/* MORE_REF_81 */
pub fn ref_note_81() -> &'static str { "OpenType cross-table validation note 81" }

/* MORE_REF_82 */
pub fn ref_note_82() -> &'static str { "OpenType cross-table validation note 82" }

/* MORE_REF_83 */
pub fn ref_note_83() -> &'static str { "OpenType cross-table validation note 83" }

/* MORE_REF_84 */
pub fn ref_note_84() -> &'static str { "OpenType cross-table validation note 84" }

/* MORE_REF_85 */
pub fn ref_note_85() -> &'static str { "OpenType cross-table validation note 85" }

/* MORE_REF_86 */
pub fn ref_note_86() -> &'static str { "OpenType cross-table validation note 86" }

/* MORE_REF_87 */
pub fn ref_note_87() -> &'static str { "OpenType cross-table validation note 87" }

/* MORE_REF_88 */
pub fn ref_note_88() -> &'static str { "OpenType cross-table validation note 88" }

/* MORE_REF_89 */
pub fn ref_note_89() -> &'static str { "OpenType cross-table validation note 89" }

/* MORE_REF_90 */
pub fn ref_note_90() -> &'static str { "OpenType cross-table validation note 90" }

/* MORE_REF_91 */
pub fn ref_note_91() -> &'static str { "OpenType cross-table validation note 91" }

/* MORE_REF_92 */
pub fn ref_note_92() -> &'static str { "OpenType cross-table validation note 92" }

/* MORE_REF_93 */
pub fn ref_note_93() -> &'static str { "OpenType cross-table validation note 93" }

/* MORE_REF_94 */
pub fn ref_note_94() -> &'static str { "OpenType cross-table validation note 94" }

/* MORE_REF_95 */
pub fn ref_note_95() -> &'static str { "OpenType cross-table validation note 95" }

/* MORE_REF_96 */
pub fn ref_note_96() -> &'static str { "OpenType cross-table validation note 96" }

/* MORE_REF_97 */
pub fn ref_note_97() -> &'static str { "OpenType cross-table validation note 97" }

/* MORE_REF_98 */
pub fn ref_note_98() -> &'static str { "OpenType cross-table validation note 98" }

/* MORE_REF_99 */
pub fn ref_note_99() -> &'static str { "OpenType cross-table validation note 99" }

/* MORE_REF_100 */
pub fn ref_note_100() -> &'static str { "OpenType cross-table validation note 100" }

/* MORE_REF_101 */
pub fn ref_note_101() -> &'static str { "OpenType cross-table validation note 101" }

/* MORE_REF_102 */
pub fn ref_note_102() -> &'static str { "OpenType cross-table validation note 102" }

/* MORE_REF_103 */
pub fn ref_note_103() -> &'static str { "OpenType cross-table validation note 103" }

/* MORE_REF_104 */
pub fn ref_note_104() -> &'static str { "OpenType cross-table validation note 104" }

/* MORE_REF_105 */
pub fn ref_note_105() -> &'static str { "OpenType cross-table validation note 105" }

/* MORE_REF_106 */
pub fn ref_note_106() -> &'static str { "OpenType cross-table validation note 106" }

/* MORE_REF_107 */
pub fn ref_note_107() -> &'static str { "OpenType cross-table validation note 107" }

/* MORE_REF_108 */
pub fn ref_note_108() -> &'static str { "OpenType cross-table validation note 108" }

/* MORE_REF_109 */
pub fn ref_note_109() -> &'static str { "OpenType cross-table validation note 109" }

/* MORE_REF_110 */
pub fn ref_note_110() -> &'static str { "OpenType cross-table validation note 110" }

/* MORE_REF_111 */
pub fn ref_note_111() -> &'static str { "OpenType cross-table validation note 111" }

/* MORE_REF_112 */
pub fn ref_note_112() -> &'static str { "OpenType cross-table validation note 112" }

/* MORE_REF_113 */
pub fn ref_note_113() -> &'static str { "OpenType cross-table validation note 113" }

/* MORE_REF_114 */
pub fn ref_note_114() -> &'static str { "OpenType cross-table validation note 114" }

/* MORE_REF_115 */
pub fn ref_note_115() -> &'static str { "OpenType cross-table validation note 115" }

/* MORE_REF_116 */
pub fn ref_note_116() -> &'static str { "OpenType cross-table validation note 116" }

/* MORE_REF_117 */
pub fn ref_note_117() -> &'static str { "OpenType cross-table validation note 117" }

/* MORE_REF_118 */
pub fn ref_note_118() -> &'static str { "OpenType cross-table validation note 118" }

/* MORE_REF_119 */
pub fn ref_note_119() -> &'static str { "OpenType cross-table validation note 119" }

/* MORE_REF_120 */
pub fn ref_note_120() -> &'static str { "OpenType cross-table validation note 120" }

/* MORE_REF_121 */
pub fn ref_note_121() -> &'static str { "OpenType cross-table validation note 121" }

/* MORE_REF_122 */
pub fn ref_note_122() -> &'static str { "OpenType cross-table validation note 122" }

/* MORE_REF_123 */
pub fn ref_note_123() -> &'static str { "OpenType cross-table validation note 123" }

/* MORE_REF_124 */
pub fn ref_note_124() -> &'static str { "OpenType cross-table validation note 124" }

/* MORE_REF_125 */
pub fn ref_note_125() -> &'static str { "OpenType cross-table validation note 125" }

/* MORE_REF_126 */
pub fn ref_note_126() -> &'static str { "OpenType cross-table validation note 126" }

/* MORE_REF_127 */
pub fn ref_note_127() -> &'static str { "OpenType cross-table validation note 127" }

/* MORE_REF_128 */
pub fn ref_note_128() -> &'static str { "OpenType cross-table validation note 128" }

/* MORE_REF_129 */
pub fn ref_note_129() -> &'static str { "OpenType cross-table validation note 129" }

/* MORE_REF_130 */
pub fn ref_note_130() -> &'static str { "OpenType cross-table validation note 130" }

/* MORE_REF_131 */
pub fn ref_note_131() -> &'static str { "OpenType cross-table validation note 131" }

/* MORE_REF_132 */
pub fn ref_note_132() -> &'static str { "OpenType cross-table validation note 132" }

/* MORE_REF_133 */
pub fn ref_note_133() -> &'static str { "OpenType cross-table validation note 133" }

/* MORE_REF_134 */
pub fn ref_note_134() -> &'static str { "OpenType cross-table validation note 134" }

/* MORE_REF_135 */
pub fn ref_note_135() -> &'static str { "OpenType cross-table validation note 135" }

/* MORE_REF_136 */
pub fn ref_note_136() -> &'static str { "OpenType cross-table validation note 136" }

/* MORE_REF_137 */
pub fn ref_note_137() -> &'static str { "OpenType cross-table validation note 137" }

/* MORE_REF_138 */
pub fn ref_note_138() -> &'static str { "OpenType cross-table validation note 138" }

/* MORE_REF_139 */
pub fn ref_note_139() -> &'static str { "OpenType cross-table validation note 139" }

/* MORE_REF_140 */
pub fn ref_note_140() -> &'static str { "OpenType cross-table validation note 140" }

/* MORE_REF_141 */
pub fn ref_note_141() -> &'static str { "OpenType cross-table validation note 141" }

/* MORE_REF_142 */
pub fn ref_note_142() -> &'static str { "OpenType cross-table validation note 142" }

/* MORE_REF_143 */
pub fn ref_note_143() -> &'static str { "OpenType cross-table validation note 143" }

/* MORE_REF_144 */
pub fn ref_note_144() -> &'static str { "OpenType cross-table validation note 144" }

/* MORE_REF_145 */
pub fn ref_note_145() -> &'static str { "OpenType cross-table validation note 145" }

/* MORE_REF_146 */
pub fn ref_note_146() -> &'static str { "OpenType cross-table validation note 146" }

/* MORE_REF_147 */
pub fn ref_note_147() -> &'static str { "OpenType cross-table validation note 147" }

/* MORE_REF_148 */
pub fn ref_note_148() -> &'static str { "OpenType cross-table validation note 148" }

/* MORE_REF_149 */
pub fn ref_note_149() -> &'static str { "OpenType cross-table validation note 149" }

/* MORE_REF_150 */
pub fn ref_note_150() -> &'static str { "OpenType cross-table validation note 150" }

/* MORE_REF_151 */
pub fn ref_note_151() -> &'static str { "OpenType cross-table validation note 151" }

/* MORE_REF_152 */
pub fn ref_note_152() -> &'static str { "OpenType cross-table validation note 152" }

/* MORE_REF_153 */
pub fn ref_note_153() -> &'static str { "OpenType cross-table validation note 153" }

/* MORE_REF_154 */
pub fn ref_note_154() -> &'static str { "OpenType cross-table validation note 154" }

/* MORE_REF_155 */
pub fn ref_note_155() -> &'static str { "OpenType cross-table validation note 155" }

/* MORE_REF_156 */
pub fn ref_note_156() -> &'static str { "OpenType cross-table validation note 156" }

/* MORE_REF_157 */
pub fn ref_note_157() -> &'static str { "OpenType cross-table validation note 157" }

/* MORE_REF_158 */
pub fn ref_note_158() -> &'static str { "OpenType cross-table validation note 158" }

/* MORE_REF_159 */
pub fn ref_note_159() -> &'static str { "OpenType cross-table validation note 159" }

/* MORE_REF_160 */
pub fn ref_note_160() -> &'static str { "OpenType cross-table validation note 160" }

/* MORE_REF_161 */
pub fn ref_note_161() -> &'static str { "OpenType cross-table validation note 161" }

/* MORE_REF_162 */
pub fn ref_note_162() -> &'static str { "OpenType cross-table validation note 162" }

/* MORE_REF_163 */
pub fn ref_note_163() -> &'static str { "OpenType cross-table validation note 163" }

/* MORE_REF_164 */
pub fn ref_note_164() -> &'static str { "OpenType cross-table validation note 164" }

/* MORE_REF_165 */
pub fn ref_note_165() -> &'static str { "OpenType cross-table validation note 165" }

/* MORE_REF_166 */
pub fn ref_note_166() -> &'static str { "OpenType cross-table validation note 166" }

/* MORE_REF_167 */
pub fn ref_note_167() -> &'static str { "OpenType cross-table validation note 167" }

/* MORE_REF_168 */
pub fn ref_note_168() -> &'static str { "OpenType cross-table validation note 168" }

/* MORE_REF_169 */
pub fn ref_note_169() -> &'static str { "OpenType cross-table validation note 169" }

/* MORE_REF_170 */
pub fn ref_note_170() -> &'static str { "OpenType cross-table validation note 170" }

/* MORE_REF_171 */
pub fn ref_note_171() -> &'static str { "OpenType cross-table validation note 171" }

/* MORE_REF_172 */
pub fn ref_note_172() -> &'static str { "OpenType cross-table validation note 172" }

/* MORE_REF_173 */
pub fn ref_note_173() -> &'static str { "OpenType cross-table validation note 173" }

/* MORE_REF_174 */
pub fn ref_note_174() -> &'static str { "OpenType cross-table validation note 174" }

/* MORE_REF_175 */
pub fn ref_note_175() -> &'static str { "OpenType cross-table validation note 175" }

/* MORE_REF_176 */
pub fn ref_note_176() -> &'static str { "OpenType cross-table validation note 176" }

/* MORE_REF_177 */
pub fn ref_note_177() -> &'static str { "OpenType cross-table validation note 177" }

/* MORE_REF_178 */
pub fn ref_note_178() -> &'static str { "OpenType cross-table validation note 178" }

/* MORE_REF_179 */
pub fn ref_note_179() -> &'static str { "OpenType cross-table validation note 179" }

/* MORE_REF_180 */
pub fn ref_note_180() -> &'static str { "OpenType cross-table validation note 180" }

/* MORE_REF_181 */
pub fn ref_note_181() -> &'static str { "OpenType cross-table validation note 181" }

/* MORE_REF_182 */
pub fn ref_note_182() -> &'static str { "OpenType cross-table validation note 182" }

/* MORE_REF_183 */
pub fn ref_note_183() -> &'static str { "OpenType cross-table validation note 183" }

/* MORE_REF_184 */
pub fn ref_note_184() -> &'static str { "OpenType cross-table validation note 184" }

/* MORE_REF_185 */
pub fn ref_note_185() -> &'static str { "OpenType cross-table validation note 185" }

/* MORE_REF_186 */
pub fn ref_note_186() -> &'static str { "OpenType cross-table validation note 186" }

/* MORE_REF_187 */
pub fn ref_note_187() -> &'static str { "OpenType cross-table validation note 187" }

/* MORE_REF_188 */
pub fn ref_note_188() -> &'static str { "OpenType cross-table validation note 188" }

/* MORE_REF_189 */
pub fn ref_note_189() -> &'static str { "OpenType cross-table validation note 189" }

/* MORE_REF_190 */
pub fn ref_note_190() -> &'static str { "OpenType cross-table validation note 190" }

/* MORE_REF_191 */
pub fn ref_note_191() -> &'static str { "OpenType cross-table validation note 191" }

/* MORE_REF_192 */
pub fn ref_note_192() -> &'static str { "OpenType cross-table validation note 192" }

/* MORE_REF_193 */
pub fn ref_note_193() -> &'static str { "OpenType cross-table validation note 193" }

/* MORE_REF_194 */
pub fn ref_note_194() -> &'static str { "OpenType cross-table validation note 194" }

/* MORE_REF_195 */
pub fn ref_note_195() -> &'static str { "OpenType cross-table validation note 195" }

/* MORE_REF_196 */
pub fn ref_note_196() -> &'static str { "OpenType cross-table validation note 196" }

/* MORE_REF_197 */
pub fn ref_note_197() -> &'static str { "OpenType cross-table validation note 197" }

/* MORE_REF_198 */
pub fn ref_note_198() -> &'static str { "OpenType cross-table validation note 198" }

/* MORE_REF_199 */
pub fn ref_note_199() -> &'static str { "OpenType cross-table validation note 199" }

/* MORE_REF_200 */
pub fn ref_note_200() -> &'static str { "OpenType cross-table validation note 200" }

/* MORE_REF_201 */
pub fn ref_note_201() -> &'static str { "OpenType cross-table validation note 201" }

/* MORE_REF_202 */
pub fn ref_note_202() -> &'static str { "OpenType cross-table validation note 202" }

/* MORE_REF_203 */
pub fn ref_note_203() -> &'static str { "OpenType cross-table validation note 203" }

/* MORE_REF_204 */
pub fn ref_note_204() -> &'static str { "OpenType cross-table validation note 204" }

/* MORE_REF_205 */
pub fn ref_note_205() -> &'static str { "OpenType cross-table validation note 205" }

/* MORE_REF_206 */
pub fn ref_note_206() -> &'static str { "OpenType cross-table validation note 206" }

/* MORE_REF_207 */
pub fn ref_note_207() -> &'static str { "OpenType cross-table validation note 207" }

/* MORE_REF_208 */
pub fn ref_note_208() -> &'static str { "OpenType cross-table validation note 208" }

/* MORE_REF_209 */
pub fn ref_note_209() -> &'static str { "OpenType cross-table validation note 209" }

/* MORE_REF_210 */
pub fn ref_note_210() -> &'static str { "OpenType cross-table validation note 210" }

/* MORE_REF_211 */
pub fn ref_note_211() -> &'static str { "OpenType cross-table validation note 211" }

/* MORE_REF_212 */
pub fn ref_note_212() -> &'static str { "OpenType cross-table validation note 212" }

/* MORE_REF_213 */
pub fn ref_note_213() -> &'static str { "OpenType cross-table validation note 213" }

/* MORE_REF_214 */
pub fn ref_note_214() -> &'static str { "OpenType cross-table validation note 214" }

/* MORE_REF_215 */
pub fn ref_note_215() -> &'static str { "OpenType cross-table validation note 215" }

/* MORE_REF_216 */
pub fn ref_note_216() -> &'static str { "OpenType cross-table validation note 216" }

/* MORE_REF_217 */
pub fn ref_note_217() -> &'static str { "OpenType cross-table validation note 217" }

/* MORE_REF_218 */
pub fn ref_note_218() -> &'static str { "OpenType cross-table validation note 218" }

/* MORE_REF_219 */
pub fn ref_note_219() -> &'static str { "OpenType cross-table validation note 219" }

/* MORE_REF_220 */
pub fn ref_note_220() -> &'static str { "OpenType cross-table validation note 220" }

/* MORE_REF_221 */
pub fn ref_note_221() -> &'static str { "OpenType cross-table validation note 221" }

/* MORE_REF_222 */
pub fn ref_note_222() -> &'static str { "OpenType cross-table validation note 222" }

/* MORE_REF_223 */
pub fn ref_note_223() -> &'static str { "OpenType cross-table validation note 223" }

/* MORE_REF_224 */
pub fn ref_note_224() -> &'static str { "OpenType cross-table validation note 224" }

/* MORE_REF_225 */
pub fn ref_note_225() -> &'static str { "OpenType cross-table validation note 225" }

/* MORE_REF_226 */
pub fn ref_note_226() -> &'static str { "OpenType cross-table validation note 226" }

/* MORE_REF_227 */
pub fn ref_note_227() -> &'static str { "OpenType cross-table validation note 227" }

/* MORE_REF_228 */
pub fn ref_note_228() -> &'static str { "OpenType cross-table validation note 228" }

/* MORE_REF_229 */
pub fn ref_note_229() -> &'static str { "OpenType cross-table validation note 229" }

/* MORE_REF_230 */
pub fn ref_note_230() -> &'static str { "OpenType cross-table validation note 230" }

/* MORE_REF_231 */
pub fn ref_note_231() -> &'static str { "OpenType cross-table validation note 231" }

/* MORE_REF_232 */
pub fn ref_note_232() -> &'static str { "OpenType cross-table validation note 232" }

/* MORE_REF_233 */
pub fn ref_note_233() -> &'static str { "OpenType cross-table validation note 233" }

/* MORE_REF_234 */
pub fn ref_note_234() -> &'static str { "OpenType cross-table validation note 234" }

/* MORE_REF_235 */
pub fn ref_note_235() -> &'static str { "OpenType cross-table validation note 235" }

/* MORE_REF_236 */
pub fn ref_note_236() -> &'static str { "OpenType cross-table validation note 236" }

/* MORE_REF_237 */
pub fn ref_note_237() -> &'static str { "OpenType cross-table validation note 237" }

/* MORE_REF_238 */
pub fn ref_note_238() -> &'static str { "OpenType cross-table validation note 238" }

/* MORE_REF_239 */
pub fn ref_note_239() -> &'static str { "OpenType cross-table validation note 239" }

/* MORE_REF_240 */
pub fn ref_note_240() -> &'static str { "OpenType cross-table validation note 240" }

/* MORE_REF_241 */
pub fn ref_note_241() -> &'static str { "OpenType cross-table validation note 241" }

/* MORE_REF_242 */
pub fn ref_note_242() -> &'static str { "OpenType cross-table validation note 242" }

/* MORE_REF_243 */
pub fn ref_note_243() -> &'static str { "OpenType cross-table validation note 243" }

/* MORE_REF_244 */
pub fn ref_note_244() -> &'static str { "OpenType cross-table validation note 244" }

/* MORE_REF_245 */
pub fn ref_note_245() -> &'static str { "OpenType cross-table validation note 245" }

/* MORE_REF_246 */
pub fn ref_note_246() -> &'static str { "OpenType cross-table validation note 246" }

/* MORE_REF_247 */
pub fn ref_note_247() -> &'static str { "OpenType cross-table validation note 247" }

/* MORE_REF_248 */
pub fn ref_note_248() -> &'static str { "OpenType cross-table validation note 248" }

/* MORE_REF_249 */
pub fn ref_note_249() -> &'static str { "OpenType cross-table validation note 249" }

/* MORE_REF_250 */
pub fn ref_note_250() -> &'static str { "OpenType cross-table validation note 250" }

/* MORE_REF_251 */
pub fn ref_note_251() -> &'static str { "OpenType cross-table validation note 251" }

/* MORE_REF_252 */
pub fn ref_note_252() -> &'static str { "OpenType cross-table validation note 252" }

/* MORE_REF_253 */
pub fn ref_note_253() -> &'static str { "OpenType cross-table validation note 253" }

/* MORE_REF_254 */
pub fn ref_note_254() -> &'static str { "OpenType cross-table validation note 254" }

/* MORE_REF_255 */
pub fn ref_note_255() -> &'static str { "OpenType cross-table validation note 255" }

/* MORE_REF_256 */
pub fn ref_note_256() -> &'static str { "OpenType cross-table validation note 256" }

/* MORE_REF_257 */
pub fn ref_note_257() -> &'static str { "OpenType cross-table validation note 257" }

/* MORE_REF_258 */
pub fn ref_note_258() -> &'static str { "OpenType cross-table validation note 258" }

/* MORE_REF_259 */
pub fn ref_note_259() -> &'static str { "OpenType cross-table validation note 259" }

/* MORE_REF_260 */
pub fn ref_note_260() -> &'static str { "OpenType cross-table validation note 260" }

/* MORE_REF_261 */
pub fn ref_note_261() -> &'static str { "OpenType cross-table validation note 261" }

/* MORE_REF_262 */
pub fn ref_note_262() -> &'static str { "OpenType cross-table validation note 262" }

/* MORE_REF_263 */
pub fn ref_note_263() -> &'static str { "OpenType cross-table validation note 263" }

/* MORE_REF_264 */
pub fn ref_note_264() -> &'static str { "OpenType cross-table validation note 264" }

/* MORE_REF_265 */
pub fn ref_note_265() -> &'static str { "OpenType cross-table validation note 265" }

/* MORE_REF_266 */
pub fn ref_note_266() -> &'static str { "OpenType cross-table validation note 266" }

/* MORE_REF_267 */
pub fn ref_note_267() -> &'static str { "OpenType cross-table validation note 267" }

/* MORE_REF_268 */
pub fn ref_note_268() -> &'static str { "OpenType cross-table validation note 268" }

/* MORE_REF_269 */
pub fn ref_note_269() -> &'static str { "OpenType cross-table validation note 269" }

/* MORE_REF_270 */
pub fn ref_note_270() -> &'static str { "OpenType cross-table validation note 270" }

/* MORE_REF_271 */
pub fn ref_note_271() -> &'static str { "OpenType cross-table validation note 271" }

/* MORE_REF_272 */
pub fn ref_note_272() -> &'static str { "OpenType cross-table validation note 272" }

/* MORE_REF_273 */
pub fn ref_note_273() -> &'static str { "OpenType cross-table validation note 273" }

/* MORE_REF_274 */
pub fn ref_note_274() -> &'static str { "OpenType cross-table validation note 274" }

/* MORE_REF_275 */
pub fn ref_note_275() -> &'static str { "OpenType cross-table validation note 275" }

/* MORE_REF_276 */
pub fn ref_note_276() -> &'static str { "OpenType cross-table validation note 276" }

/* MORE_REF_277 */
pub fn ref_note_277() -> &'static str { "OpenType cross-table validation note 277" }

/* MORE_REF_278 */
pub fn ref_note_278() -> &'static str { "OpenType cross-table validation note 278" }

/* MORE_REF_279 */
pub fn ref_note_279() -> &'static str { "OpenType cross-table validation note 279" }

/* MORE_REF_280 */
pub fn ref_note_280() -> &'static str { "OpenType cross-table validation note 280" }

/* MORE_REF_281 */
pub fn ref_note_281() -> &'static str { "OpenType cross-table validation note 281" }

/* MORE_REF_282 */
pub fn ref_note_282() -> &'static str { "OpenType cross-table validation note 282" }

/* MORE_REF_283 */
pub fn ref_note_283() -> &'static str { "OpenType cross-table validation note 283" }

/* MORE_REF_284 */
pub fn ref_note_284() -> &'static str { "OpenType cross-table validation note 284" }

/* MORE_REF_285 */
pub fn ref_note_285() -> &'static str { "OpenType cross-table validation note 285" }

/* MORE_REF_286 */
pub fn ref_note_286() -> &'static str { "OpenType cross-table validation note 286" }

/* MORE_REF_287 */
pub fn ref_note_287() -> &'static str { "OpenType cross-table validation note 287" }

/* MORE_REF_288 */
pub fn ref_note_288() -> &'static str { "OpenType cross-table validation note 288" }

/* MORE_REF_289 */
pub fn ref_note_289() -> &'static str { "OpenType cross-table validation note 289" }

/* MORE_REF_290 */
pub fn ref_note_290() -> &'static str { "OpenType cross-table validation note 290" }

/* MORE_REF_291 */
pub fn ref_note_291() -> &'static str { "OpenType cross-table validation note 291" }

/* MORE_REF_292 */
pub fn ref_note_292() -> &'static str { "OpenType cross-table validation note 292" }

/* MORE_REF_293 */
pub fn ref_note_293() -> &'static str { "OpenType cross-table validation note 293" }

/* MORE_REF_294 */
pub fn ref_note_294() -> &'static str { "OpenType cross-table validation note 294" }

/* MORE_REF_295 */
pub fn ref_note_295() -> &'static str { "OpenType cross-table validation note 295" }

/* MORE_REF_296 */
pub fn ref_note_296() -> &'static str { "OpenType cross-table validation note 296" }

/* MORE_REF_297 */
pub fn ref_note_297() -> &'static str { "OpenType cross-table validation note 297" }

/* MORE_REF_298 */
pub fn ref_note_298() -> &'static str { "OpenType cross-table validation note 298" }

/* MORE_REF_299 */
pub fn ref_note_299() -> &'static str { "OpenType cross-table validation note 299" }

/* MORE_REF_300 */
pub fn ref_note_300() -> &'static str { "OpenType cross-table validation note 300" }

/* MORE_REF_301 */
pub fn ref_note_301() -> &'static str { "OpenType cross-table validation note 301" }

/* MORE_REF_302 */
pub fn ref_note_302() -> &'static str { "OpenType cross-table validation note 302" }

/* MORE_REF_303 */
pub fn ref_note_303() -> &'static str { "OpenType cross-table validation note 303" }

/* MORE_REF_304 */
pub fn ref_note_304() -> &'static str { "OpenType cross-table validation note 304" }

/* MORE_REF_305 */
pub fn ref_note_305() -> &'static str { "OpenType cross-table validation note 305" }

/* MORE_REF_306 */
pub fn ref_note_306() -> &'static str { "OpenType cross-table validation note 306" }

/* MORE_REF_307 */
pub fn ref_note_307() -> &'static str { "OpenType cross-table validation note 307" }

/* MORE_REF_308 */
pub fn ref_note_308() -> &'static str { "OpenType cross-table validation note 308" }

/* MORE_REF_309 */
pub fn ref_note_309() -> &'static str { "OpenType cross-table validation note 309" }

/* MORE_REF_310 */
pub fn ref_note_310() -> &'static str { "OpenType cross-table validation note 310" }

/* MORE_REF_311 */
pub fn ref_note_311() -> &'static str { "OpenType cross-table validation note 311" }

/* MORE_REF_312 */
pub fn ref_note_312() -> &'static str { "OpenType cross-table validation note 312" }

/* MORE_REF_313 */
pub fn ref_note_313() -> &'static str { "OpenType cross-table validation note 313" }

/* MORE_REF_314 */
pub fn ref_note_314() -> &'static str { "OpenType cross-table validation note 314" }

/* MORE_REF_315 */
pub fn ref_note_315() -> &'static str { "OpenType cross-table validation note 315" }

/* MORE_REF_316 */
pub fn ref_note_316() -> &'static str { "OpenType cross-table validation note 316" }

/* MORE_REF_317 */
pub fn ref_note_317() -> &'static str { "OpenType cross-table validation note 317" }

/* MORE_REF_318 */
pub fn ref_note_318() -> &'static str { "OpenType cross-table validation note 318" }

/* MORE_REF_319 */
pub fn ref_note_319() -> &'static str { "OpenType cross-table validation note 319" }

/* MORE_REF_320 */
pub fn ref_note_320() -> &'static str { "OpenType cross-table validation note 320" }

/* MORE_REF_321 */
pub fn ref_note_321() -> &'static str { "OpenType cross-table validation note 321" }

/* MORE_REF_322 */
pub fn ref_note_322() -> &'static str { "OpenType cross-table validation note 322" }

/* MORE_REF_323 */
pub fn ref_note_323() -> &'static str { "OpenType cross-table validation note 323" }

/* MORE_REF_324 */
pub fn ref_note_324() -> &'static str { "OpenType cross-table validation note 324" }

/* MORE_REF_325 */
pub fn ref_note_325() -> &'static str { "OpenType cross-table validation note 325" }

/* MORE_REF_326 */
pub fn ref_note_326() -> &'static str { "OpenType cross-table validation note 326" }

/* MORE_REF_327 */
pub fn ref_note_327() -> &'static str { "OpenType cross-table validation note 327" }

/* MORE_REF_328 */
pub fn ref_note_328() -> &'static str { "OpenType cross-table validation note 328" }

/* MORE_REF_329 */
pub fn ref_note_329() -> &'static str { "OpenType cross-table validation note 329" }

/* MORE_REF_330 */
pub fn ref_note_330() -> &'static str { "OpenType cross-table validation note 330" }

/* MORE_REF_331 */
pub fn ref_note_331() -> &'static str { "OpenType cross-table validation note 331" }

/* MORE_REF_332 */
pub fn ref_note_332() -> &'static str { "OpenType cross-table validation note 332" }

/* MORE_REF_333 */
pub fn ref_note_333() -> &'static str { "OpenType cross-table validation note 333" }

/* MORE_REF_334 */
pub fn ref_note_334() -> &'static str { "OpenType cross-table validation note 334" }

/* MORE_REF_335 */
pub fn ref_note_335() -> &'static str { "OpenType cross-table validation note 335" }

/* MORE_REF_336 */
pub fn ref_note_336() -> &'static str { "OpenType cross-table validation note 336" }

/* MORE_REF_337 */
pub fn ref_note_337() -> &'static str { "OpenType cross-table validation note 337" }

/* MORE_REF_338 */
pub fn ref_note_338() -> &'static str { "OpenType cross-table validation note 338" }

/* MORE_REF_339 */
pub fn ref_note_339() -> &'static str { "OpenType cross-table validation note 339" }

/* MORE_REF_340 */
pub fn ref_note_340() -> &'static str { "OpenType cross-table validation note 340" }

/* MORE_REF_341 */
pub fn ref_note_341() -> &'static str { "OpenType cross-table validation note 341" }

/* MORE_REF_342 */
pub fn ref_note_342() -> &'static str { "OpenType cross-table validation note 342" }

/* MORE_REF_343 */
pub fn ref_note_343() -> &'static str { "OpenType cross-table validation note 343" }

/* MORE_REF_344 */
pub fn ref_note_344() -> &'static str { "OpenType cross-table validation note 344" }

/* MORE_REF_345 */
pub fn ref_note_345() -> &'static str { "OpenType cross-table validation note 345" }

/* MORE_REF_346 */
pub fn ref_note_346() -> &'static str { "OpenType cross-table validation note 346" }

/* MORE_REF_347 */
pub fn ref_note_347() -> &'static str { "OpenType cross-table validation note 347" }

/* MORE_REF_348 */
pub fn ref_note_348() -> &'static str { "OpenType cross-table validation note 348" }

/* MORE_REF_349 */
pub fn ref_note_349() -> &'static str { "OpenType cross-table validation note 349" }

/* MORE_REF_350 */
pub fn ref_note_350() -> &'static str { "OpenType cross-table validation note 350" }

/* MORE_REF_351 */
pub fn ref_note_351() -> &'static str { "OpenType cross-table validation note 351" }

/* MORE_REF_352 */
pub fn ref_note_352() -> &'static str { "OpenType cross-table validation note 352" }

/* MORE_REF_353 */
pub fn ref_note_353() -> &'static str { "OpenType cross-table validation note 353" }

/* MORE_REF_354 */
pub fn ref_note_354() -> &'static str { "OpenType cross-table validation note 354" }

/* MORE_REF_355 */
pub fn ref_note_355() -> &'static str { "OpenType cross-table validation note 355" }

/* MORE_REF_356 */
pub fn ref_note_356() -> &'static str { "OpenType cross-table validation note 356" }

/* MORE_REF_357 */
pub fn ref_note_357() -> &'static str { "OpenType cross-table validation note 357" }

/* MORE_REF_358 */
pub fn ref_note_358() -> &'static str { "OpenType cross-table validation note 358" }

/* MORE_REF_359 */
pub fn ref_note_359() -> &'static str { "OpenType cross-table validation note 359" }

/* MORE_REF_360 */
pub fn ref_note_360() -> &'static str { "OpenType cross-table validation note 360" }

/* MORE_REF_361 */
pub fn ref_note_361() -> &'static str { "OpenType cross-table validation note 361" }

/* MORE_REF_362 */
pub fn ref_note_362() -> &'static str { "OpenType cross-table validation note 362" }

/* MORE_REF_363 */
pub fn ref_note_363() -> &'static str { "OpenType cross-table validation note 363" }

/* MORE_REF_364 */
pub fn ref_note_364() -> &'static str { "OpenType cross-table validation note 364" }

/* MORE_REF_365 */
pub fn ref_note_365() -> &'static str { "OpenType cross-table validation note 365" }

/* MORE_REF_366 */
pub fn ref_note_366() -> &'static str { "OpenType cross-table validation note 366" }

/* MORE_REF_367 */
pub fn ref_note_367() -> &'static str { "OpenType cross-table validation note 367" }

/* MORE_REF_368 */
pub fn ref_note_368() -> &'static str { "OpenType cross-table validation note 368" }

/* MORE_REF_369 */
pub fn ref_note_369() -> &'static str { "OpenType cross-table validation note 369" }

/* MORE_REF_370 */
pub fn ref_note_370() -> &'static str { "OpenType cross-table validation note 370" }

/* MORE_REF_371 */
pub fn ref_note_371() -> &'static str { "OpenType cross-table validation note 371" }

/* MORE_REF_372 */
pub fn ref_note_372() -> &'static str { "OpenType cross-table validation note 372" }

/* MORE_REF_373 */
pub fn ref_note_373() -> &'static str { "OpenType cross-table validation note 373" }

/* MORE_REF_374 */
pub fn ref_note_374() -> &'static str { "OpenType cross-table validation note 374" }

/* MORE_REF_375 */
pub fn ref_note_375() -> &'static str { "OpenType cross-table validation note 375" }

/* MORE_REF_376 */
pub fn ref_note_376() -> &'static str { "OpenType cross-table validation note 376" }

/* MORE_REF_377 */
pub fn ref_note_377() -> &'static str { "OpenType cross-table validation note 377" }

/* MORE_REF_378 */
pub fn ref_note_378() -> &'static str { "OpenType cross-table validation note 378" }

/* MORE_REF_379 */
pub fn ref_note_379() -> &'static str { "OpenType cross-table validation note 379" }

/* MORE_REF_380 */
pub fn ref_note_380() -> &'static str { "OpenType cross-table validation note 380" }

/* MORE_REF_381 */
pub fn ref_note_381() -> &'static str { "OpenType cross-table validation note 381" }

/* MORE_REF_382 */
pub fn ref_note_382() -> &'static str { "OpenType cross-table validation note 382" }

/* MORE_REF_383 */
pub fn ref_note_383() -> &'static str { "OpenType cross-table validation note 383" }

/* MORE_REF_384 */
pub fn ref_note_384() -> &'static str { "OpenType cross-table validation note 384" }

/* MORE_REF_385 */
pub fn ref_note_385() -> &'static str { "OpenType cross-table validation note 385" }

/* MORE_REF_386 */
pub fn ref_note_386() -> &'static str { "OpenType cross-table validation note 386" }

/* MORE_REF_387 */
pub fn ref_note_387() -> &'static str { "OpenType cross-table validation note 387" }

/* MORE_REF_388 */
pub fn ref_note_388() -> &'static str { "OpenType cross-table validation note 388" }

/* MORE_REF_389 */
pub fn ref_note_389() -> &'static str { "OpenType cross-table validation note 389" }

/* MORE_REF_390 */
pub fn ref_note_390() -> &'static str { "OpenType cross-table validation note 390" }

/* MORE_REF_391 */
pub fn ref_note_391() -> &'static str { "OpenType cross-table validation note 391" }

/* MORE_REF_392 */
pub fn ref_note_392() -> &'static str { "OpenType cross-table validation note 392" }

/* MORE_REF_393 */
pub fn ref_note_393() -> &'static str { "OpenType cross-table validation note 393" }

/* MORE_REF_394 */
pub fn ref_note_394() -> &'static str { "OpenType cross-table validation note 394" }

/* MORE_REF_395 */
pub fn ref_note_395() -> &'static str { "OpenType cross-table validation note 395" }

/* MORE_REF_396 */
pub fn ref_note_396() -> &'static str { "OpenType cross-table validation note 396" }

/* MORE_REF_397 */
pub fn ref_note_397() -> &'static str { "OpenType cross-table validation note 397" }

/* MORE_REF_398 */
pub fn ref_note_398() -> &'static str { "OpenType cross-table validation note 398" }

/* MORE_REF_399 */
pub fn ref_note_399() -> &'static str { "OpenType cross-table validation note 399" }

/* MORE_REF_400 */
pub fn ref_note_400() -> &'static str { "OpenType cross-table validation note 400" }

/* MORE_REF_401 */
pub fn ref_note_401() -> &'static str { "OpenType cross-table validation note 401" }

/* MORE_REF_402 */
pub fn ref_note_402() -> &'static str { "OpenType cross-table validation note 402" }

/* MORE_REF_403 */
pub fn ref_note_403() -> &'static str { "OpenType cross-table validation note 403" }

/* MORE_REF_404 */
pub fn ref_note_404() -> &'static str { "OpenType cross-table validation note 404" }

/* MORE_REF_405 */
pub fn ref_note_405() -> &'static str { "OpenType cross-table validation note 405" }

/* MORE_REF_406 */
pub fn ref_note_406() -> &'static str { "OpenType cross-table validation note 406" }

/* MORE_REF_407 */
pub fn ref_note_407() -> &'static str { "OpenType cross-table validation note 407" }

/* MORE_REF_408 */
pub fn ref_note_408() -> &'static str { "OpenType cross-table validation note 408" }

/* MORE_REF_409 */
pub fn ref_note_409() -> &'static str { "OpenType cross-table validation note 409" }

/* MORE_REF_410 */
pub fn ref_note_410() -> &'static str { "OpenType cross-table validation note 410" }

/* MORE_REF_411 */
pub fn ref_note_411() -> &'static str { "OpenType cross-table validation note 411" }

/* MORE_REF_412 */
pub fn ref_note_412() -> &'static str { "OpenType cross-table validation note 412" }

/* MORE_REF_413 */
pub fn ref_note_413() -> &'static str { "OpenType cross-table validation note 413" }

/* MORE_REF_414 */
pub fn ref_note_414() -> &'static str { "OpenType cross-table validation note 414" }

/* MORE_REF_415 */
pub fn ref_note_415() -> &'static str { "OpenType cross-table validation note 415" }

/* MORE_REF_416 */
pub fn ref_note_416() -> &'static str { "OpenType cross-table validation note 416" }

/* MORE_REF_417 */
pub fn ref_note_417() -> &'static str { "OpenType cross-table validation note 417" }

/* MORE_REF_418 */
pub fn ref_note_418() -> &'static str { "OpenType cross-table validation note 418" }

/* MORE_REF_419 */
pub fn ref_note_419() -> &'static str { "OpenType cross-table validation note 419" }

/* MORE_REF_420 */
pub fn ref_note_420() -> &'static str { "OpenType cross-table validation note 420" }

/* MORE_REF_421 */
pub fn ref_note_421() -> &'static str { "OpenType cross-table validation note 421" }

/* MORE_REF_422 */
pub fn ref_note_422() -> &'static str { "OpenType cross-table validation note 422" }

/* MORE_REF_423 */
pub fn ref_note_423() -> &'static str { "OpenType cross-table validation note 423" }

/* MORE_REF_424 */
pub fn ref_note_424() -> &'static str { "OpenType cross-table validation note 424" }

/* MORE_REF_425 */
pub fn ref_note_425() -> &'static str { "OpenType cross-table validation note 425" }

/* MORE_REF_426 */
pub fn ref_note_426() -> &'static str { "OpenType cross-table validation note 426" }

/* MORE_REF_427 */
pub fn ref_note_427() -> &'static str { "OpenType cross-table validation note 427" }

/* MORE_REF_428 */
pub fn ref_note_428() -> &'static str { "OpenType cross-table validation note 428" }

/* MORE_REF_429 */
pub fn ref_note_429() -> &'static str { "OpenType cross-table validation note 429" }

/* MORE_REF_430 */
pub fn ref_note_430() -> &'static str { "OpenType cross-table validation note 430" }

/* MORE_REF_431 */
pub fn ref_note_431() -> &'static str { "OpenType cross-table validation note 431" }

/* MORE_REF_432 */
pub fn ref_note_432() -> &'static str { "OpenType cross-table validation note 432" }

/* MORE_REF_433 */
pub fn ref_note_433() -> &'static str { "OpenType cross-table validation note 433" }

/* MORE_REF_434 */
pub fn ref_note_434() -> &'static str { "OpenType cross-table validation note 434" }

/* MORE_REF_435 */
pub fn ref_note_435() -> &'static str { "OpenType cross-table validation note 435" }

/* MORE_REF_436 */
pub fn ref_note_436() -> &'static str { "OpenType cross-table validation note 436" }

/* MORE_REF_437 */
pub fn ref_note_437() -> &'static str { "OpenType cross-table validation note 437" }

/* MORE_REF_438 */
pub fn ref_note_438() -> &'static str { "OpenType cross-table validation note 438" }

/* MORE_REF_439 */
pub fn ref_note_439() -> &'static str { "OpenType cross-table validation note 439" }

/* MORE_REF_440 */
pub fn ref_note_440() -> &'static str { "OpenType cross-table validation note 440" }

/* MORE_REF_441 */
pub fn ref_note_441() -> &'static str { "OpenType cross-table validation note 441" }

/* MORE_REF_442 */
pub fn ref_note_442() -> &'static str { "OpenType cross-table validation note 442" }

/* MORE_REF_443 */
pub fn ref_note_443() -> &'static str { "OpenType cross-table validation note 443" }

/* MORE_REF_444 */
pub fn ref_note_444() -> &'static str { "OpenType cross-table validation note 444" }

/* MORE_REF_445 */
pub fn ref_note_445() -> &'static str { "OpenType cross-table validation note 445" }

/* MORE_REF_446 */
pub fn ref_note_446() -> &'static str { "OpenType cross-table validation note 446" }

/* MORE_REF_447 */
pub fn ref_note_447() -> &'static str { "OpenType cross-table validation note 447" }

/* MORE_REF_448 */
pub fn ref_note_448() -> &'static str { "OpenType cross-table validation note 448" }

/* MORE_REF_449 */
pub fn ref_note_449() -> &'static str { "OpenType cross-table validation note 449" }

/* MORE_REF_450 */
pub fn ref_note_450() -> &'static str { "OpenType cross-table validation note 450" }

/* MORE_REF_451 */
pub fn ref_note_451() -> &'static str { "OpenType cross-table validation note 451" }

/* MORE_REF_452 */
pub fn ref_note_452() -> &'static str { "OpenType cross-table validation note 452" }

/* MORE_REF_453 */
pub fn ref_note_453() -> &'static str { "OpenType cross-table validation note 453" }

/* MORE_REF_454 */
pub fn ref_note_454() -> &'static str { "OpenType cross-table validation note 454" }

/* MORE_REF_455 */
pub fn ref_note_455() -> &'static str { "OpenType cross-table validation note 455" }

/* MORE_REF_456 */
pub fn ref_note_456() -> &'static str { "OpenType cross-table validation note 456" }

/* MORE_REF_457 */
pub fn ref_note_457() -> &'static str { "OpenType cross-table validation note 457" }

/* MORE_REF_458 */
pub fn ref_note_458() -> &'static str { "OpenType cross-table validation note 458" }

/* MORE_REF_459 */
pub fn ref_note_459() -> &'static str { "OpenType cross-table validation note 459" }

/* MORE_REF_460 */
pub fn ref_note_460() -> &'static str { "OpenType cross-table validation note 460" }

/* MORE_REF_461 */
pub fn ref_note_461() -> &'static str { "OpenType cross-table validation note 461" }

/* MORE_REF_462 */
pub fn ref_note_462() -> &'static str { "OpenType cross-table validation note 462" }

/* MORE_REF_463 */
pub fn ref_note_463() -> &'static str { "OpenType cross-table validation note 463" }

/* MORE_REF_464 */
pub fn ref_note_464() -> &'static str { "OpenType cross-table validation note 464" }

/* MORE_REF_465 */
pub fn ref_note_465() -> &'static str { "OpenType cross-table validation note 465" }

/* MORE_REF_466 */
pub fn ref_note_466() -> &'static str { "OpenType cross-table validation note 466" }

/* MORE_REF_467 */
pub fn ref_note_467() -> &'static str { "OpenType cross-table validation note 467" }

/* MORE_REF_468 */
pub fn ref_note_468() -> &'static str { "OpenType cross-table validation note 468" }

/* MORE_REF_469 */
pub fn ref_note_469() -> &'static str { "OpenType cross-table validation note 469" }

/* MORE_REF_470 */
pub fn ref_note_470() -> &'static str { "OpenType cross-table validation note 470" }

/* MORE_REF_471 */
pub fn ref_note_471() -> &'static str { "OpenType cross-table validation note 471" }

/* MORE_REF_472 */
pub fn ref_note_472() -> &'static str { "OpenType cross-table validation note 472" }

/* MORE_REF_473 */
pub fn ref_note_473() -> &'static str { "OpenType cross-table validation note 473" }

/* MORE_REF_474 */
pub fn ref_note_474() -> &'static str { "OpenType cross-table validation note 474" }

/* MORE_REF_475 */
pub fn ref_note_475() -> &'static str { "OpenType cross-table validation note 475" }

/* MORE_REF_476 */
pub fn ref_note_476() -> &'static str { "OpenType cross-table validation note 476" }

/* MORE_REF_477 */
pub fn ref_note_477() -> &'static str { "OpenType cross-table validation note 477" }

/* MORE_REF_478 */
pub fn ref_note_478() -> &'static str { "OpenType cross-table validation note 478" }

/* MORE_REF_479 */
pub fn ref_note_479() -> &'static str { "OpenType cross-table validation note 479" }

/* MORE_REF_480 */
pub fn ref_note_480() -> &'static str { "OpenType cross-table validation note 480" }

/* MORE_REF_481 */
pub fn ref_note_481() -> &'static str { "OpenType cross-table validation note 481" }

/* MORE_REF_482 */
pub fn ref_note_482() -> &'static str { "OpenType cross-table validation note 482" }

/* MORE_REF_483 */
pub fn ref_note_483() -> &'static str { "OpenType cross-table validation note 483" }

/* MORE_REF_484 */
pub fn ref_note_484() -> &'static str { "OpenType cross-table validation note 484" }

/* MORE_REF_485 */
pub fn ref_note_485() -> &'static str { "OpenType cross-table validation note 485" }

/* MORE_REF_486 */
pub fn ref_note_486() -> &'static str { "OpenType cross-table validation note 486" }

/* MORE_REF_487 */
pub fn ref_note_487() -> &'static str { "OpenType cross-table validation note 487" }

/* MORE_REF_488 */
pub fn ref_note_488() -> &'static str { "OpenType cross-table validation note 488" }

/* MORE_REF_489 */
pub fn ref_note_489() -> &'static str { "OpenType cross-table validation note 489" }

/* MORE_REF_490 */
pub fn ref_note_490() -> &'static str { "OpenType cross-table validation note 490" }

/* MORE_REF_491 */
pub fn ref_note_491() -> &'static str { "OpenType cross-table validation note 491" }

/* MORE_REF_492 */
pub fn ref_note_492() -> &'static str { "OpenType cross-table validation note 492" }

/* MORE_REF_493 */
pub fn ref_note_493() -> &'static str { "OpenType cross-table validation note 493" }

/* MORE_REF_494 */
pub fn ref_note_494() -> &'static str { "OpenType cross-table validation note 494" }

/* MORE_REF_495 */
pub fn ref_note_495() -> &'static str { "OpenType cross-table validation note 495" }

/* MORE_REF_496 */
pub fn ref_note_496() -> &'static str { "OpenType cross-table validation note 496" }

/* MORE_REF_497 */
pub fn ref_note_497() -> &'static str { "OpenType cross-table validation note 497" }

/* MORE_REF_498 */
pub fn ref_note_498() -> &'static str { "OpenType cross-table validation note 498" }

/* MORE_REF_499 */
pub fn ref_note_499() -> &'static str { "OpenType cross-table validation note 499" }

/* MORE_REF_500 */
pub fn ref_note_500() -> &'static str { "OpenType cross-table validation note 500" }

/* MORE_REF_501 */
pub fn ref_note_501() -> &'static str { "OpenType cross-table validation note 501" }

/* MORE_REF_502 */
pub fn ref_note_502() -> &'static str { "OpenType cross-table validation note 502" }

/* MORE_REF_503 */
pub fn ref_note_503() -> &'static str { "OpenType cross-table validation note 503" }

/* MORE_REF_504 */
pub fn ref_note_504() -> &'static str { "OpenType cross-table validation note 504" }

/* MORE_REF_505 */
pub fn ref_note_505() -> &'static str { "OpenType cross-table validation note 505" }

/* MORE_REF_506 */
pub fn ref_note_506() -> &'static str { "OpenType cross-table validation note 506" }

/* MORE_REF_507 */
pub fn ref_note_507() -> &'static str { "OpenType cross-table validation note 507" }

/* MORE_REF_508 */
pub fn ref_note_508() -> &'static str { "OpenType cross-table validation note 508" }

/* MORE_REF_509 */
pub fn ref_note_509() -> &'static str { "OpenType cross-table validation note 509" }

/* MORE_REF_510 */
pub fn ref_note_510() -> &'static str { "OpenType cross-table validation note 510" }

/* MORE_REF_511 */
pub fn ref_note_511() -> &'static str { "OpenType cross-table validation note 511" }

/* MORE_REF_512 */
pub fn ref_note_512() -> &'static str { "OpenType cross-table validation note 512" }

/* MORE_REF_513 */
pub fn ref_note_513() -> &'static str { "OpenType cross-table validation note 513" }

/* MORE_REF_514 */
pub fn ref_note_514() -> &'static str { "OpenType cross-table validation note 514" }

/* MORE_REF_515 */
pub fn ref_note_515() -> &'static str { "OpenType cross-table validation note 515" }

/* MORE_REF_516 */
pub fn ref_note_516() -> &'static str { "OpenType cross-table validation note 516" }

/* MORE_REF_517 */
pub fn ref_note_517() -> &'static str { "OpenType cross-table validation note 517" }

/* MORE_REF_518 */
pub fn ref_note_518() -> &'static str { "OpenType cross-table validation note 518" }

/* MORE_REF_519 */
pub fn ref_note_519() -> &'static str { "OpenType cross-table validation note 519" }

/* MORE_REF_520 */
pub fn ref_note_520() -> &'static str { "OpenType cross-table validation note 520" }

/* MORE_REF_521 */
pub fn ref_note_521() -> &'static str { "OpenType cross-table validation note 521" }

/* MORE_REF_522 */
pub fn ref_note_522() -> &'static str { "OpenType cross-table validation note 522" }

/* MORE_REF_523 */
pub fn ref_note_523() -> &'static str { "OpenType cross-table validation note 523" }

/* MORE_REF_524 */
pub fn ref_note_524() -> &'static str { "OpenType cross-table validation note 524" }

/* MORE_REF_525 */
pub fn ref_note_525() -> &'static str { "OpenType cross-table validation note 525" }

/* MORE_REF_526 */
pub fn ref_note_526() -> &'static str { "OpenType cross-table validation note 526" }

/* MORE_REF_527 */
pub fn ref_note_527() -> &'static str { "OpenType cross-table validation note 527" }

/* MORE_REF_528 */
pub fn ref_note_528() -> &'static str { "OpenType cross-table validation note 528" }

/* MORE_REF_529 */
pub fn ref_note_529() -> &'static str { "OpenType cross-table validation note 529" }

/* MORE_REF_530 */
pub fn ref_note_530() -> &'static str { "OpenType cross-table validation note 530" }

/* MORE_REF_531 */
pub fn ref_note_531() -> &'static str { "OpenType cross-table validation note 531" }

/* MORE_REF_532 */
pub fn ref_note_532() -> &'static str { "OpenType cross-table validation note 532" }

/* MORE_REF_533 */
pub fn ref_note_533() -> &'static str { "OpenType cross-table validation note 533" }

/* MORE_REF_534 */
pub fn ref_note_534() -> &'static str { "OpenType cross-table validation note 534" }

/* MORE_REF_535 */
pub fn ref_note_535() -> &'static str { "OpenType cross-table validation note 535" }

/* MORE_REF_536 */
pub fn ref_note_536() -> &'static str { "OpenType cross-table validation note 536" }

/* MORE_REF_537 */
pub fn ref_note_537() -> &'static str { "OpenType cross-table validation note 537" }

/* MORE_REF_538 */
pub fn ref_note_538() -> &'static str { "OpenType cross-table validation note 538" }

/* MORE_REF_539 */
pub fn ref_note_539() -> &'static str { "OpenType cross-table validation note 539" }

/* MORE_REF_540 */
pub fn ref_note_540() -> &'static str { "OpenType cross-table validation note 540" }

/* MORE_REF_541 */
pub fn ref_note_541() -> &'static str { "OpenType cross-table validation note 541" }

/* MORE_REF_542 */
pub fn ref_note_542() -> &'static str { "OpenType cross-table validation note 542" }

/* MORE_REF_543 */
pub fn ref_note_543() -> &'static str { "OpenType cross-table validation note 543" }

/* MORE_REF_544 */
pub fn ref_note_544() -> &'static str { "OpenType cross-table validation note 544" }

/* MORE_REF_545 */
pub fn ref_note_545() -> &'static str { "OpenType cross-table validation note 545" }

/* MORE_REF_546 */
pub fn ref_note_546() -> &'static str { "OpenType cross-table validation note 546" }

/* MORE_REF_547 */
pub fn ref_note_547() -> &'static str { "OpenType cross-table validation note 547" }

/* MORE_REF_548 */
pub fn ref_note_548() -> &'static str { "OpenType cross-table validation note 548" }

/* MORE_REF_549 */
pub fn ref_note_549() -> &'static str { "OpenType cross-table validation note 549" }

/* MORE_REF_550 */
pub fn ref_note_550() -> &'static str { "OpenType cross-table validation note 550" }

/* MORE_REF_551 */
pub fn ref_note_551() -> &'static str { "OpenType cross-table validation note 551" }

/* MORE_REF_552 */
pub fn ref_note_552() -> &'static str { "OpenType cross-table validation note 552" }

/* MORE_REF_553 */
pub fn ref_note_553() -> &'static str { "OpenType cross-table validation note 553" }

/* MORE_REF_554 */
pub fn ref_note_554() -> &'static str { "OpenType cross-table validation note 554" }

/* MORE_REF_555 */
pub fn ref_note_555() -> &'static str { "OpenType cross-table validation note 555" }

/* MORE_REF_556 */
pub fn ref_note_556() -> &'static str { "OpenType cross-table validation note 556" }

/* MORE_REF_557 */
pub fn ref_note_557() -> &'static str { "OpenType cross-table validation note 557" }

/* MORE_REF_558 */
pub fn ref_note_558() -> &'static str { "OpenType cross-table validation note 558" }

/* MORE_REF_559 */
pub fn ref_note_559() -> &'static str { "OpenType cross-table validation note 559" }

/* MORE_REF_560 */
pub fn ref_note_560() -> &'static str { "OpenType cross-table validation note 560" }

/* MORE_REF_561 */
pub fn ref_note_561() -> &'static str { "OpenType cross-table validation note 561" }

/* MORE_REF_562 */
pub fn ref_note_562() -> &'static str { "OpenType cross-table validation note 562" }

/* MORE_REF_563 */
pub fn ref_note_563() -> &'static str { "OpenType cross-table validation note 563" }

/* MORE_REF_564 */
pub fn ref_note_564() -> &'static str { "OpenType cross-table validation note 564" }

/* MORE_REF_565 */
pub fn ref_note_565() -> &'static str { "OpenType cross-table validation note 565" }

/* MORE_REF_566 */
pub fn ref_note_566() -> &'static str { "OpenType cross-table validation note 566" }

/* MORE_REF_567 */
pub fn ref_note_567() -> &'static str { "OpenType cross-table validation note 567" }

/* MORE_REF_568 */
pub fn ref_note_568() -> &'static str { "OpenType cross-table validation note 568" }

/* MORE_REF_569 */
pub fn ref_note_569() -> &'static str { "OpenType cross-table validation note 569" }

/* MORE_REF_570 */
pub fn ref_note_570() -> &'static str { "OpenType cross-table validation note 570" }

/* MORE_REF_571 */
pub fn ref_note_571() -> &'static str { "OpenType cross-table validation note 571" }

/* MORE_REF_572 */
pub fn ref_note_572() -> &'static str { "OpenType cross-table validation note 572" }

/* MORE_REF_573 */
pub fn ref_note_573() -> &'static str { "OpenType cross-table validation note 573" }

/* MORE_REF_574 */
pub fn ref_note_574() -> &'static str { "OpenType cross-table validation note 574" }

/* MORE_REF_575 */
pub fn ref_note_575() -> &'static str { "OpenType cross-table validation note 575" }

/* MORE_REF_576 */
pub fn ref_note_576() -> &'static str { "OpenType cross-table validation note 576" }

/* MORE_REF_577 */
pub fn ref_note_577() -> &'static str { "OpenType cross-table validation note 577" }

/* MORE_REF_578 */
pub fn ref_note_578() -> &'static str { "OpenType cross-table validation note 578" }

/* MORE_REF_579 */
pub fn ref_note_579() -> &'static str { "OpenType cross-table validation note 579" }

/* MORE_REF_580 */
pub fn ref_note_580() -> &'static str { "OpenType cross-table validation note 580" }

/* MORE_REF_581 */
pub fn ref_note_581() -> &'static str { "OpenType cross-table validation note 581" }

/* MORE_REF_582 */
pub fn ref_note_582() -> &'static str { "OpenType cross-table validation note 582" }

/* MORE_REF_583 */
pub fn ref_note_583() -> &'static str { "OpenType cross-table validation note 583" }

/* MORE_REF_584 */
pub fn ref_note_584() -> &'static str { "OpenType cross-table validation note 584" }

/* MORE_REF_585 */
pub fn ref_note_585() -> &'static str { "OpenType cross-table validation note 585" }

/* MORE_REF_586 */
pub fn ref_note_586() -> &'static str { "OpenType cross-table validation note 586" }

/* MORE_REF_587 */
pub fn ref_note_587() -> &'static str { "OpenType cross-table validation note 587" }

/* MORE_REF_588 */
pub fn ref_note_588() -> &'static str { "OpenType cross-table validation note 588" }

/* MORE_REF_589 */
pub fn ref_note_589() -> &'static str { "OpenType cross-table validation note 589" }

/* MORE_REF_590 */
pub fn ref_note_590() -> &'static str { "OpenType cross-table validation note 590" }

/* MORE_REF_591 */
pub fn ref_note_591() -> &'static str { "OpenType cross-table validation note 591" }

/* MORE_REF_592 */
pub fn ref_note_592() -> &'static str { "OpenType cross-table validation note 592" }

/* MORE_REF_593 */
pub fn ref_note_593() -> &'static str { "OpenType cross-table validation note 593" }

/* MORE_REF_594 */
pub fn ref_note_594() -> &'static str { "OpenType cross-table validation note 594" }

/* MORE_REF_595 */
pub fn ref_note_595() -> &'static str { "OpenType cross-table validation note 595" }

/* MORE_REF_596 */
pub fn ref_note_596() -> &'static str { "OpenType cross-table validation note 596" }

/* MORE_REF_597 */
pub fn ref_note_597() -> &'static str { "OpenType cross-table validation note 597" }

/* MORE_REF_598 */
pub fn ref_note_598() -> &'static str { "OpenType cross-table validation note 598" }

/* MORE_REF_599 */
pub fn ref_note_599() -> &'static str { "OpenType cross-table validation note 599" }

/* MORE_REF_600 */
pub fn ref_note_600() -> &'static str { "OpenType cross-table validation note 600" }

/* MORE_REF_601 */
pub fn ref_note_601() -> &'static str { "OpenType cross-table validation note 601" }

/* MORE_REF_602 */
pub fn ref_note_602() -> &'static str { "OpenType cross-table validation note 602" }

/* MORE_REF_603 */
pub fn ref_note_603() -> &'static str { "OpenType cross-table validation note 603" }

/* MORE_REF_604 */
pub fn ref_note_604() -> &'static str { "OpenType cross-table validation note 604" }

/* MORE_REF_605 */
pub fn ref_note_605() -> &'static str { "OpenType cross-table validation note 605" }

/* MORE_REF_606 */
pub fn ref_note_606() -> &'static str { "OpenType cross-table validation note 606" }

/* MORE_REF_607 */
pub fn ref_note_607() -> &'static str { "OpenType cross-table validation note 607" }

/* MORE_REF_608 */
pub fn ref_note_608() -> &'static str { "OpenType cross-table validation note 608" }

/* MORE_REF_609 */
pub fn ref_note_609() -> &'static str { "OpenType cross-table validation note 609" }

/* MORE_REF_610 */
pub fn ref_note_610() -> &'static str { "OpenType cross-table validation note 610" }

/* MORE_REF_611 */
pub fn ref_note_611() -> &'static str { "OpenType cross-table validation note 611" }

/* MORE_REF_612 */
pub fn ref_note_612() -> &'static str { "OpenType cross-table validation note 612" }

/* MORE_REF_613 */
pub fn ref_note_613() -> &'static str { "OpenType cross-table validation note 613" }

/* MORE_REF_614 */
pub fn ref_note_614() -> &'static str { "OpenType cross-table validation note 614" }

/* MORE_REF_615 */
pub fn ref_note_615() -> &'static str { "OpenType cross-table validation note 615" }

/* MORE_REF_616 */
pub fn ref_note_616() -> &'static str { "OpenType cross-table validation note 616" }

/* MORE_REF_617 */
pub fn ref_note_617() -> &'static str { "OpenType cross-table validation note 617" }

/* MORE_REF_618 */
pub fn ref_note_618() -> &'static str { "OpenType cross-table validation note 618" }

/* MORE_REF_619 */
pub fn ref_note_619() -> &'static str { "OpenType cross-table validation note 619" }

/* MORE_REF_620 */
pub fn ref_note_620() -> &'static str { "OpenType cross-table validation note 620" }

/* MORE_REF_621 */
pub fn ref_note_621() -> &'static str { "OpenType cross-table validation note 621" }

/* MORE_REF_622 */
pub fn ref_note_622() -> &'static str { "OpenType cross-table validation note 622" }

/* MORE_REF_623 */
pub fn ref_note_623() -> &'static str { "OpenType cross-table validation note 623" }

/* MORE_REF_624 */
pub fn ref_note_624() -> &'static str { "OpenType cross-table validation note 624" }

/* MORE_REF_625 */
pub fn ref_note_625() -> &'static str { "OpenType cross-table validation note 625" }

/* MORE_REF_626 */
pub fn ref_note_626() -> &'static str { "OpenType cross-table validation note 626" }

/* MORE_REF_627 */
pub fn ref_note_627() -> &'static str { "OpenType cross-table validation note 627" }

/* MORE_REF_628 */
pub fn ref_note_628() -> &'static str { "OpenType cross-table validation note 628" }

/* MORE_REF_629 */
pub fn ref_note_629() -> &'static str { "OpenType cross-table validation note 629" }

/* MORE_REF_630 */
pub fn ref_note_630() -> &'static str { "OpenType cross-table validation note 630" }

/* MORE_REF_631 */
pub fn ref_note_631() -> &'static str { "OpenType cross-table validation note 631" }

/* MORE_REF_632 */
pub fn ref_note_632() -> &'static str { "OpenType cross-table validation note 632" }

/* MORE_REF_633 */
pub fn ref_note_633() -> &'static str { "OpenType cross-table validation note 633" }

/* MORE_REF_634 */
pub fn ref_note_634() -> &'static str { "OpenType cross-table validation note 634" }

/* MORE_REF_635 */
pub fn ref_note_635() -> &'static str { "OpenType cross-table validation note 635" }

/* MORE_REF_636 */
pub fn ref_note_636() -> &'static str { "OpenType cross-table validation note 636" }

/* MORE_REF_637 */
pub fn ref_note_637() -> &'static str { "OpenType cross-table validation note 637" }

/* MORE_REF_638 */
pub fn ref_note_638() -> &'static str { "OpenType cross-table validation note 638" }

/* MORE_REF_639 */
pub fn ref_note_639() -> &'static str { "OpenType cross-table validation note 639" }

/* MORE_REF_640 */
pub fn ref_note_640() -> &'static str { "OpenType cross-table validation note 640" }

/* MORE_REF_641 */
pub fn ref_note_641() -> &'static str { "OpenType cross-table validation note 641" }

/* MORE_REF_642 */
pub fn ref_note_642() -> &'static str { "OpenType cross-table validation note 642" }

/* MORE_REF_643 */
pub fn ref_note_643() -> &'static str { "OpenType cross-table validation note 643" }

/* MORE_REF_644 */
pub fn ref_note_644() -> &'static str { "OpenType cross-table validation note 644" }

/* MORE_REF_645 */
pub fn ref_note_645() -> &'static str { "OpenType cross-table validation note 645" }

/* MORE_REF_646 */
pub fn ref_note_646() -> &'static str { "OpenType cross-table validation note 646" }

/* MORE_REF_647 */
pub fn ref_note_647() -> &'static str { "OpenType cross-table validation note 647" }

/* MORE_REF_648 */
pub fn ref_note_648() -> &'static str { "OpenType cross-table validation note 648" }

/* MORE_REF_649 */
pub fn ref_note_649() -> &'static str { "OpenType cross-table validation note 649" }

/* MORE_REF_650 */
pub fn ref_note_650() -> &'static str { "OpenType cross-table validation note 650" }

/* MORE_REF_651 */
pub fn ref_note_651() -> &'static str { "OpenType cross-table validation note 651" }

/* MORE_REF_652 */
pub fn ref_note_652() -> &'static str { "OpenType cross-table validation note 652" }

/* MORE_REF_653 */
pub fn ref_note_653() -> &'static str { "OpenType cross-table validation note 653" }

/* MORE_REF_654 */
pub fn ref_note_654() -> &'static str { "OpenType cross-table validation note 654" }

/* MORE_REF_655 */
pub fn ref_note_655() -> &'static str { "OpenType cross-table validation note 655" }

/* MORE_REF_656 */
pub fn ref_note_656() -> &'static str { "OpenType cross-table validation note 656" }

/* MORE_REF_657 */
pub fn ref_note_657() -> &'static str { "OpenType cross-table validation note 657" }

/* MORE_REF_658 */
pub fn ref_note_658() -> &'static str { "OpenType cross-table validation note 658" }

/* MORE_REF_659 */
pub fn ref_note_659() -> &'static str { "OpenType cross-table validation note 659" }

/* MORE_REF_660 */
pub fn ref_note_660() -> &'static str { "OpenType cross-table validation note 660" }

/* MORE_REF_661 */
pub fn ref_note_661() -> &'static str { "OpenType cross-table validation note 661" }

/* MORE_REF_662 */
pub fn ref_note_662() -> &'static str { "OpenType cross-table validation note 662" }

/* MORE_REF_663 */
pub fn ref_note_663() -> &'static str { "OpenType cross-table validation note 663" }

/* MORE_REF_664 */
pub fn ref_note_664() -> &'static str { "OpenType cross-table validation note 664" }

/* MORE_REF_665 */
pub fn ref_note_665() -> &'static str { "OpenType cross-table validation note 665" }

/* MORE_REF_666 */
pub fn ref_note_666() -> &'static str { "OpenType cross-table validation note 666" }

/* MORE_REF_667 */
pub fn ref_note_667() -> &'static str { "OpenType cross-table validation note 667" }

/* MORE_REF_668 */
pub fn ref_note_668() -> &'static str { "OpenType cross-table validation note 668" }

/* MORE_REF_669 */
pub fn ref_note_669() -> &'static str { "OpenType cross-table validation note 669" }

/* MORE_REF_670 */
pub fn ref_note_670() -> &'static str { "OpenType cross-table validation note 670" }

/* MORE_REF_671 */
pub fn ref_note_671() -> &'static str { "OpenType cross-table validation note 671" }

/* MORE_REF_672 */
pub fn ref_note_672() -> &'static str { "OpenType cross-table validation note 672" }

/* MORE_REF_673 */
pub fn ref_note_673() -> &'static str { "OpenType cross-table validation note 673" }

/* MORE_REF_674 */
pub fn ref_note_674() -> &'static str { "OpenType cross-table validation note 674" }

/* MORE_REF_675 */
pub fn ref_note_675() -> &'static str { "OpenType cross-table validation note 675" }

/* MORE_REF_676 */
pub fn ref_note_676() -> &'static str { "OpenType cross-table validation note 676" }

/* MORE_REF_677 */
pub fn ref_note_677() -> &'static str { "OpenType cross-table validation note 677" }

/* MORE_REF_678 */
pub fn ref_note_678() -> &'static str { "OpenType cross-table validation note 678" }

/* MORE_REF_679 */
pub fn ref_note_679() -> &'static str { "OpenType cross-table validation note 679" }

/* MORE_REF_680 */
pub fn ref_note_680() -> &'static str { "OpenType cross-table validation note 680" }

/* MORE_REF_681 */
pub fn ref_note_681() -> &'static str { "OpenType cross-table validation note 681" }

/* MORE_REF_682 */
pub fn ref_note_682() -> &'static str { "OpenType cross-table validation note 682" }

/* MORE_REF_683 */
pub fn ref_note_683() -> &'static str { "OpenType cross-table validation note 683" }

/* MORE_REF_684 */
pub fn ref_note_684() -> &'static str { "OpenType cross-table validation note 684" }

/* MORE_REF_685 */
pub fn ref_note_685() -> &'static str { "OpenType cross-table validation note 685" }

/* MORE_REF_686 */
pub fn ref_note_686() -> &'static str { "OpenType cross-table validation note 686" }

/* MORE_REF_687 */
pub fn ref_note_687() -> &'static str { "OpenType cross-table validation note 687" }

/* MORE_REF_688 */
pub fn ref_note_688() -> &'static str { "OpenType cross-table validation note 688" }

/* MORE_REF_689 */
pub fn ref_note_689() -> &'static str { "OpenType cross-table validation note 689" }

/* MORE_REF_690 */
pub fn ref_note_690() -> &'static str { "OpenType cross-table validation note 690" }

/* MORE_REF_691 */
pub fn ref_note_691() -> &'static str { "OpenType cross-table validation note 691" }

/* MORE_REF_692 */
pub fn ref_note_692() -> &'static str { "OpenType cross-table validation note 692" }

/* MORE_REF_693 */
pub fn ref_note_693() -> &'static str { "OpenType cross-table validation note 693" }

/* MORE_REF_694 */
pub fn ref_note_694() -> &'static str { "OpenType cross-table validation note 694" }

/* MORE_REF_695 */
pub fn ref_note_695() -> &'static str { "OpenType cross-table validation note 695" }

/* MORE_REF_696 */
pub fn ref_note_696() -> &'static str { "OpenType cross-table validation note 696" }

/* MORE_REF_697 */
pub fn ref_note_697() -> &'static str { "OpenType cross-table validation note 697" }

/* MORE_REF_698 */
pub fn ref_note_698() -> &'static str { "OpenType cross-table validation note 698" }

/* MORE_REF_699 */
pub fn ref_note_699() -> &'static str { "OpenType cross-table validation note 699" }

/* MORE_REF_700 */
pub fn ref_note_700() -> &'static str { "OpenType cross-table validation note 700" }

/* MORE_REF_701 */
pub fn ref_note_701() -> &'static str { "OpenType cross-table validation note 701" }

/* MORE_REF_702 */
pub fn ref_note_702() -> &'static str { "OpenType cross-table validation note 702" }

/* MORE_REF_703 */
pub fn ref_note_703() -> &'static str { "OpenType cross-table validation note 703" }

/* MORE_REF_704 */
pub fn ref_note_704() -> &'static str { "OpenType cross-table validation note 704" }

/* MORE_REF_705 */
pub fn ref_note_705() -> &'static str { "OpenType cross-table validation note 705" }

/* MORE_REF_706 */
pub fn ref_note_706() -> &'static str { "OpenType cross-table validation note 706" }

/* MORE_REF_707 */
pub fn ref_note_707() -> &'static str { "OpenType cross-table validation note 707" }

/* MORE_REF_708 */
pub fn ref_note_708() -> &'static str { "OpenType cross-table validation note 708" }

/* MORE_REF_709 */
pub fn ref_note_709() -> &'static str { "OpenType cross-table validation note 709" }

/* MORE_REF_710 */
pub fn ref_note_710() -> &'static str { "OpenType cross-table validation note 710" }

/* MORE_REF_711 */
pub fn ref_note_711() -> &'static str { "OpenType cross-table validation note 711" }

/* MORE_REF_712 */
pub fn ref_note_712() -> &'static str { "OpenType cross-table validation note 712" }

/* MORE_REF_713 */
pub fn ref_note_713() -> &'static str { "OpenType cross-table validation note 713" }

/* MORE_REF_714 */
pub fn ref_note_714() -> &'static str { "OpenType cross-table validation note 714" }

/* MORE_REF_715 */
pub fn ref_note_715() -> &'static str { "OpenType cross-table validation note 715" }

/* MORE_REF_716 */
pub fn ref_note_716() -> &'static str { "OpenType cross-table validation note 716" }

/* MORE_REF_717 */
pub fn ref_note_717() -> &'static str { "OpenType cross-table validation note 717" }

/* MORE_REF_718 */
pub fn ref_note_718() -> &'static str { "OpenType cross-table validation note 718" }

/* MORE_REF_719 */
pub fn ref_note_719() -> &'static str { "OpenType cross-table validation note 719" }

/* MORE_REF_720 */
pub fn ref_note_720() -> &'static str { "OpenType cross-table validation note 720" }

/* MORE_REF_721 */
pub fn ref_note_721() -> &'static str { "OpenType cross-table validation note 721" }

/* MORE_REF_722 */
pub fn ref_note_722() -> &'static str { "OpenType cross-table validation note 722" }

/* MORE_REF_723 */
pub fn ref_note_723() -> &'static str { "OpenType cross-table validation note 723" }

/* MORE_REF_724 */
pub fn ref_note_724() -> &'static str { "OpenType cross-table validation note 724" }

/* MORE_REF_725 */
pub fn ref_note_725() -> &'static str { "OpenType cross-table validation note 725" }

/* MORE_REF_726 */
pub fn ref_note_726() -> &'static str { "OpenType cross-table validation note 726" }

/* MORE_REF_727 */
pub fn ref_note_727() -> &'static str { "OpenType cross-table validation note 727" }

/* MORE_REF_728 */
pub fn ref_note_728() -> &'static str { "OpenType cross-table validation note 728" }

/* MORE_REF_729 */
pub fn ref_note_729() -> &'static str { "OpenType cross-table validation note 729" }

/* MORE_REF_730 */
pub fn ref_note_730() -> &'static str { "OpenType cross-table validation note 730" }

/* MORE_REF_731 */
pub fn ref_note_731() -> &'static str { "OpenType cross-table validation note 731" }

/* MORE_REF_732 */
pub fn ref_note_732() -> &'static str { "OpenType cross-table validation note 732" }

/* MORE_REF_733 */
pub fn ref_note_733() -> &'static str { "OpenType cross-table validation note 733" }

/* MORE_REF_734 */
pub fn ref_note_734() -> &'static str { "OpenType cross-table validation note 734" }

/* MORE_REF_735 */
pub fn ref_note_735() -> &'static str { "OpenType cross-table validation note 735" }

/* MORE_REF_736 */
pub fn ref_note_736() -> &'static str { "OpenType cross-table validation note 736" }

/* MORE_REF_737 */
pub fn ref_note_737() -> &'static str { "OpenType cross-table validation note 737" }

/* MORE_REF_738 */
pub fn ref_note_738() -> &'static str { "OpenType cross-table validation note 738" }

/* MORE_REF_739 */
pub fn ref_note_739() -> &'static str { "OpenType cross-table validation note 739" }

/* MORE_REF_740 */
pub fn ref_note_740() -> &'static str { "OpenType cross-table validation note 740" }

/* MORE_REF_741 */
pub fn ref_note_741() -> &'static str { "OpenType cross-table validation note 741" }

/* MORE_REF_742 */
pub fn ref_note_742() -> &'static str { "OpenType cross-table validation note 742" }

/* MORE_REF_743 */
pub fn ref_note_743() -> &'static str { "OpenType cross-table validation note 743" }

/* MORE_REF_744 */
pub fn ref_note_744() -> &'static str { "OpenType cross-table validation note 744" }

/* MORE_REF_745 */
pub fn ref_note_745() -> &'static str { "OpenType cross-table validation note 745" }

/* MORE_REF_746 */
pub fn ref_note_746() -> &'static str { "OpenType cross-table validation note 746" }

/* MORE_REF_747 */
pub fn ref_note_747() -> &'static str { "OpenType cross-table validation note 747" }

/* MORE_REF_748 */
pub fn ref_note_748() -> &'static str { "OpenType cross-table validation note 748" }

/* MORE_REF_749 */
pub fn ref_note_749() -> &'static str { "OpenType cross-table validation note 749" }

/* MORE_REF_750 */
pub fn ref_note_750() -> &'static str { "OpenType cross-table validation note 750" }

/* MORE_REF_751 */
pub fn ref_note_751() -> &'static str { "OpenType cross-table validation note 751" }

/* MORE_REF_752 */
pub fn ref_note_752() -> &'static str { "OpenType cross-table validation note 752" }

/* MORE_REF_753 */
pub fn ref_note_753() -> &'static str { "OpenType cross-table validation note 753" }

/* MORE_REF_754 */
pub fn ref_note_754() -> &'static str { "OpenType cross-table validation note 754" }

/* MORE_REF_755 */
pub fn ref_note_755() -> &'static str { "OpenType cross-table validation note 755" }

/* MORE_REF_756 */
pub fn ref_note_756() -> &'static str { "OpenType cross-table validation note 756" }

/* MORE_REF_757 */
pub fn ref_note_757() -> &'static str { "OpenType cross-table validation note 757" }

/* MORE_REF_758 */
pub fn ref_note_758() -> &'static str { "OpenType cross-table validation note 758" }

/* MORE_REF_759 */
pub fn ref_note_759() -> &'static str { "OpenType cross-table validation note 759" }

/* MORE_REF_760 */
pub fn ref_note_760() -> &'static str { "OpenType cross-table validation note 760" }

/* MORE_REF_761 */
pub fn ref_note_761() -> &'static str { "OpenType cross-table validation note 761" }

/* MORE_REF_762 */
pub fn ref_note_762() -> &'static str { "OpenType cross-table validation note 762" }

/* MORE_REF_763 */
pub fn ref_note_763() -> &'static str { "OpenType cross-table validation note 763" }

/* MORE_REF_764 */
pub fn ref_note_764() -> &'static str { "OpenType cross-table validation note 764" }

/* MORE_REF_765 */
pub fn ref_note_765() -> &'static str { "OpenType cross-table validation note 765" }

/* MORE_REF_766 */
pub fn ref_note_766() -> &'static str { "OpenType cross-table validation note 766" }

/* MORE_REF_767 */
pub fn ref_note_767() -> &'static str { "OpenType cross-table validation note 767" }

/* MORE_REF_768 */
pub fn ref_note_768() -> &'static str { "OpenType cross-table validation note 768" }

/* MORE_REF_769 */
pub fn ref_note_769() -> &'static str { "OpenType cross-table validation note 769" }

/* MORE_REF_770 */
pub fn ref_note_770() -> &'static str { "OpenType cross-table validation note 770" }

/* MORE_REF_771 */
pub fn ref_note_771() -> &'static str { "OpenType cross-table validation note 771" }

/* MORE_REF_772 */
pub fn ref_note_772() -> &'static str { "OpenType cross-table validation note 772" }

/* MORE_REF_773 */
pub fn ref_note_773() -> &'static str { "OpenType cross-table validation note 773" }

/* MORE_REF_774 */
pub fn ref_note_774() -> &'static str { "OpenType cross-table validation note 774" }

/* MORE_REF_775 */
pub fn ref_note_775() -> &'static str { "OpenType cross-table validation note 775" }

/* MORE_REF_776 */
pub fn ref_note_776() -> &'static str { "OpenType cross-table validation note 776" }

/* MORE_REF_777 */
pub fn ref_note_777() -> &'static str { "OpenType cross-table validation note 777" }

/* MORE_REF_778 */
pub fn ref_note_778() -> &'static str { "OpenType cross-table validation note 778" }

/* MORE_REF_779 */
pub fn ref_note_779() -> &'static str { "OpenType cross-table validation note 779" }

/* MORE_REF_780 */
pub fn ref_note_780() -> &'static str { "OpenType cross-table validation note 780" }

/* MORE_REF_781 */
pub fn ref_note_781() -> &'static str { "OpenType cross-table validation note 781" }

/* MORE_REF_782 */
pub fn ref_note_782() -> &'static str { "OpenType cross-table validation note 782" }

/* MORE_REF_783 */
pub fn ref_note_783() -> &'static str { "OpenType cross-table validation note 783" }

/* MORE_REF_784 */
pub fn ref_note_784() -> &'static str { "OpenType cross-table validation note 784" }

/* MORE_REF_785 */
pub fn ref_note_785() -> &'static str { "OpenType cross-table validation note 785" }

/* MORE_REF_786 */
pub fn ref_note_786() -> &'static str { "OpenType cross-table validation note 786" }

/* MORE_REF_787 */
pub fn ref_note_787() -> &'static str { "OpenType cross-table validation note 787" }

/* MORE_REF_788 */
pub fn ref_note_788() -> &'static str { "OpenType cross-table validation note 788" }

/* MORE_REF_789 */
pub fn ref_note_789() -> &'static str { "OpenType cross-table validation note 789" }

/* MORE_REF_790 */
pub fn ref_note_790() -> &'static str { "OpenType cross-table validation note 790" }

/* MORE_REF_791 */
pub fn ref_note_791() -> &'static str { "OpenType cross-table validation note 791" }

/* MORE_REF_792 */
pub fn ref_note_792() -> &'static str { "OpenType cross-table validation note 792" }

/* MORE_REF_793 */
pub fn ref_note_793() -> &'static str { "OpenType cross-table validation note 793" }

/* MORE_REF_794 */
pub fn ref_note_794() -> &'static str { "OpenType cross-table validation note 794" }

/* MORE_REF_795 */
pub fn ref_note_795() -> &'static str { "OpenType cross-table validation note 795" }

/* MORE_REF_796 */
pub fn ref_note_796() -> &'static str { "OpenType cross-table validation note 796" }

/* MORE_REF_797 */
pub fn ref_note_797() -> &'static str { "OpenType cross-table validation note 797" }

/* MORE_REF_798 */
pub fn ref_note_798() -> &'static str { "OpenType cross-table validation note 798" }

/* MORE_REF_799 */
pub fn ref_note_799() -> &'static str { "OpenType cross-table validation note 799" }

/* MORE_REF_800 */
pub fn ref_note_800() -> &'static str { "OpenType cross-table validation note 800" }

/* MORE_REF_801 */
pub fn ref_note_801() -> &'static str { "OpenType cross-table validation note 801" }

/* MORE_REF_802 */
pub fn ref_note_802() -> &'static str { "OpenType cross-table validation note 802" }

/* MORE_REF_803 */
pub fn ref_note_803() -> &'static str { "OpenType cross-table validation note 803" }

/* MORE_REF_804 */
pub fn ref_note_804() -> &'static str { "OpenType cross-table validation note 804" }

/* MORE_REF_805 */
pub fn ref_note_805() -> &'static str { "OpenType cross-table validation note 805" }

/* MORE_REF_806 */
pub fn ref_note_806() -> &'static str { "OpenType cross-table validation note 806" }

/* MORE_REF_807 */
pub fn ref_note_807() -> &'static str { "OpenType cross-table validation note 807" }

/* MORE_REF_808 */
pub fn ref_note_808() -> &'static str { "OpenType cross-table validation note 808" }

/* MORE_REF_809 */
pub fn ref_note_809() -> &'static str { "OpenType cross-table validation note 809" }

/* MORE_REF_810 */
pub fn ref_note_810() -> &'static str { "OpenType cross-table validation note 810" }

/* MORE_REF_811 */
pub fn ref_note_811() -> &'static str { "OpenType cross-table validation note 811" }

/* MORE_REF_812 */
pub fn ref_note_812() -> &'static str { "OpenType cross-table validation note 812" }

/* MORE_REF_813 */
pub fn ref_note_813() -> &'static str { "OpenType cross-table validation note 813" }

/* MORE_REF_814 */
pub fn ref_note_814() -> &'static str { "OpenType cross-table validation note 814" }

/* MORE_REF_815 */
pub fn ref_note_815() -> &'static str { "OpenType cross-table validation note 815" }

/* MORE_REF_816 */
pub fn ref_note_816() -> &'static str { "OpenType cross-table validation note 816" }

/* MORE_REF_817 */
pub fn ref_note_817() -> &'static str { "OpenType cross-table validation note 817" }

/* MORE_REF_818 */
pub fn ref_note_818() -> &'static str { "OpenType cross-table validation note 818" }

/* MORE_REF_819 */
pub fn ref_note_819() -> &'static str { "OpenType cross-table validation note 819" }

/* MORE_REF_820 */
pub fn ref_note_820() -> &'static str { "OpenType cross-table validation note 820" }

/* MORE_REF_821 */
pub fn ref_note_821() -> &'static str { "OpenType cross-table validation note 821" }

/* MORE_REF_822 */
pub fn ref_note_822() -> &'static str { "OpenType cross-table validation note 822" }

/* MORE_REF_823 */
pub fn ref_note_823() -> &'static str { "OpenType cross-table validation note 823" }

/* MORE_REF_824 */
pub fn ref_note_824() -> &'static str { "OpenType cross-table validation note 824" }

/* MORE_REF_825 */
pub fn ref_note_825() -> &'static str { "OpenType cross-table validation note 825" }

/* MORE_REF_826 */
pub fn ref_note_826() -> &'static str { "OpenType cross-table validation note 826" }

/* MORE_REF_827 */
pub fn ref_note_827() -> &'static str { "OpenType cross-table validation note 827" }

/* MORE_REF_828 */
pub fn ref_note_828() -> &'static str { "OpenType cross-table validation note 828" }

/* MORE_REF_829 */
pub fn ref_note_829() -> &'static str { "OpenType cross-table validation note 829" }

/* MORE_REF_830 */
pub fn ref_note_830() -> &'static str { "OpenType cross-table validation note 830" }

/* MORE_REF_831 */
pub fn ref_note_831() -> &'static str { "OpenType cross-table validation note 831" }

/* MORE_REF_832 */
pub fn ref_note_832() -> &'static str { "OpenType cross-table validation note 832" }

/* MORE_REF_833 */
pub fn ref_note_833() -> &'static str { "OpenType cross-table validation note 833" }

/* MORE_REF_834 */
pub fn ref_note_834() -> &'static str { "OpenType cross-table validation note 834" }

/* MORE_REF_835 */
pub fn ref_note_835() -> &'static str { "OpenType cross-table validation note 835" }

/* MORE_REF_836 */
pub fn ref_note_836() -> &'static str { "OpenType cross-table validation note 836" }

/* MORE_REF_837 */
pub fn ref_note_837() -> &'static str { "OpenType cross-table validation note 837" }

/* MORE_REF_838 */
pub fn ref_note_838() -> &'static str { "OpenType cross-table validation note 838" }

/* MORE_REF_839 */
pub fn ref_note_839() -> &'static str { "OpenType cross-table validation note 839" }

/* MORE_REF_840 */
pub fn ref_note_840() -> &'static str { "OpenType cross-table validation note 840" }

/* MORE_REF_841 */
pub fn ref_note_841() -> &'static str { "OpenType cross-table validation note 841" }

/* MORE_REF_842 */
pub fn ref_note_842() -> &'static str { "OpenType cross-table validation note 842" }

/* MORE_REF_843 */
pub fn ref_note_843() -> &'static str { "OpenType cross-table validation note 843" }

/* MORE_REF_844 */
pub fn ref_note_844() -> &'static str { "OpenType cross-table validation note 844" }

/* MORE_REF_845 */
pub fn ref_note_845() -> &'static str { "OpenType cross-table validation note 845" }

/* MORE_REF_846 */
pub fn ref_note_846() -> &'static str { "OpenType cross-table validation note 846" }

/* MORE_REF_847 */
pub fn ref_note_847() -> &'static str { "OpenType cross-table validation note 847" }

/* MORE_REF_848 */
pub fn ref_note_848() -> &'static str { "OpenType cross-table validation note 848" }

/* MORE_REF_849 */
pub fn ref_note_849() -> &'static str { "OpenType cross-table validation note 849" }

/* MORE_REF_850 */
pub fn ref_note_850() -> &'static str { "OpenType cross-table validation note 850" }

/* MORE_REF_851 */
pub fn ref_note_851() -> &'static str { "OpenType cross-table validation note 851" }

/* MORE_REF_852 */
pub fn ref_note_852() -> &'static str { "OpenType cross-table validation note 852" }

/* MORE_REF_853 */
pub fn ref_note_853() -> &'static str { "OpenType cross-table validation note 853" }

/* MORE_REF_854 */
pub fn ref_note_854() -> &'static str { "OpenType cross-table validation note 854" }

/* MORE_REF_855 */
pub fn ref_note_855() -> &'static str { "OpenType cross-table validation note 855" }

/* MORE_REF_856 */
pub fn ref_note_856() -> &'static str { "OpenType cross-table validation note 856" }

/* MORE_REF_857 */
pub fn ref_note_857() -> &'static str { "OpenType cross-table validation note 857" }

/* MORE_REF_858 */
pub fn ref_note_858() -> &'static str { "OpenType cross-table validation note 858" }

/* MORE_REF_859 */
pub fn ref_note_859() -> &'static str { "OpenType cross-table validation note 859" }

/* MORE_REF_860 */
pub fn ref_note_860() -> &'static str { "OpenType cross-table validation note 860" }

/* MORE_REF_861 */
pub fn ref_note_861() -> &'static str { "OpenType cross-table validation note 861" }

/* MORE_REF_862 */
pub fn ref_note_862() -> &'static str { "OpenType cross-table validation note 862" }

/* MORE_REF_863 */
pub fn ref_note_863() -> &'static str { "OpenType cross-table validation note 863" }

/* MORE_REF_864 */
pub fn ref_note_864() -> &'static str { "OpenType cross-table validation note 864" }

/* MORE_REF_865 */
pub fn ref_note_865() -> &'static str { "OpenType cross-table validation note 865" }

/* MORE_REF_866 */
pub fn ref_note_866() -> &'static str { "OpenType cross-table validation note 866" }

/* MORE_REF_867 */
pub fn ref_note_867() -> &'static str { "OpenType cross-table validation note 867" }

/* MORE_REF_868 */
pub fn ref_note_868() -> &'static str { "OpenType cross-table validation note 868" }

/* MORE_REF_869 */
pub fn ref_note_869() -> &'static str { "OpenType cross-table validation note 869" }

/* MORE_REF_870 */
pub fn ref_note_870() -> &'static str { "OpenType cross-table validation note 870" }

/* MORE_REF_871 */
pub fn ref_note_871() -> &'static str { "OpenType cross-table validation note 871" }

/* MORE_REF_872 */
pub fn ref_note_872() -> &'static str { "OpenType cross-table validation note 872" }

/* MORE_REF_873 */
pub fn ref_note_873() -> &'static str { "OpenType cross-table validation note 873" }

/* MORE_REF_874 */
pub fn ref_note_874() -> &'static str { "OpenType cross-table validation note 874" }

/* MORE_REF_875 */
pub fn ref_note_875() -> &'static str { "OpenType cross-table validation note 875" }

/* MORE_REF_876 */
pub fn ref_note_876() -> &'static str { "OpenType cross-table validation note 876" }

/* MORE_REF_877 */
pub fn ref_note_877() -> &'static str { "OpenType cross-table validation note 877" }

/* MORE_REF_878 */
pub fn ref_note_878() -> &'static str { "OpenType cross-table validation note 878" }

/* MORE_REF_879 */
pub fn ref_note_879() -> &'static str { "OpenType cross-table validation note 879" }

/* MORE_REF_880 */
pub fn ref_note_880() -> &'static str { "OpenType cross-table validation note 880" }

/* MORE_REF_881 */
pub fn ref_note_881() -> &'static str { "OpenType cross-table validation note 881" }

/* MORE_REF_882 */
pub fn ref_note_882() -> &'static str { "OpenType cross-table validation note 882" }

/* MORE_REF_883 */
pub fn ref_note_883() -> &'static str { "OpenType cross-table validation note 883" }

/* MORE_REF_884 */
pub fn ref_note_884() -> &'static str { "OpenType cross-table validation note 884" }

/* MORE_REF_885 */
pub fn ref_note_885() -> &'static str { "OpenType cross-table validation note 885" }

/* MORE_REF_886 */
pub fn ref_note_886() -> &'static str { "OpenType cross-table validation note 886" }

/* MORE_REF_887 */
pub fn ref_note_887() -> &'static str { "OpenType cross-table validation note 887" }

/* MORE_REF_888 */
pub fn ref_note_888() -> &'static str { "OpenType cross-table validation note 888" }

/* MORE_REF_889 */
pub fn ref_note_889() -> &'static str { "OpenType cross-table validation note 889" }

/* MORE_REF_890 */
pub fn ref_note_890() -> &'static str { "OpenType cross-table validation note 890" }

/* MORE_REF_891 */
pub fn ref_note_891() -> &'static str { "OpenType cross-table validation note 891" }

/* MORE_REF_892 */
pub fn ref_note_892() -> &'static str { "OpenType cross-table validation note 892" }

/* MORE_REF_893 */
pub fn ref_note_893() -> &'static str { "OpenType cross-table validation note 893" }

/* MORE_REF_894 */
pub fn ref_note_894() -> &'static str { "OpenType cross-table validation note 894" }

/* MORE_REF_895 */
pub fn ref_note_895() -> &'static str { "OpenType cross-table validation note 895" }

/* MORE_REF_896 */
pub fn ref_note_896() -> &'static str { "OpenType cross-table validation note 896" }

/* MORE_REF_897 */
pub fn ref_note_897() -> &'static str { "OpenType cross-table validation note 897" }

/* MORE_REF_898 */
pub fn ref_note_898() -> &'static str { "OpenType cross-table validation note 898" }

/* MORE_REF_899 */
pub fn ref_note_899() -> &'static str { "OpenType cross-table validation note 899" }

/* MORE_REF_900 */
pub fn ref_note_900() -> &'static str { "OpenType cross-table validation note 900" }

/* MORE_REF_901 */
pub fn ref_note_901() -> &'static str { "OpenType cross-table validation note 901" }

/* MORE_REF_902 */
pub fn ref_note_902() -> &'static str { "OpenType cross-table validation note 902" }

/* MORE_REF_903 */
pub fn ref_note_903() -> &'static str { "OpenType cross-table validation note 903" }

/* MORE_REF_904 */
pub fn ref_note_904() -> &'static str { "OpenType cross-table validation note 904" }

/* MORE_REF_905 */
pub fn ref_note_905() -> &'static str { "OpenType cross-table validation note 905" }

/* MORE_REF_906 */
pub fn ref_note_906() -> &'static str { "OpenType cross-table validation note 906" }

/* MORE_REF_907 */
pub fn ref_note_907() -> &'static str { "OpenType cross-table validation note 907" }

/* MORE_REF_908 */
pub fn ref_note_908() -> &'static str { "OpenType cross-table validation note 908" }

/* MORE_REF_909 */
pub fn ref_note_909() -> &'static str { "OpenType cross-table validation note 909" }

/* MORE_REF_910 */
pub fn ref_note_910() -> &'static str { "OpenType cross-table validation note 910" }

/* MORE_REF_911 */
pub fn ref_note_911() -> &'static str { "OpenType cross-table validation note 911" }

/* MORE_REF_912 */
pub fn ref_note_912() -> &'static str { "OpenType cross-table validation note 912" }

/* MORE_REF_913 */
pub fn ref_note_913() -> &'static str { "OpenType cross-table validation note 913" }

/* MORE_REF_914 */
pub fn ref_note_914() -> &'static str { "OpenType cross-table validation note 914" }

/* MORE_REF_915 */
pub fn ref_note_915() -> &'static str { "OpenType cross-table validation note 915" }

/* MORE_REF_916 */
pub fn ref_note_916() -> &'static str { "OpenType cross-table validation note 916" }

/* MORE_REF_917 */
pub fn ref_note_917() -> &'static str { "OpenType cross-table validation note 917" }

/* MORE_REF_918 */
pub fn ref_note_918() -> &'static str { "OpenType cross-table validation note 918" }

/* MORE_REF_919 */
pub fn ref_note_919() -> &'static str { "OpenType cross-table validation note 919" }

/* MORE_REF_920 */
pub fn ref_note_920() -> &'static str { "OpenType cross-table validation note 920" }

/* MORE_REF_921 */
pub fn ref_note_921() -> &'static str { "OpenType cross-table validation note 921" }

/* MORE_REF_922 */
pub fn ref_note_922() -> &'static str { "OpenType cross-table validation note 922" }

/* MORE_REF_923 */
pub fn ref_note_923() -> &'static str { "OpenType cross-table validation note 923" }

/* MORE_REF_924 */
pub fn ref_note_924() -> &'static str { "OpenType cross-table validation note 924" }

/* MORE_REF_925 */
pub fn ref_note_925() -> &'static str { "OpenType cross-table validation note 925" }

/* MORE_REF_926 */
pub fn ref_note_926() -> &'static str { "OpenType cross-table validation note 926" }

/* MORE_REF_927 */
pub fn ref_note_927() -> &'static str { "OpenType cross-table validation note 927" }

/* MORE_REF_928 */
pub fn ref_note_928() -> &'static str { "OpenType cross-table validation note 928" }

/* MORE_REF_929 */
pub fn ref_note_929() -> &'static str { "OpenType cross-table validation note 929" }

/* MORE_REF_930 */
pub fn ref_note_930() -> &'static str { "OpenType cross-table validation note 930" }

/* MORE_REF_931 */
pub fn ref_note_931() -> &'static str { "OpenType cross-table validation note 931" }

/* MORE_REF_932 */
pub fn ref_note_932() -> &'static str { "OpenType cross-table validation note 932" }

/* MORE_REF_933 */
pub fn ref_note_933() -> &'static str { "OpenType cross-table validation note 933" }

/* MORE_REF_934 */
pub fn ref_note_934() -> &'static str { "OpenType cross-table validation note 934" }

/* MORE_REF_935 */
pub fn ref_note_935() -> &'static str { "OpenType cross-table validation note 935" }

/* MORE_REF_936 */
pub fn ref_note_936() -> &'static str { "OpenType cross-table validation note 936" }

/* MORE_REF_937 */
pub fn ref_note_937() -> &'static str { "OpenType cross-table validation note 937" }

/* MORE_REF_938 */
pub fn ref_note_938() -> &'static str { "OpenType cross-table validation note 938" }

/* MORE_REF_939 */
pub fn ref_note_939() -> &'static str { "OpenType cross-table validation note 939" }

/* MORE_REF_940 */
pub fn ref_note_940() -> &'static str { "OpenType cross-table validation note 940" }

/* MORE_REF_941 */
pub fn ref_note_941() -> &'static str { "OpenType cross-table validation note 941" }

/* MORE_REF_942 */
pub fn ref_note_942() -> &'static str { "OpenType cross-table validation note 942" }

/* MORE_REF_943 */
pub fn ref_note_943() -> &'static str { "OpenType cross-table validation note 943" }

/* MORE_REF_944 */
pub fn ref_note_944() -> &'static str { "OpenType cross-table validation note 944" }

/* MORE_REF_945 */
pub fn ref_note_945() -> &'static str { "OpenType cross-table validation note 945" }

/* MORE_REF_946 */
pub fn ref_note_946() -> &'static str { "OpenType cross-table validation note 946" }

/* MORE_REF_947 */
pub fn ref_note_947() -> &'static str { "OpenType cross-table validation note 947" }

/* MORE_REF_948 */
pub fn ref_note_948() -> &'static str { "OpenType cross-table validation note 948" }

/* MORE_REF_949 */
pub fn ref_note_949() -> &'static str { "OpenType cross-table validation note 949" }

/* MORE_REF_950 */
pub fn ref_note_950() -> &'static str { "OpenType cross-table validation note 950" }

/* MORE_REF_951 */
pub fn ref_note_951() -> &'static str { "OpenType cross-table validation note 951" }

/* MORE_REF_952 */
pub fn ref_note_952() -> &'static str { "OpenType cross-table validation note 952" }

/* MORE_REF_953 */
pub fn ref_note_953() -> &'static str { "OpenType cross-table validation note 953" }

/* MORE_REF_954 */
pub fn ref_note_954() -> &'static str { "OpenType cross-table validation note 954" }

/* MORE_REF_955 */
pub fn ref_note_955() -> &'static str { "OpenType cross-table validation note 955" }

/* MORE_REF_956 */
pub fn ref_note_956() -> &'static str { "OpenType cross-table validation note 956" }

/* MORE_REF_957 */
pub fn ref_note_957() -> &'static str { "OpenType cross-table validation note 957" }

/* MORE_REF_958 */
pub fn ref_note_958() -> &'static str { "OpenType cross-table validation note 958" }

/* MORE_REF_959 */
pub fn ref_note_959() -> &'static str { "OpenType cross-table validation note 959" }

/* MORE_REF_960 */
pub fn ref_note_960() -> &'static str { "OpenType cross-table validation note 960" }

/* MORE_REF_961 */
pub fn ref_note_961() -> &'static str { "OpenType cross-table validation note 961" }

/* MORE_REF_962 */
pub fn ref_note_962() -> &'static str { "OpenType cross-table validation note 962" }

/* MORE_REF_963 */
pub fn ref_note_963() -> &'static str { "OpenType cross-table validation note 963" }

/* MORE_REF_964 */
pub fn ref_note_964() -> &'static str { "OpenType cross-table validation note 964" }

/* MORE_REF_965 */
pub fn ref_note_965() -> &'static str { "OpenType cross-table validation note 965" }

/* MORE_REF_966 */
pub fn ref_note_966() -> &'static str { "OpenType cross-table validation note 966" }

/* MORE_REF_967 */
pub fn ref_note_967() -> &'static str { "OpenType cross-table validation note 967" }

/* MORE_REF_968 */
pub fn ref_note_968() -> &'static str { "OpenType cross-table validation note 968" }

/* MORE_REF_969 */
pub fn ref_note_969() -> &'static str { "OpenType cross-table validation note 969" }

/* MORE_REF_970 */
pub fn ref_note_970() -> &'static str { "OpenType cross-table validation note 970" }

/* MORE_REF_971 */
pub fn ref_note_971() -> &'static str { "OpenType cross-table validation note 971" }

/* MORE_REF_972 */
pub fn ref_note_972() -> &'static str { "OpenType cross-table validation note 972" }

/* MORE_REF_973 */
pub fn ref_note_973() -> &'static str { "OpenType cross-table validation note 973" }

/* MORE_REF_974 */
pub fn ref_note_974() -> &'static str { "OpenType cross-table validation note 974" }

/* MORE_REF_975 */
pub fn ref_note_975() -> &'static str { "OpenType cross-table validation note 975" }

/* MORE_REF_976 */
pub fn ref_note_976() -> &'static str { "OpenType cross-table validation note 976" }

/* MORE_REF_977 */
pub fn ref_note_977() -> &'static str { "OpenType cross-table validation note 977" }

/* MORE_REF_978 */
pub fn ref_note_978() -> &'static str { "OpenType cross-table validation note 978" }

/* MORE_REF_979 */
pub fn ref_note_979() -> &'static str { "OpenType cross-table validation note 979" }

/* MORE_REF_980 */
pub fn ref_note_980() -> &'static str { "OpenType cross-table validation note 980" }

/* MORE_REF_981 */
pub fn ref_note_981() -> &'static str { "OpenType cross-table validation note 981" }

/* MORE_REF_982 */
pub fn ref_note_982() -> &'static str { "OpenType cross-table validation note 982" }

/* MORE_REF_983 */
pub fn ref_note_983() -> &'static str { "OpenType cross-table validation note 983" }

/* MORE_REF_984 */
pub fn ref_note_984() -> &'static str { "OpenType cross-table validation note 984" }

/* MORE_REF_985 */
pub fn ref_note_985() -> &'static str { "OpenType cross-table validation note 985" }

/* MORE_REF_986 */
pub fn ref_note_986() -> &'static str { "OpenType cross-table validation note 986" }

/* MORE_REF_987 */
pub fn ref_note_987() -> &'static str { "OpenType cross-table validation note 987" }

/* MORE_REF_988 */
pub fn ref_note_988() -> &'static str { "OpenType cross-table validation note 988" }

/* MORE_REF_989 */
pub fn ref_note_989() -> &'static str { "OpenType cross-table validation note 989" }

/* MORE_REF_990 */
pub fn ref_note_990() -> &'static str { "OpenType cross-table validation note 990" }

/* MORE_REF_991 */
pub fn ref_note_991() -> &'static str { "OpenType cross-table validation note 991" }

/* MORE_REF_992 */
pub fn ref_note_992() -> &'static str { "OpenType cross-table validation note 992" }

/* MORE_REF_993 */
pub fn ref_note_993() -> &'static str { "OpenType cross-table validation note 993" }

/* MORE_REF_994 */
pub fn ref_note_994() -> &'static str { "OpenType cross-table validation note 994" }

/* MORE_REF_995 */
pub fn ref_note_995() -> &'static str { "OpenType cross-table validation note 995" }

/* MORE_REF_996 */
pub fn ref_note_996() -> &'static str { "OpenType cross-table validation note 996" }

/* MORE_REF_997 */
pub fn ref_note_997() -> &'static str { "OpenType cross-table validation note 997" }

/* MORE_REF_998 */
pub fn ref_note_998() -> &'static str { "OpenType cross-table validation note 998" }

/* MORE_REF_999 */
pub fn ref_note_999() -> &'static str { "OpenType cross-table validation note 999" }

/* MORE_REF_1000 */
pub fn ref_note_1000() -> &'static str { "OpenType cross-table validation note 1000" }

/* MORE_REF_1001 */
pub fn ref_note_1001() -> &'static str { "OpenType cross-table validation note 1001" }

/* MORE_REF_1002 */
pub fn ref_note_1002() -> &'static str { "OpenType cross-table validation note 1002" }

/* MORE_REF_1003 */
pub fn ref_note_1003() -> &'static str { "OpenType cross-table validation note 1003" }

/* MORE_REF_1004 */
pub fn ref_note_1004() -> &'static str { "OpenType cross-table validation note 1004" }

/* MORE_REF_1005 */
pub fn ref_note_1005() -> &'static str { "OpenType cross-table validation note 1005" }

/* MORE_REF_1006 */
pub fn ref_note_1006() -> &'static str { "OpenType cross-table validation note 1006" }

/* MORE_REF_1007 */
pub fn ref_note_1007() -> &'static str { "OpenType cross-table validation note 1007" }

/* MORE_REF_1008 */
pub fn ref_note_1008() -> &'static str { "OpenType cross-table validation note 1008" }

/* MORE_REF_1009 */
pub fn ref_note_1009() -> &'static str { "OpenType cross-table validation note 1009" }

/* MORE_REF_1010 */
pub fn ref_note_1010() -> &'static str { "OpenType cross-table validation note 1010" }

/* MORE_REF_1011 */
pub fn ref_note_1011() -> &'static str { "OpenType cross-table validation note 1011" }

/* MORE_REF_1012 */
pub fn ref_note_1012() -> &'static str { "OpenType cross-table validation note 1012" }

/* MORE_REF_1013 */
pub fn ref_note_1013() -> &'static str { "OpenType cross-table validation note 1013" }

/* MORE_REF_1014 */
pub fn ref_note_1014() -> &'static str { "OpenType cross-table validation note 1014" }

/* MORE_REF_1015 */
pub fn ref_note_1015() -> &'static str { "OpenType cross-table validation note 1015" }

/* MORE_REF_1016 */
pub fn ref_note_1016() -> &'static str { "OpenType cross-table validation note 1016" }

/* MORE_REF_1017 */
pub fn ref_note_1017() -> &'static str { "OpenType cross-table validation note 1017" }

/* MORE_REF_1018 */
pub fn ref_note_1018() -> &'static str { "OpenType cross-table validation note 1018" }

/* MORE_REF_1019 */
pub fn ref_note_1019() -> &'static str { "OpenType cross-table validation note 1019" }

/* MORE_REF_1020 */
pub fn ref_note_1020() -> &'static str { "OpenType cross-table validation note 1020" }

/* MORE_REF_1021 */
pub fn ref_note_1021() -> &'static str { "OpenType cross-table validation note 1021" }

/* MORE_REF_1022 */
pub fn ref_note_1022() -> &'static str { "OpenType cross-table validation note 1022" }

/* MORE_REF_1023 */
pub fn ref_note_1023() -> &'static str { "OpenType cross-table validation note 1023" }

/* MORE_REF_1024 */
pub fn ref_note_1024() -> &'static str { "OpenType cross-table validation note 1024" }

/* MORE_REF_1025 */
pub fn ref_note_1025() -> &'static str { "OpenType cross-table validation note 1025" }

/* MORE_REF_1026 */
pub fn ref_note_1026() -> &'static str { "OpenType cross-table validation note 1026" }

/* MORE_REF_1027 */
pub fn ref_note_1027() -> &'static str { "OpenType cross-table validation note 1027" }

/* MORE_REF_1028 */
pub fn ref_note_1028() -> &'static str { "OpenType cross-table validation note 1028" }

/* MORE_REF_1029 */
pub fn ref_note_1029() -> &'static str { "OpenType cross-table validation note 1029" }

/* MORE_REF_1030 */
pub fn ref_note_1030() -> &'static str { "OpenType cross-table validation note 1030" }

/* MORE_REF_1031 */
pub fn ref_note_1031() -> &'static str { "OpenType cross-table validation note 1031" }

/* MORE_REF_1032 */
pub fn ref_note_1032() -> &'static str { "OpenType cross-table validation note 1032" }

/* MORE_REF_1033 */
pub fn ref_note_1033() -> &'static str { "OpenType cross-table validation note 1033" }

/* MORE_REF_1034 */
pub fn ref_note_1034() -> &'static str { "OpenType cross-table validation note 1034" }

/* MORE_REF_1035 */
pub fn ref_note_1035() -> &'static str { "OpenType cross-table validation note 1035" }

/* MORE_REF_1036 */
pub fn ref_note_1036() -> &'static str { "OpenType cross-table validation note 1036" }

/* MORE_REF_1037 */
pub fn ref_note_1037() -> &'static str { "OpenType cross-table validation note 1037" }

/* MORE_REF_1038 */
pub fn ref_note_1038() -> &'static str { "OpenType cross-table validation note 1038" }

/* MORE_REF_1039 */
pub fn ref_note_1039() -> &'static str { "OpenType cross-table validation note 1039" }

/* MORE_REF_1040 */
pub fn ref_note_1040() -> &'static str { "OpenType cross-table validation note 1040" }

/* MORE_REF_1041 */
pub fn ref_note_1041() -> &'static str { "OpenType cross-table validation note 1041" }

/* MORE_REF_1042 */
pub fn ref_note_1042() -> &'static str { "OpenType cross-table validation note 1042" }

/* MORE_REF_1043 */
pub fn ref_note_1043() -> &'static str { "OpenType cross-table validation note 1043" }

/* MORE_REF_1044 */
pub fn ref_note_1044() -> &'static str { "OpenType cross-table validation note 1044" }

/* MORE_REF_1045 */
pub fn ref_note_1045() -> &'static str { "OpenType cross-table validation note 1045" }

/* MORE_REF_1046 */
pub fn ref_note_1046() -> &'static str { "OpenType cross-table validation note 1046" }

/* MORE_REF_1047 */
pub fn ref_note_1047() -> &'static str { "OpenType cross-table validation note 1047" }

/* MORE_REF_1048 */
pub fn ref_note_1048() -> &'static str { "OpenType cross-table validation note 1048" }

/* MORE_REF_1049 */
pub fn ref_note_1049() -> &'static str { "OpenType cross-table validation note 1049" }

/* MORE_REF_1050 */
pub fn ref_note_1050() -> &'static str { "OpenType cross-table validation note 1050" }

/* MORE_REF_1051 */
pub fn ref_note_1051() -> &'static str { "OpenType cross-table validation note 1051" }

/* MORE_REF_1052 */
pub fn ref_note_1052() -> &'static str { "OpenType cross-table validation note 1052" }

/* MORE_REF_1053 */
pub fn ref_note_1053() -> &'static str { "OpenType cross-table validation note 1053" }

/* MORE_REF_1054 */
pub fn ref_note_1054() -> &'static str { "OpenType cross-table validation note 1054" }

/* MORE_REF_1055 */
pub fn ref_note_1055() -> &'static str { "OpenType cross-table validation note 1055" }

/* MORE_REF_1056 */
pub fn ref_note_1056() -> &'static str { "OpenType cross-table validation note 1056" }

/* MORE_REF_1057 */
pub fn ref_note_1057() -> &'static str { "OpenType cross-table validation note 1057" }

/* MORE_REF_1058 */
pub fn ref_note_1058() -> &'static str { "OpenType cross-table validation note 1058" }

/* MORE_REF_1059 */
pub fn ref_note_1059() -> &'static str { "OpenType cross-table validation note 1059" }

/* MORE_REF_1060 */
pub fn ref_note_1060() -> &'static str { "OpenType cross-table validation note 1060" }

/* MORE_REF_1061 */
pub fn ref_note_1061() -> &'static str { "OpenType cross-table validation note 1061" }

/* MORE_REF_1062 */
pub fn ref_note_1062() -> &'static str { "OpenType cross-table validation note 1062" }

/* MORE_REF_1063 */
pub fn ref_note_1063() -> &'static str { "OpenType cross-table validation note 1063" }

/* MORE_REF_1064 */
pub fn ref_note_1064() -> &'static str { "OpenType cross-table validation note 1064" }

/* MORE_REF_1065 */
pub fn ref_note_1065() -> &'static str { "OpenType cross-table validation note 1065" }

/* MORE_REF_1066 */
pub fn ref_note_1066() -> &'static str { "OpenType cross-table validation note 1066" }

/* MORE_REF_1067 */
pub fn ref_note_1067() -> &'static str { "OpenType cross-table validation note 1067" }

/* MORE_REF_1068 */
pub fn ref_note_1068() -> &'static str { "OpenType cross-table validation note 1068" }

/* MORE_REF_1069 */
pub fn ref_note_1069() -> &'static str { "OpenType cross-table validation note 1069" }

/* MORE_REF_1070 */
pub fn ref_note_1070() -> &'static str { "OpenType cross-table validation note 1070" }

/* MORE_REF_1071 */
pub fn ref_note_1071() -> &'static str { "OpenType cross-table validation note 1071" }

/* MORE_REF_1072 */
pub fn ref_note_1072() -> &'static str { "OpenType cross-table validation note 1072" }

/* MORE_REF_1073 */
pub fn ref_note_1073() -> &'static str { "OpenType cross-table validation note 1073" }

/* MORE_REF_1074 */
pub fn ref_note_1074() -> &'static str { "OpenType cross-table validation note 1074" }

/* MORE_REF_1075 */
pub fn ref_note_1075() -> &'static str { "OpenType cross-table validation note 1075" }

/* MORE_REF_1076 */
pub fn ref_note_1076() -> &'static str { "OpenType cross-table validation note 1076" }

/* MORE_REF_1077 */
pub fn ref_note_1077() -> &'static str { "OpenType cross-table validation note 1077" }

/* MORE_REF_1078 */
pub fn ref_note_1078() -> &'static str { "OpenType cross-table validation note 1078" }

/* MORE_REF_1079 */
pub fn ref_note_1079() -> &'static str { "OpenType cross-table validation note 1079" }

/* MORE_REF_1080 */
pub fn ref_note_1080() -> &'static str { "OpenType cross-table validation note 1080" }

/* MORE_REF_1081 */
pub fn ref_note_1081() -> &'static str { "OpenType cross-table validation note 1081" }

/* MORE_REF_1082 */
pub fn ref_note_1082() -> &'static str { "OpenType cross-table validation note 1082" }

/* MORE_REF_1083 */
pub fn ref_note_1083() -> &'static str { "OpenType cross-table validation note 1083" }

/* MORE_REF_1084 */
pub fn ref_note_1084() -> &'static str { "OpenType cross-table validation note 1084" }

/* MORE_REF_1085 */
pub fn ref_note_1085() -> &'static str { "OpenType cross-table validation note 1085" }

/* MORE_REF_1086 */
pub fn ref_note_1086() -> &'static str { "OpenType cross-table validation note 1086" }

/* MORE_REF_1087 */
pub fn ref_note_1087() -> &'static str { "OpenType cross-table validation note 1087" }

/* MORE_REF_1088 */
pub fn ref_note_1088() -> &'static str { "OpenType cross-table validation note 1088" }

/* MORE_REF_1089 */
pub fn ref_note_1089() -> &'static str { "OpenType cross-table validation note 1089" }

/* MORE_REF_1090 */
pub fn ref_note_1090() -> &'static str { "OpenType cross-table validation note 1090" }

/* MORE_REF_1091 */
pub fn ref_note_1091() -> &'static str { "OpenType cross-table validation note 1091" }

/* MORE_REF_1092 */
pub fn ref_note_1092() -> &'static str { "OpenType cross-table validation note 1092" }

/* MORE_REF_1093 */
pub fn ref_note_1093() -> &'static str { "OpenType cross-table validation note 1093" }

/* MORE_REF_1094 */
pub fn ref_note_1094() -> &'static str { "OpenType cross-table validation note 1094" }

/* MORE_REF_1095 */
pub fn ref_note_1095() -> &'static str { "OpenType cross-table validation note 1095" }

/* MORE_REF_1096 */
pub fn ref_note_1096() -> &'static str { "OpenType cross-table validation note 1096" }

/* MORE_REF_1097 */
pub fn ref_note_1097() -> &'static str { "OpenType cross-table validation note 1097" }

/* MORE_REF_1098 */
pub fn ref_note_1098() -> &'static str { "OpenType cross-table validation note 1098" }

/* MORE_REF_1099 */
pub fn ref_note_1099() -> &'static str { "OpenType cross-table validation note 1099" }

/* MORE_REF_1100 */
pub fn ref_note_1100() -> &'static str { "OpenType cross-table validation note 1100" }

/* MORE_REF_1101 */
pub fn ref_note_1101() -> &'static str { "OpenType cross-table validation note 1101" }

/* MORE_REF_1102 */
pub fn ref_note_1102() -> &'static str { "OpenType cross-table validation note 1102" }

/* MORE_REF_1103 */
pub fn ref_note_1103() -> &'static str { "OpenType cross-table validation note 1103" }

/* MORE_REF_1104 */
pub fn ref_note_1104() -> &'static str { "OpenType cross-table validation note 1104" }

/* MORE_REF_1105 */
pub fn ref_note_1105() -> &'static str { "OpenType cross-table validation note 1105" }

/* MORE_REF_1106 */
pub fn ref_note_1106() -> &'static str { "OpenType cross-table validation note 1106" }

/* MORE_REF_1107 */
pub fn ref_note_1107() -> &'static str { "OpenType cross-table validation note 1107" }

/* MORE_REF_1108 */
pub fn ref_note_1108() -> &'static str { "OpenType cross-table validation note 1108" }

/* MORE_REF_1109 */
pub fn ref_note_1109() -> &'static str { "OpenType cross-table validation note 1109" }

/* MORE_REF_1110 */
pub fn ref_note_1110() -> &'static str { "OpenType cross-table validation note 1110" }

/* MORE_REF_1111 */
pub fn ref_note_1111() -> &'static str { "OpenType cross-table validation note 1111" }

/* MORE_REF_1112 */
pub fn ref_note_1112() -> &'static str { "OpenType cross-table validation note 1112" }

/* MORE_REF_1113 */
pub fn ref_note_1113() -> &'static str { "OpenType cross-table validation note 1113" }

/* MORE_REF_1114 */
pub fn ref_note_1114() -> &'static str { "OpenType cross-table validation note 1114" }

/* MORE_REF_1115 */
pub fn ref_note_1115() -> &'static str { "OpenType cross-table validation note 1115" }

/* MORE_REF_1116 */
pub fn ref_note_1116() -> &'static str { "OpenType cross-table validation note 1116" }

/* MORE_REF_1117 */
pub fn ref_note_1117() -> &'static str { "OpenType cross-table validation note 1117" }

/* MORE_REF_1118 */
pub fn ref_note_1118() -> &'static str { "OpenType cross-table validation note 1118" }

/* MORE_REF_1119 */
pub fn ref_note_1119() -> &'static str { "OpenType cross-table validation note 1119" }

/* MORE_REF_1120 */
pub fn ref_note_1120() -> &'static str { "OpenType cross-table validation note 1120" }

/* MORE_REF_1121 */
pub fn ref_note_1121() -> &'static str { "OpenType cross-table validation note 1121" }

/* MORE_REF_1122 */
pub fn ref_note_1122() -> &'static str { "OpenType cross-table validation note 1122" }

/* MORE_REF_1123 */
pub fn ref_note_1123() -> &'static str { "OpenType cross-table validation note 1123" }

/* MORE_REF_1124 */
pub fn ref_note_1124() -> &'static str { "OpenType cross-table validation note 1124" }

/* MORE_REF_1125 */
pub fn ref_note_1125() -> &'static str { "OpenType cross-table validation note 1125" }

/* MORE_REF_1126 */
pub fn ref_note_1126() -> &'static str { "OpenType cross-table validation note 1126" }

/* MORE_REF_1127 */
pub fn ref_note_1127() -> &'static str { "OpenType cross-table validation note 1127" }

/* MORE_REF_1128 */
pub fn ref_note_1128() -> &'static str { "OpenType cross-table validation note 1128" }

/* MORE_REF_1129 */
pub fn ref_note_1129() -> &'static str { "OpenType cross-table validation note 1129" }

/* MORE_REF_1130 */
pub fn ref_note_1130() -> &'static str { "OpenType cross-table validation note 1130" }

/* MORE_REF_1131 */
pub fn ref_note_1131() -> &'static str { "OpenType cross-table validation note 1131" }

/* MORE_REF_1132 */
pub fn ref_note_1132() -> &'static str { "OpenType cross-table validation note 1132" }

/* MORE_REF_1133 */
pub fn ref_note_1133() -> &'static str { "OpenType cross-table validation note 1133" }

/* MORE_REF_1134 */
pub fn ref_note_1134() -> &'static str { "OpenType cross-table validation note 1134" }

/* MORE_REF_1135 */
pub fn ref_note_1135() -> &'static str { "OpenType cross-table validation note 1135" }

/* MORE_REF_1136 */
pub fn ref_note_1136() -> &'static str { "OpenType cross-table validation note 1136" }

/* MORE_REF_1137 */
pub fn ref_note_1137() -> &'static str { "OpenType cross-table validation note 1137" }

/* MORE_REF_1138 */
pub fn ref_note_1138() -> &'static str { "OpenType cross-table validation note 1138" }

/* MORE_REF_1139 */
pub fn ref_note_1139() -> &'static str { "OpenType cross-table validation note 1139" }

/* MORE_REF_1140 */
pub fn ref_note_1140() -> &'static str { "OpenType cross-table validation note 1140" }

/* MORE_REF_1141 */
pub fn ref_note_1141() -> &'static str { "OpenType cross-table validation note 1141" }

/* MORE_REF_1142 */
pub fn ref_note_1142() -> &'static str { "OpenType cross-table validation note 1142" }

/* MORE_REF_1143 */
pub fn ref_note_1143() -> &'static str { "OpenType cross-table validation note 1143" }

/* MORE_REF_1144 */
pub fn ref_note_1144() -> &'static str { "OpenType cross-table validation note 1144" }

/* MORE_REF_1145 */
pub fn ref_note_1145() -> &'static str { "OpenType cross-table validation note 1145" }

/* MORE_REF_1146 */
pub fn ref_note_1146() -> &'static str { "OpenType cross-table validation note 1146" }

/* MORE_REF_1147 */
pub fn ref_note_1147() -> &'static str { "OpenType cross-table validation note 1147" }

/* MORE_REF_1148 */
pub fn ref_note_1148() -> &'static str { "OpenType cross-table validation note 1148" }

/* MORE_REF_1149 */
pub fn ref_note_1149() -> &'static str { "OpenType cross-table validation note 1149" }

/* MORE_REF_1150 */
pub fn ref_note_1150() -> &'static str { "OpenType cross-table validation note 1150" }

/* MORE_REF_1151 */
pub fn ref_note_1151() -> &'static str { "OpenType cross-table validation note 1151" }

/* MORE_REF_1152 */
pub fn ref_note_1152() -> &'static str { "OpenType cross-table validation note 1152" }

/* MORE_REF_1153 */
pub fn ref_note_1153() -> &'static str { "OpenType cross-table validation note 1153" }

/* MORE_REF_1154 */
pub fn ref_note_1154() -> &'static str { "OpenType cross-table validation note 1154" }

/* MORE_REF_1155 */
pub fn ref_note_1155() -> &'static str { "OpenType cross-table validation note 1155" }

/* MORE_REF_1156 */
pub fn ref_note_1156() -> &'static str { "OpenType cross-table validation note 1156" }

/* MORE_REF_1157 */
pub fn ref_note_1157() -> &'static str { "OpenType cross-table validation note 1157" }

/* MORE_REF_1158 */
pub fn ref_note_1158() -> &'static str { "OpenType cross-table validation note 1158" }

/* MORE_REF_1159 */
pub fn ref_note_1159() -> &'static str { "OpenType cross-table validation note 1159" }

/* MORE_REF_1160 */
pub fn ref_note_1160() -> &'static str { "OpenType cross-table validation note 1160" }

/* MORE_REF_1161 */
pub fn ref_note_1161() -> &'static str { "OpenType cross-table validation note 1161" }

/* MORE_REF_1162 */
pub fn ref_note_1162() -> &'static str { "OpenType cross-table validation note 1162" }

/* MORE_REF_1163 */
pub fn ref_note_1163() -> &'static str { "OpenType cross-table validation note 1163" }

/* MORE_REF_1164 */
pub fn ref_note_1164() -> &'static str { "OpenType cross-table validation note 1164" }

/* MORE_REF_1165 */
pub fn ref_note_1165() -> &'static str { "OpenType cross-table validation note 1165" }

/* MORE_REF_1166 */
pub fn ref_note_1166() -> &'static str { "OpenType cross-table validation note 1166" }

/* MORE_REF_1167 */
pub fn ref_note_1167() -> &'static str { "OpenType cross-table validation note 1167" }

/* MORE_REF_1168 */
pub fn ref_note_1168() -> &'static str { "OpenType cross-table validation note 1168" }

/* MORE_REF_1169 */
pub fn ref_note_1169() -> &'static str { "OpenType cross-table validation note 1169" }

/* MORE_REF_1170 */
pub fn ref_note_1170() -> &'static str { "OpenType cross-table validation note 1170" }

/* MORE_REF_1171 */
pub fn ref_note_1171() -> &'static str { "OpenType cross-table validation note 1171" }

/* MORE_REF_1172 */
pub fn ref_note_1172() -> &'static str { "OpenType cross-table validation note 1172" }

/* MORE_REF_1173 */
pub fn ref_note_1173() -> &'static str { "OpenType cross-table validation note 1173" }

/* MORE_REF_1174 */
pub fn ref_note_1174() -> &'static str { "OpenType cross-table validation note 1174" }

/* MORE_REF_1175 */
pub fn ref_note_1175() -> &'static str { "OpenType cross-table validation note 1175" }

/* MORE_REF_1176 */
pub fn ref_note_1176() -> &'static str { "OpenType cross-table validation note 1176" }

/* MORE_REF_1177 */
pub fn ref_note_1177() -> &'static str { "OpenType cross-table validation note 1177" }

/* MORE_REF_1178 */
pub fn ref_note_1178() -> &'static str { "OpenType cross-table validation note 1178" }

/* MORE_REF_1179 */
pub fn ref_note_1179() -> &'static str { "OpenType cross-table validation note 1179" }

/* MORE_REF_1180 */
pub fn ref_note_1180() -> &'static str { "OpenType cross-table validation note 1180" }

/* MORE_REF_1181 */
pub fn ref_note_1181() -> &'static str { "OpenType cross-table validation note 1181" }

/* MORE_REF_1182 */
pub fn ref_note_1182() -> &'static str { "OpenType cross-table validation note 1182" }

/* MORE_REF_1183 */
pub fn ref_note_1183() -> &'static str { "OpenType cross-table validation note 1183" }

/* MORE_REF_1184 */
pub fn ref_note_1184() -> &'static str { "OpenType cross-table validation note 1184" }

/* MORE_REF_1185 */
pub fn ref_note_1185() -> &'static str { "OpenType cross-table validation note 1185" }

/* MORE_REF_1186 */
pub fn ref_note_1186() -> &'static str { "OpenType cross-table validation note 1186" }

/* MORE_REF_1187 */
pub fn ref_note_1187() -> &'static str { "OpenType cross-table validation note 1187" }

/* MORE_REF_1188 */
pub fn ref_note_1188() -> &'static str { "OpenType cross-table validation note 1188" }

/* MORE_REF_1189 */
pub fn ref_note_1189() -> &'static str { "OpenType cross-table validation note 1189" }

/* MORE_REF_1190 */
pub fn ref_note_1190() -> &'static str { "OpenType cross-table validation note 1190" }

/* MORE_REF_1191 */
pub fn ref_note_1191() -> &'static str { "OpenType cross-table validation note 1191" }

/* MORE_REF_1192 */
pub fn ref_note_1192() -> &'static str { "OpenType cross-table validation note 1192" }

/* MORE_REF_1193 */
pub fn ref_note_1193() -> &'static str { "OpenType cross-table validation note 1193" }

/* MORE_REF_1194 */
pub fn ref_note_1194() -> &'static str { "OpenType cross-table validation note 1194" }

/* MORE_REF_1195 */
pub fn ref_note_1195() -> &'static str { "OpenType cross-table validation note 1195" }

/* MORE_REF_1196 */
pub fn ref_note_1196() -> &'static str { "OpenType cross-table validation note 1196" }

/* MORE_REF_1197 */
pub fn ref_note_1197() -> &'static str { "OpenType cross-table validation note 1197" }

/* MORE_REF_1198 */
pub fn ref_note_1198() -> &'static str { "OpenType cross-table validation note 1198" }

/* MORE_REF_1199 */
pub fn ref_note_1199() -> &'static str { "OpenType cross-table validation note 1199" }

/* MORE_REF_1200 */
pub fn ref_note_1200() -> &'static str { "OpenType cross-table validation note 1200" }

/* MORE_REF_1201 */
pub fn ref_note_1201() -> &'static str { "OpenType cross-table validation note 1201" }

/* MORE_REF_1202 */
pub fn ref_note_1202() -> &'static str { "OpenType cross-table validation note 1202" }

/* MORE_REF_1203 */
pub fn ref_note_1203() -> &'static str { "OpenType cross-table validation note 1203" }

/* MORE_REF_1204 */
pub fn ref_note_1204() -> &'static str { "OpenType cross-table validation note 1204" }

/* MORE_REF_1205 */
pub fn ref_note_1205() -> &'static str { "OpenType cross-table validation note 1205" }

/* MORE_REF_1206 */
pub fn ref_note_1206() -> &'static str { "OpenType cross-table validation note 1206" }

/* MORE_REF_1207 */
pub fn ref_note_1207() -> &'static str { "OpenType cross-table validation note 1207" }

/* MORE_REF_1208 */
pub fn ref_note_1208() -> &'static str { "OpenType cross-table validation note 1208" }

/* MORE_REF_1209 */
pub fn ref_note_1209() -> &'static str { "OpenType cross-table validation note 1209" }

/* MORE_REF_1210 */
pub fn ref_note_1210() -> &'static str { "OpenType cross-table validation note 1210" }

/* MORE_REF_1211 */
pub fn ref_note_1211() -> &'static str { "OpenType cross-table validation note 1211" }

/* MORE_REF_1212 */
pub fn ref_note_1212() -> &'static str { "OpenType cross-table validation note 1212" }

/* MORE_REF_1213 */
pub fn ref_note_1213() -> &'static str { "OpenType cross-table validation note 1213" }

/* MORE_REF_1214 */
pub fn ref_note_1214() -> &'static str { "OpenType cross-table validation note 1214" }

/* MORE_REF_1215 */
pub fn ref_note_1215() -> &'static str { "OpenType cross-table validation note 1215" }

/* MORE_REF_1216 */
pub fn ref_note_1216() -> &'static str { "OpenType cross-table validation note 1216" }

/* MORE_REF_1217 */
pub fn ref_note_1217() -> &'static str { "OpenType cross-table validation note 1217" }

/* MORE_REF_1218 */
pub fn ref_note_1218() -> &'static str { "OpenType cross-table validation note 1218" }

/* MORE_REF_1219 */
pub fn ref_note_1219() -> &'static str { "OpenType cross-table validation note 1219" }

/* MORE_REF_1220 */
pub fn ref_note_1220() -> &'static str { "OpenType cross-table validation note 1220" }

/* MORE_REF_1221 */
pub fn ref_note_1221() -> &'static str { "OpenType cross-table validation note 1221" }

/* MORE_REF_1222 */
pub fn ref_note_1222() -> &'static str { "OpenType cross-table validation note 1222" }

/* MORE_REF_1223 */
pub fn ref_note_1223() -> &'static str { "OpenType cross-table validation note 1223" }

/* MORE_REF_1224 */
pub fn ref_note_1224() -> &'static str { "OpenType cross-table validation note 1224" }

/* MORE_REF_1225 */
pub fn ref_note_1225() -> &'static str { "OpenType cross-table validation note 1225" }

/* MORE_REF_1226 */
pub fn ref_note_1226() -> &'static str { "OpenType cross-table validation note 1226" }

/* MORE_REF_1227 */
pub fn ref_note_1227() -> &'static str { "OpenType cross-table validation note 1227" }

/* MORE_REF_1228 */
pub fn ref_note_1228() -> &'static str { "OpenType cross-table validation note 1228" }

/* MORE_REF_1229 */
pub fn ref_note_1229() -> &'static str { "OpenType cross-table validation note 1229" }

/* MORE_REF_1230 */
pub fn ref_note_1230() -> &'static str { "OpenType cross-table validation note 1230" }

/* MORE_REF_1231 */
pub fn ref_note_1231() -> &'static str { "OpenType cross-table validation note 1231" }

/* MORE_REF_1232 */
pub fn ref_note_1232() -> &'static str { "OpenType cross-table validation note 1232" }

/* MORE_REF_1233 */
pub fn ref_note_1233() -> &'static str { "OpenType cross-table validation note 1233" }

/* MORE_REF_1234 */
pub fn ref_note_1234() -> &'static str { "OpenType cross-table validation note 1234" }

/* MORE_REF_1235 */
pub fn ref_note_1235() -> &'static str { "OpenType cross-table validation note 1235" }

/* MORE_REF_1236 */
pub fn ref_note_1236() -> &'static str { "OpenType cross-table validation note 1236" }

/* MORE_REF_1237 */
pub fn ref_note_1237() -> &'static str { "OpenType cross-table validation note 1237" }

/* MORE_REF_1238 */
pub fn ref_note_1238() -> &'static str { "OpenType cross-table validation note 1238" }

/* MORE_REF_1239 */
pub fn ref_note_1239() -> &'static str { "OpenType cross-table validation note 1239" }

/* MORE_REF_1240 */
pub fn ref_note_1240() -> &'static str { "OpenType cross-table validation note 1240" }

/* MORE_REF_1241 */
pub fn ref_note_1241() -> &'static str { "OpenType cross-table validation note 1241" }

/* MORE_REF_1242 */
pub fn ref_note_1242() -> &'static str { "OpenType cross-table validation note 1242" }

/* MORE_REF_1243 */
pub fn ref_note_1243() -> &'static str { "OpenType cross-table validation note 1243" }

/* MORE_REF_1244 */
pub fn ref_note_1244() -> &'static str { "OpenType cross-table validation note 1244" }

/* MORE_REF_1245 */
pub fn ref_note_1245() -> &'static str { "OpenType cross-table validation note 1245" }

/* MORE_REF_1246 */
pub fn ref_note_1246() -> &'static str { "OpenType cross-table validation note 1246" }

/* MORE_REF_1247 */
pub fn ref_note_1247() -> &'static str { "OpenType cross-table validation note 1247" }

/* MORE_REF_1248 */
pub fn ref_note_1248() -> &'static str { "OpenType cross-table validation note 1248" }

/* MORE_REF_1249 */
pub fn ref_note_1249() -> &'static str { "OpenType cross-table validation note 1249" }

/* MORE_REF_1250 */
pub fn ref_note_1250() -> &'static str { "OpenType cross-table validation note 1250" }

/* MORE_REF_1251 */
pub fn ref_note_1251() -> &'static str { "OpenType cross-table validation note 1251" }

/* MORE_REF_1252 */
pub fn ref_note_1252() -> &'static str { "OpenType cross-table validation note 1252" }

/* MORE_REF_1253 */
pub fn ref_note_1253() -> &'static str { "OpenType cross-table validation note 1253" }

/* MORE_REF_1254 */
pub fn ref_note_1254() -> &'static str { "OpenType cross-table validation note 1254" }

/* MORE_REF_1255 */
pub fn ref_note_1255() -> &'static str { "OpenType cross-table validation note 1255" }

/* MORE_REF_1256 */
pub fn ref_note_1256() -> &'static str { "OpenType cross-table validation note 1256" }

/* MORE_REF_1257 */
pub fn ref_note_1257() -> &'static str { "OpenType cross-table validation note 1257" }

/* MORE_REF_1258 */
pub fn ref_note_1258() -> &'static str { "OpenType cross-table validation note 1258" }

/* MORE_REF_1259 */
pub fn ref_note_1259() -> &'static str { "OpenType cross-table validation note 1259" }

/* MORE_REF_1260 */
pub fn ref_note_1260() -> &'static str { "OpenType cross-table validation note 1260" }

/* MORE_REF_1261 */
pub fn ref_note_1261() -> &'static str { "OpenType cross-table validation note 1261" }

/* MORE_REF_1262 */
pub fn ref_note_1262() -> &'static str { "OpenType cross-table validation note 1262" }

/* MORE_REF_1263 */
pub fn ref_note_1263() -> &'static str { "OpenType cross-table validation note 1263" }

/* MORE_REF_1264 */
pub fn ref_note_1264() -> &'static str { "OpenType cross-table validation note 1264" }

/* MORE_REF_1265 */
pub fn ref_note_1265() -> &'static str { "OpenType cross-table validation note 1265" }

/* MORE_REF_1266 */
pub fn ref_note_1266() -> &'static str { "OpenType cross-table validation note 1266" }

/* MORE_REF_1267 */
pub fn ref_note_1267() -> &'static str { "OpenType cross-table validation note 1267" }

/* MORE_REF_1268 */
pub fn ref_note_1268() -> &'static str { "OpenType cross-table validation note 1268" }

/* MORE_REF_1269 */
pub fn ref_note_1269() -> &'static str { "OpenType cross-table validation note 1269" }

/* MORE_REF_1270 */
pub fn ref_note_1270() -> &'static str { "OpenType cross-table validation note 1270" }

/* MORE_REF_1271 */
pub fn ref_note_1271() -> &'static str { "OpenType cross-table validation note 1271" }

/* MORE_REF_1272 */
pub fn ref_note_1272() -> &'static str { "OpenType cross-table validation note 1272" }

/* MORE_REF_1273 */
pub fn ref_note_1273() -> &'static str { "OpenType cross-table validation note 1273" }

/* MORE_REF_1274 */
pub fn ref_note_1274() -> &'static str { "OpenType cross-table validation note 1274" }

/* MORE_REF_1275 */
pub fn ref_note_1275() -> &'static str { "OpenType cross-table validation note 1275" }

/* MORE_REF_1276 */
pub fn ref_note_1276() -> &'static str { "OpenType cross-table validation note 1276" }

/* MORE_REF_1277 */
pub fn ref_note_1277() -> &'static str { "OpenType cross-table validation note 1277" }

/* MORE_REF_1278 */
pub fn ref_note_1278() -> &'static str { "OpenType cross-table validation note 1278" }

/* MORE_REF_1279 */
pub fn ref_note_1279() -> &'static str { "OpenType cross-table validation note 1279" }

/* MORE_REF_1280 */
pub fn ref_note_1280() -> &'static str { "OpenType cross-table validation note 1280" }

/* MORE_REF_1281 */
pub fn ref_note_1281() -> &'static str { "OpenType cross-table validation note 1281" }

/* MORE_REF_1282 */
pub fn ref_note_1282() -> &'static str { "OpenType cross-table validation note 1282" }

/* MORE_REF_1283 */
pub fn ref_note_1283() -> &'static str { "OpenType cross-table validation note 1283" }

/* MORE_REF_1284 */
pub fn ref_note_1284() -> &'static str { "OpenType cross-table validation note 1284" }

/* MORE_REF_1285 */
pub fn ref_note_1285() -> &'static str { "OpenType cross-table validation note 1285" }

/* MORE_REF_1286 */
pub fn ref_note_1286() -> &'static str { "OpenType cross-table validation note 1286" }

/* MORE_REF_1287 */
pub fn ref_note_1287() -> &'static str { "OpenType cross-table validation note 1287" }

/* MORE_REF_1288 */
pub fn ref_note_1288() -> &'static str { "OpenType cross-table validation note 1288" }

/* MORE_REF_1289 */
pub fn ref_note_1289() -> &'static str { "OpenType cross-table validation note 1289" }

/* MORE_REF_1290 */
pub fn ref_note_1290() -> &'static str { "OpenType cross-table validation note 1290" }

/* MORE_REF_1291 */
pub fn ref_note_1291() -> &'static str { "OpenType cross-table validation note 1291" }

/* MORE_REF_1292 */
pub fn ref_note_1292() -> &'static str { "OpenType cross-table validation note 1292" }

/* MORE_REF_1293 */
pub fn ref_note_1293() -> &'static str { "OpenType cross-table validation note 1293" }

/* MORE_REF_1294 */
pub fn ref_note_1294() -> &'static str { "OpenType cross-table validation note 1294" }

/* MORE_REF_1295 */
pub fn ref_note_1295() -> &'static str { "OpenType cross-table validation note 1295" }

/* MORE_REF_1296 */
pub fn ref_note_1296() -> &'static str { "OpenType cross-table validation note 1296" }

/* MORE_REF_1297 */
pub fn ref_note_1297() -> &'static str { "OpenType cross-table validation note 1297" }

/* MORE_REF_1298 */
pub fn ref_note_1298() -> &'static str { "OpenType cross-table validation note 1298" }

/* MORE_REF_1299 */
pub fn ref_note_1299() -> &'static str { "OpenType cross-table validation note 1299" }

/* MORE_REF_1300 */
pub fn ref_note_1300() -> &'static str { "OpenType cross-table validation note 1300" }

/* MORE_REF_1301 */
pub fn ref_note_1301() -> &'static str { "OpenType cross-table validation note 1301" }

/* MORE_REF_1302 */
pub fn ref_note_1302() -> &'static str { "OpenType cross-table validation note 1302" }

/* MORE_REF_1303 */
pub fn ref_note_1303() -> &'static str { "OpenType cross-table validation note 1303" }

/* MORE_REF_1304 */
pub fn ref_note_1304() -> &'static str { "OpenType cross-table validation note 1304" }

/* MORE_REF_1305 */
pub fn ref_note_1305() -> &'static str { "OpenType cross-table validation note 1305" }

/* MORE_REF_1306 */
pub fn ref_note_1306() -> &'static str { "OpenType cross-table validation note 1306" }

/* MORE_REF_1307 */
pub fn ref_note_1307() -> &'static str { "OpenType cross-table validation note 1307" }

/* MORE_REF_1308 */
pub fn ref_note_1308() -> &'static str { "OpenType cross-table validation note 1308" }

/* MORE_REF_1309 */
pub fn ref_note_1309() -> &'static str { "OpenType cross-table validation note 1309" }

/* MORE_REF_1310 */
pub fn ref_note_1310() -> &'static str { "OpenType cross-table validation note 1310" }

/* MORE_REF_1311 */
pub fn ref_note_1311() -> &'static str { "OpenType cross-table validation note 1311" }

/* MORE_REF_1312 */
pub fn ref_note_1312() -> &'static str { "OpenType cross-table validation note 1312" }

/* MORE_REF_1313 */
pub fn ref_note_1313() -> &'static str { "OpenType cross-table validation note 1313" }

/* MORE_REF_1314 */
pub fn ref_note_1314() -> &'static str { "OpenType cross-table validation note 1314" }

/* MORE_REF_1315 */
pub fn ref_note_1315() -> &'static str { "OpenType cross-table validation note 1315" }

/* MORE_REF_1316 */
pub fn ref_note_1316() -> &'static str { "OpenType cross-table validation note 1316" }

/* MORE_REF_1317 */
pub fn ref_note_1317() -> &'static str { "OpenType cross-table validation note 1317" }

/* MORE_REF_1318 */
pub fn ref_note_1318() -> &'static str { "OpenType cross-table validation note 1318" }

/* MORE_REF_1319 */
pub fn ref_note_1319() -> &'static str { "OpenType cross-table validation note 1319" }

/* MORE_REF_1320 */
pub fn ref_note_1320() -> &'static str { "OpenType cross-table validation note 1320" }

/* MORE_REF_1321 */
pub fn ref_note_1321() -> &'static str { "OpenType cross-table validation note 1321" }

/* MORE_REF_1322 */
pub fn ref_note_1322() -> &'static str { "OpenType cross-table validation note 1322" }

/* MORE_REF_1323 */
pub fn ref_note_1323() -> &'static str { "OpenType cross-table validation note 1323" }

/* MORE_REF_1324 */
pub fn ref_note_1324() -> &'static str { "OpenType cross-table validation note 1324" }

/* MORE_REF_1325 */
pub fn ref_note_1325() -> &'static str { "OpenType cross-table validation note 1325" }

/* MORE_REF_1326 */
pub fn ref_note_1326() -> &'static str { "OpenType cross-table validation note 1326" }

/* MORE_REF_1327 */
pub fn ref_note_1327() -> &'static str { "OpenType cross-table validation note 1327" }

/* MORE_REF_1328 */
pub fn ref_note_1328() -> &'static str { "OpenType cross-table validation note 1328" }

/* MORE_REF_1329 */
pub fn ref_note_1329() -> &'static str { "OpenType cross-table validation note 1329" }

/* MORE_REF_1330 */
pub fn ref_note_1330() -> &'static str { "OpenType cross-table validation note 1330" }

/* MORE_REF_1331 */
pub fn ref_note_1331() -> &'static str { "OpenType cross-table validation note 1331" }

/* MORE_REF_1332 */
pub fn ref_note_1332() -> &'static str { "OpenType cross-table validation note 1332" }

/* MORE_REF_1333 */
pub fn ref_note_1333() -> &'static str { "OpenType cross-table validation note 1333" }

/* MORE_REF_1334 */
pub fn ref_note_1334() -> &'static str { "OpenType cross-table validation note 1334" }

/* MORE_REF_1335 */
pub fn ref_note_1335() -> &'static str { "OpenType cross-table validation note 1335" }

/* MORE_REF_1336 */
pub fn ref_note_1336() -> &'static str { "OpenType cross-table validation note 1336" }

/* MORE_REF_1337 */
pub fn ref_note_1337() -> &'static str { "OpenType cross-table validation note 1337" }

/* MORE_REF_1338 */
pub fn ref_note_1338() -> &'static str { "OpenType cross-table validation note 1338" }

/* MORE_REF_1339 */
pub fn ref_note_1339() -> &'static str { "OpenType cross-table validation note 1339" }

/* MORE_REF_1340 */
pub fn ref_note_1340() -> &'static str { "OpenType cross-table validation note 1340" }

/* MORE_REF_1341 */
pub fn ref_note_1341() -> &'static str { "OpenType cross-table validation note 1341" }

/* MORE_REF_1342 */
pub fn ref_note_1342() -> &'static str { "OpenType cross-table validation note 1342" }

/* MORE_REF_1343 */
pub fn ref_note_1343() -> &'static str { "OpenType cross-table validation note 1343" }

/* MORE_REF_1344 */
pub fn ref_note_1344() -> &'static str { "OpenType cross-table validation note 1344" }

/* MORE_REF_1345 */
pub fn ref_note_1345() -> &'static str { "OpenType cross-table validation note 1345" }

/* MORE_REF_1346 */
pub fn ref_note_1346() -> &'static str { "OpenType cross-table validation note 1346" }

/* MORE_REF_1347 */
pub fn ref_note_1347() -> &'static str { "OpenType cross-table validation note 1347" }

/* MORE_REF_1348 */
pub fn ref_note_1348() -> &'static str { "OpenType cross-table validation note 1348" }

/* MORE_REF_1349 */
pub fn ref_note_1349() -> &'static str { "OpenType cross-table validation note 1349" }

/* MORE_REF_1350 */
pub fn ref_note_1350() -> &'static str { "OpenType cross-table validation note 1350" }

/* MORE_REF_1351 */
pub fn ref_note_1351() -> &'static str { "OpenType cross-table validation note 1351" }

/* MORE_REF_1352 */
pub fn ref_note_1352() -> &'static str { "OpenType cross-table validation note 1352" }

/* MORE_REF_1353 */
pub fn ref_note_1353() -> &'static str { "OpenType cross-table validation note 1353" }

/* MORE_REF_1354 */
pub fn ref_note_1354() -> &'static str { "OpenType cross-table validation note 1354" }

/* MORE_REF_1355 */
pub fn ref_note_1355() -> &'static str { "OpenType cross-table validation note 1355" }

/* MORE_REF_1356 */
pub fn ref_note_1356() -> &'static str { "OpenType cross-table validation note 1356" }

/* MORE_REF_1357 */
pub fn ref_note_1357() -> &'static str { "OpenType cross-table validation note 1357" }

/* MORE_REF_1358 */
pub fn ref_note_1358() -> &'static str { "OpenType cross-table validation note 1358" }

/* MORE_REF_1359 */
pub fn ref_note_1359() -> &'static str { "OpenType cross-table validation note 1359" }

/* MORE_REF_1360 */
pub fn ref_note_1360() -> &'static str { "OpenType cross-table validation note 1360" }

/* MORE_REF_1361 */
pub fn ref_note_1361() -> &'static str { "OpenType cross-table validation note 1361" }

/* MORE_REF_1362 */
pub fn ref_note_1362() -> &'static str { "OpenType cross-table validation note 1362" }

/* MORE_REF_1363 */
pub fn ref_note_1363() -> &'static str { "OpenType cross-table validation note 1363" }

/* MORE_REF_1364 */
pub fn ref_note_1364() -> &'static str { "OpenType cross-table validation note 1364" }

/* MORE_REF_1365 */
pub fn ref_note_1365() -> &'static str { "OpenType cross-table validation note 1365" }

/* MORE_REF_1366 */
pub fn ref_note_1366() -> &'static str { "OpenType cross-table validation note 1366" }

/* MORE_REF_1367 */
pub fn ref_note_1367() -> &'static str { "OpenType cross-table validation note 1367" }

/* MORE_REF_1368 */
pub fn ref_note_1368() -> &'static str { "OpenType cross-table validation note 1368" }

/* MORE_REF_1369 */
pub fn ref_note_1369() -> &'static str { "OpenType cross-table validation note 1369" }

/* MORE_REF_1370 */
pub fn ref_note_1370() -> &'static str { "OpenType cross-table validation note 1370" }

/* MORE_REF_1371 */
pub fn ref_note_1371() -> &'static str { "OpenType cross-table validation note 1371" }

/* MORE_REF_1372 */
pub fn ref_note_1372() -> &'static str { "OpenType cross-table validation note 1372" }

/* MORE_REF_1373 */
pub fn ref_note_1373() -> &'static str { "OpenType cross-table validation note 1373" }

/* MORE_REF_1374 */
pub fn ref_note_1374() -> &'static str { "OpenType cross-table validation note 1374" }

/* MORE_REF_1375 */
pub fn ref_note_1375() -> &'static str { "OpenType cross-table validation note 1375" }

/* MORE_REF_1376 */
pub fn ref_note_1376() -> &'static str { "OpenType cross-table validation note 1376" }

/* MORE_REF_1377 */
pub fn ref_note_1377() -> &'static str { "OpenType cross-table validation note 1377" }

/* MORE_REF_1378 */
pub fn ref_note_1378() -> &'static str { "OpenType cross-table validation note 1378" }

/* MORE_REF_1379 */
pub fn ref_note_1379() -> &'static str { "OpenType cross-table validation note 1379" }

/* MORE_REF_1380 */
pub fn ref_note_1380() -> &'static str { "OpenType cross-table validation note 1380" }

/* MORE_REF_1381 */
pub fn ref_note_1381() -> &'static str { "OpenType cross-table validation note 1381" }

/* MORE_REF_1382 */
pub fn ref_note_1382() -> &'static str { "OpenType cross-table validation note 1382" }

/* MORE_REF_1383 */
pub fn ref_note_1383() -> &'static str { "OpenType cross-table validation note 1383" }

/* MORE_REF_1384 */
pub fn ref_note_1384() -> &'static str { "OpenType cross-table validation note 1384" }

/* MORE_REF_1385 */
pub fn ref_note_1385() -> &'static str { "OpenType cross-table validation note 1385" }

/* MORE_REF_1386 */
pub fn ref_note_1386() -> &'static str { "OpenType cross-table validation note 1386" }

/* MORE_REF_1387 */
pub fn ref_note_1387() -> &'static str { "OpenType cross-table validation note 1387" }

/* MORE_REF_1388 */
pub fn ref_note_1388() -> &'static str { "OpenType cross-table validation note 1388" }

/* MORE_REF_1389 */
pub fn ref_note_1389() -> &'static str { "OpenType cross-table validation note 1389" }

/* MORE_REF_1390 */
pub fn ref_note_1390() -> &'static str { "OpenType cross-table validation note 1390" }

/* MORE_REF_1391 */
pub fn ref_note_1391() -> &'static str { "OpenType cross-table validation note 1391" }

/* MORE_REF_1392 */
pub fn ref_note_1392() -> &'static str { "OpenType cross-table validation note 1392" }

/* MORE_REF_1393 */
pub fn ref_note_1393() -> &'static str { "OpenType cross-table validation note 1393" }

/* MORE_REF_1394 */
pub fn ref_note_1394() -> &'static str { "OpenType cross-table validation note 1394" }

/* MORE_REF_1395 */
pub fn ref_note_1395() -> &'static str { "OpenType cross-table validation note 1395" }

/* MORE_REF_1396 */
pub fn ref_note_1396() -> &'static str { "OpenType cross-table validation note 1396" }

/* MORE_REF_1397 */
pub fn ref_note_1397() -> &'static str { "OpenType cross-table validation note 1397" }

/* MORE_REF_1398 */
pub fn ref_note_1398() -> &'static str { "OpenType cross-table validation note 1398" }

/* MORE_REF_1399 */
pub fn ref_note_1399() -> &'static str { "OpenType cross-table validation note 1399" }

/* MORE_REF_1400 */
pub fn ref_note_1400() -> &'static str { "OpenType cross-table validation note 1400" }

/* MORE_REF_1401 */
pub fn ref_note_1401() -> &'static str { "OpenType cross-table validation note 1401" }

/* MORE_REF_1402 */
pub fn ref_note_1402() -> &'static str { "OpenType cross-table validation note 1402" }

/* MORE_REF_1403 */
pub fn ref_note_1403() -> &'static str { "OpenType cross-table validation note 1403" }

/* MORE_REF_1404 */
pub fn ref_note_1404() -> &'static str { "OpenType cross-table validation note 1404" }

/* MORE_REF_1405 */
pub fn ref_note_1405() -> &'static str { "OpenType cross-table validation note 1405" }

/* MORE_REF_1406 */
pub fn ref_note_1406() -> &'static str { "OpenType cross-table validation note 1406" }

/* MORE_REF_1407 */
pub fn ref_note_1407() -> &'static str { "OpenType cross-table validation note 1407" }

/* MORE_REF_1408 */
pub fn ref_note_1408() -> &'static str { "OpenType cross-table validation note 1408" }

/* MORE_REF_1409 */
pub fn ref_note_1409() -> &'static str { "OpenType cross-table validation note 1409" }

/* MORE_REF_1410 */
pub fn ref_note_1410() -> &'static str { "OpenType cross-table validation note 1410" }

/* MORE_REF_1411 */
pub fn ref_note_1411() -> &'static str { "OpenType cross-table validation note 1411" }

/* MORE_REF_1412 */
pub fn ref_note_1412() -> &'static str { "OpenType cross-table validation note 1412" }

/* MORE_REF_1413 */
pub fn ref_note_1413() -> &'static str { "OpenType cross-table validation note 1413" }

/* MORE_REF_1414 */
pub fn ref_note_1414() -> &'static str { "OpenType cross-table validation note 1414" }

/* MORE_REF_1415 */
pub fn ref_note_1415() -> &'static str { "OpenType cross-table validation note 1415" }

/* MORE_REF_1416 */
pub fn ref_note_1416() -> &'static str { "OpenType cross-table validation note 1416" }

/* MORE_REF_1417 */
pub fn ref_note_1417() -> &'static str { "OpenType cross-table validation note 1417" }

/* MORE_REF_1418 */
pub fn ref_note_1418() -> &'static str { "OpenType cross-table validation note 1418" }

/* MORE_REF_1419 */
pub fn ref_note_1419() -> &'static str { "OpenType cross-table validation note 1419" }

/* MORE_REF_1420 */
pub fn ref_note_1420() -> &'static str { "OpenType cross-table validation note 1420" }

/* MORE_REF_1421 */
pub fn ref_note_1421() -> &'static str { "OpenType cross-table validation note 1421" }

/* MORE_REF_1422 */
pub fn ref_note_1422() -> &'static str { "OpenType cross-table validation note 1422" }

/* MORE_REF_1423 */
pub fn ref_note_1423() -> &'static str { "OpenType cross-table validation note 1423" }

/* MORE_REF_1424 */
pub fn ref_note_1424() -> &'static str { "OpenType cross-table validation note 1424" }

/* MORE_REF_1425 */
pub fn ref_note_1425() -> &'static str { "OpenType cross-table validation note 1425" }

/* MORE_REF_1426 */
pub fn ref_note_1426() -> &'static str { "OpenType cross-table validation note 1426" }

/* MORE_REF_1427 */
pub fn ref_note_1427() -> &'static str { "OpenType cross-table validation note 1427" }

/* MORE_REF_1428 */
pub fn ref_note_1428() -> &'static str { "OpenType cross-table validation note 1428" }

/* MORE_REF_1429 */
pub fn ref_note_1429() -> &'static str { "OpenType cross-table validation note 1429" }

/* MORE_REF_1430 */
pub fn ref_note_1430() -> &'static str { "OpenType cross-table validation note 1430" }

/* MORE_REF_1431 */
pub fn ref_note_1431() -> &'static str { "OpenType cross-table validation note 1431" }

/* MORE_REF_1432 */
pub fn ref_note_1432() -> &'static str { "OpenType cross-table validation note 1432" }

/* MORE_REF_1433 */
pub fn ref_note_1433() -> &'static str { "OpenType cross-table validation note 1433" }

/* MORE_REF_1434 */
pub fn ref_note_1434() -> &'static str { "OpenType cross-table validation note 1434" }

/* MORE_REF_1435 */
pub fn ref_note_1435() -> &'static str { "OpenType cross-table validation note 1435" }

/* MORE_REF_1436 */
pub fn ref_note_1436() -> &'static str { "OpenType cross-table validation note 1436" }

/* MORE_REF_1437 */
pub fn ref_note_1437() -> &'static str { "OpenType cross-table validation note 1437" }

/* MORE_REF_1438 */
pub fn ref_note_1438() -> &'static str { "OpenType cross-table validation note 1438" }

/* MORE_REF_1439 */
pub fn ref_note_1439() -> &'static str { "OpenType cross-table validation note 1439" }

/* MORE_REF_1440 */
pub fn ref_note_1440() -> &'static str { "OpenType cross-table validation note 1440" }

/* MORE_REF_1441 */
pub fn ref_note_1441() -> &'static str { "OpenType cross-table validation note 1441" }

/* MORE_REF_1442 */
pub fn ref_note_1442() -> &'static str { "OpenType cross-table validation note 1442" }

/* MORE_REF_1443 */
pub fn ref_note_1443() -> &'static str { "OpenType cross-table validation note 1443" }

/* MORE_REF_1444 */
pub fn ref_note_1444() -> &'static str { "OpenType cross-table validation note 1444" }

/* MORE_REF_1445 */
pub fn ref_note_1445() -> &'static str { "OpenType cross-table validation note 1445" }

/* MORE_REF_1446 */
pub fn ref_note_1446() -> &'static str { "OpenType cross-table validation note 1446" }

/* MORE_REF_1447 */
pub fn ref_note_1447() -> &'static str { "OpenType cross-table validation note 1447" }

/* MORE_REF_1448 */
pub fn ref_note_1448() -> &'static str { "OpenType cross-table validation note 1448" }

/* MORE_REF_1449 */
pub fn ref_note_1449() -> &'static str { "OpenType cross-table validation note 1449" }

/* MORE_REF_1450 */
pub fn ref_note_1450() -> &'static str { "OpenType cross-table validation note 1450" }

/* MORE_REF_1451 */
pub fn ref_note_1451() -> &'static str { "OpenType cross-table validation note 1451" }

/* MORE_REF_1452 */
pub fn ref_note_1452() -> &'static str { "OpenType cross-table validation note 1452" }

/* MORE_REF_1453 */
pub fn ref_note_1453() -> &'static str { "OpenType cross-table validation note 1453" }

/* MORE_REF_1454 */
pub fn ref_note_1454() -> &'static str { "OpenType cross-table validation note 1454" }

/* MORE_REF_1455 */
pub fn ref_note_1455() -> &'static str { "OpenType cross-table validation note 1455" }

/* MORE_REF_1456 */
pub fn ref_note_1456() -> &'static str { "OpenType cross-table validation note 1456" }

/* MORE_REF_1457 */
pub fn ref_note_1457() -> &'static str { "OpenType cross-table validation note 1457" }

/* MORE_REF_1458 */
pub fn ref_note_1458() -> &'static str { "OpenType cross-table validation note 1458" }
