//! cmap — OpenType table parser.


use crate::cmap::subtable::CmapSubtable;
use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;

#[derive(Clone, Debug)]
pub struct CmapTable {
    pub version: u16,
    pub num_tables: u16,
    pub subtables: Vec<CmapSubtable>,
}

impl CmapTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let version = r.read_u16()?;
        let num_tables = r.read_u16()?;
        if num_tables as usize > limits::MAX_CMAP_SUBTABLES {
            return Err(GlyphError::OutOfRange {
                field: "cmap.numSubtables",
                value: num_tables as i64,
            });
        }
        let mut subtables = Vec::with_capacity(num_tables as usize);
        for _ in 0..num_tables {
            let platform_id = r.read_u16()?;
            let encoding_id = r.read_u16()?;
            let offset = r.read_u32()? as usize;
            limits::check_len(offset, data.len())?;
            let sub = CmapSubtable::parse_at(data, offset, platform_id, encoding_id)?;
            subtables.push(sub);
        }
        Ok(Self { version, num_tables, subtables })
    }

    pub fn best_unicode_subtable(&self) -> Option<&CmapSubtable> {
        self.subtables.iter().find(|s| s.is_unicode())
            .or_else(|| self.subtables.iter().find(|s| s.is_symbol()))
    }

    pub fn map_codepoint(&self, ch: u32) -> GlyphResult<Option<u16>> {
        let sub = self.best_unicode_subtable().ok_or(GlyphError::CmapNoUnicode)?;
        sub.map_codepoint(ch)
    }

    pub fn collect_codepoints_for_glyph(&self, gid: u16) -> Vec<u32> {
        let mut out = Vec::new();
        if let Some(sub) = self.best_unicode_subtable() {
            sub.collect_glyph_codepoints(gid, &mut out);
        }
        out
    }

    pub fn validate_subtable_offsets(&self, data_len: usize) -> GlyphResult<()> {
        for sub in &self.subtables {
            sub.validate_bounds(data_len)?;
        }
        Ok(())
    }

    pub fn encoding_ids(&self) -> impl Iterator<Item = (u16, u16)> + '_ {
        self.subtables.iter().map(|s| (s.platform_id, s.encoding_id))
    }

    pub fn has_format_4(&self) -> bool {
        self.subtables.iter().any(|s| s.format == 4)
    }

    pub fn has_format_12(&self) -> bool {
        self.subtables.iter().any(|s| s.format == 12)
    }

    pub fn subtable_count(&self) -> usize {
        self.subtables.len()
    }

    pub fn describe(&self) -> String {
        let mut s = format!("cmap v{} {} subtables", self.version, self.num_tables);
        for sub in &self.subtables {
            s.push_str(&format!(
                " [platform {} enc {} format {}]",
                sub.platform_id, sub.encoding_id, sub.format
            ));
        }
        s
    }
}

/// OpenType 'cmap' table field reference.
pub mod field_docs {
    /// Number of encoding subtables
    pub const NUM_TABLES: &str = "Number of encoding subtables";
}
impl CmapTable {
    pub fn subtable_at(&self, index: usize) -> Option<&crate::cmap::subtable::CmapSubtable> {
        self.subtables.get(index)
    }

    pub fn find_subtable(&self, platform_id: u16, encoding_id: u16) -> Option<&crate::cmap::subtable::CmapSubtable> {
        self.subtables.iter().find(|s| s.platform_id == platform_id && s.encoding_id == encoding_id)
    }

    pub fn map_all_codepoints(&self, codepoints: &[u32]) -> crate::GlyphResult<Vec<Option<u16>>> {
        let mut out = Vec::with_capacity(codepoints.len());
        for &cp in codepoints {
            out.push(self.map_codepoint(cp)?);
        }
        Ok(out)
    }

    pub fn summarize_formats(&self) -> Vec<u16> {
        self.subtables.iter().map(|s| s.format).collect()
    }
}

/// Parse cmap encoding record without loading subtable body.
pub fn parse_encoding_record(data: &[u8], index: usize) -> crate::GlyphResult<(u16, u16, u32)> {
    let base = 4 + index * 8;
    crate::limits::check_len(base + 8, data.len())?;
    let mut r = crate::io::FontReader::from_slice(data, base)?;
    Ok((r.read_u16()?, r.read_u16()?, r.read_u32()?))
}

pub fn validate_format4_segments(data: &[u8]) -> crate::GlyphResult<()> {
    let segs = crate::cmap::decode::parse_format4_segments(data)?;
    for w in segs.windows(2) {
        if w[0].end >= w[1].start {
            return Err(crate::GlyphError::Consistency { detail: "cmap4 segments not sorted" });
        }
    }
    Ok(())
}

/* depth:cmap */

pub fn cmap_format_0_probe(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    let fmt = u16::from_be_bytes([data[0], data[1]]);
    Ok(fmt == 0)
}


pub fn cmap_format_2_probe(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    let fmt = u16::from_be_bytes([data[0], data[1]]);
    Ok(fmt == 2)
}


pub fn cmap_format_4_probe(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    let fmt = u16::from_be_bytes([data[0], data[1]]);
    Ok(fmt == 4)
}


pub fn cmap_format_6_probe(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    let fmt = u16::from_be_bytes([data[0], data[1]]);
    Ok(fmt == 6)
}


pub fn cmap_format_10_probe(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    let fmt = u16::from_be_bytes([data[0], data[1]]);
    Ok(fmt == 10)
}


pub fn cmap_format_12_probe(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    let fmt = u16::from_be_bytes([data[0], data[1]]);
    Ok(fmt == 12)
}


pub fn cmap_format_14_probe(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    let fmt = u16::from_be_bytes([data[0], data[1]]);
    Ok(fmt == 14)
}


impl CmapTable {
    pub fn probe_all_formats(&self) -> Vec<u16> {
        self.subtables.iter().map(|s| s.format).collect()
    }
}

/* field_matrix:cmap */
pub mod field_readers_cmap {
pub fn read_version(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[0], data[1]]))
}
pub fn read_num_tables(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[2], data[3]]))
}
}

/* field_checks:cmap */
impl CmapTable {
pub fn check_num_tables_zero(&self) -> crate::GlyphResult<()> {
    if self.num_tables == 0 {
        return Err(crate::GlyphError::Consistency { detail: "no cmap subtables" });
    }
    Ok(())
}
pub fn check_version(&self) -> crate::GlyphResult<()> {
    if self.version > 0 {
        return Err(crate::GlyphError::Consistency { detail: "cmap version unexpected" });
    }
    Ok(())
}
}

/* walker:cmap */

pub struct CmapFormat4Walker<'a> {
    data: &'a [u8],
    segments: Vec<crate::cmap::decode::Format4Segment>,
}

impl<'a> CmapFormat4Walker<'a> {
    pub fn open(data: &'a [u8]) -> crate::GlyphResult<Self> {
        Ok(Self { data, segments: crate::cmap::decode::parse_format4_segments(data)? })
    }

    pub fn segment_count(&self) -> usize { self.segments.len() }

    pub fn map(&self, cp: u32) -> crate::GlyphResult<Option<u16>> {
        crate::cmap::decode::map_codepoint_format4(self.data, cp)
    }

    pub fn validate_non_overlapping(&self) -> crate::GlyphResult<()> {
        for w in self.segments.windows(2) {
            if w[0].end >= w[1].start && w[1].end != 0xFFFF {
                return Err(crate::GlyphError::Consistency { detail: "overlapping cmap segments" });
            }
        }
        Ok(())
    }

    pub fn for_each_mapped<F>(&self, mut f: F) -> crate::GlyphResult<()>
    where F: FnMut(u32, u16) -> crate::GlyphResult<()>,
    {
        for seg in &self.segments {
            if seg.end == 0xFFFF { break; }
            for cp in seg.start..=seg.end {
                if let Some(gid) = self.map(cp)? {
                    f(cp, gid)?;
                }
            }
        }
        Ok(())
    }
}
