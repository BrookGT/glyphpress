//! kern — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KernPair {
    pub left: u16,
    pub right: u16,
    pub value: i16,
}

#[derive(Clone, Debug)]
pub struct KernSubtable {
    pub version: u16,
    pub coverage: u16,
    pub pairs: Vec<KernPair>,
}

#[derive(Clone, Debug)]
pub struct KernTable {
    pub subtables: Vec<KernSubtable>,
}

impl KernTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let version = r.read_u16()?;
        let n_tables = r.read_u16()?;
        if n_tables as usize > limits::MAX_KERN_SUBTABLES {
            return Err(GlyphError::OutOfRange { field: "kern.nTables", value: n_tables as i64 });
        }
        let mut subtables = Vec::with_capacity(n_tables as usize);
        for _ in 0..n_tables {
            subtables.push(parse_subtable(&mut r)?);
        }
        let _ = version;
        Ok(Self { subtables })
    }

    pub fn lookup(&self, left: u16, right: u16) -> Option<i16> {
        for sub in &self.subtables {
            if sub.coverage & 1 == 0 {
                if let Some(v) = sub.pairs.iter().find(|p| p.left == left && p.right == right) {
                    return Some(v.value);
                }
            }
        }
        None
    }
}

fn parse_subtable(r: &mut FontReader<'_>) -> GlyphResult<KernSubtable> {
    let _sub_len = r.read_u16()?;
    let coverage = r.read_u16()?;
    let n_pairs = r.read_u16()?;
    let _search_range = r.read_u16()?;
    let _entry_selector = r.read_u16()?;
    let _range_shift = r.read_u16()?;
    let mut pairs = Vec::with_capacity(n_pairs as usize);
    for _ in 0..n_pairs {
        pairs.push(KernPair {
            left: r.read_u16()?,
            right: r.read_u16()?,
            value: r.read_i16()?,
        });
    }
    Ok(KernSubtable { version: 0, coverage, pairs })
}

/// OpenType 'kern' table field reference.
pub mod field_docs {
    /// Subtable coverage bits
    pub const COVERAGE: &str = "Subtable coverage bits";
}
impl KernSubtable {
    pub fn is_horizontal(&self) -> bool { self.coverage & 0x01 == 0 }
    pub fn has_minimum_values(&self) -> bool { self.coverage & 0x02 != 0 }
    pub fn pair_count(&self) -> usize { self.pairs.len() }
}

impl KernTable {
    pub fn total_pairs(&self) -> usize {
        self.subtables.iter().map(|s| s.pairs.len()).sum()
    }

    pub fn lookup_sorted(&self, left: u16, right: u16) -> Option<i16> {
        for sub in &self.subtables {
            if let Ok(idx) = sub.pairs.binary_search_by_key(&(left, right), |p| (p.left, p.right)) {
                return Some(sub.pairs[idx].value);
            }
        }
        None
    }
}

/* depth:kern */

impl KernPair {
    pub fn key(&self) -> u32 {
        ((self.left as u32) << 16) | self.right as u32
    }
}

/* field_matrix:kern */
pub mod field_readers_kern {
pub fn read_n_tables(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[2], data[3]]))
}
pub fn read_subtable_coverage(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[2], data[3]]))
}
}

/* field_checks:kern */
impl KernTable {
pub fn check_empty(&self) -> crate::GlyphResult<()> {
    if self.subtables.is_empty() {
        return Err(crate::GlyphError::Consistency { detail: "no kern subtables" });
    }
    Ok(())
}
}

/* walker:kern */

impl KernTable {
    pub fn horizontal_pairs(&self) -> usize {
        self.subtables.iter().filter(|s| s.is_horizontal()).map(|s| s.pairs.len()).sum()
    }
}
