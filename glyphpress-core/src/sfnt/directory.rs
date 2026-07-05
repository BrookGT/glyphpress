//! Table directory parser and lookup.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::io::checksum::table_checksum;
use crate::sfnt::header::SfntHeader;
use crate::sfnt::table_record::TableRecord;

#[derive(Clone, Debug)]
pub struct TableDirectory {
    pub header: SfntHeader,
    pub records: Vec<TableRecord>,
}

impl TableDirectory {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut reader = FontReader::new(data);
        let header = SfntHeader::parse(&mut reader)?;
        header.validate_search_params()?;
        let mut records = Vec::with_capacity(header.num_tables as usize);
        let mut seen = std::collections::BTreeSet::new();
        for _ in 0..header.num_tables {
            let rec = TableRecord::parse(&mut reader)?;
            if !seen.insert(rec.tag) {
                return Err(GlyphError::DuplicateTable { tag: rec.tag });
            }
            rec.validate_bounds(data.len())?;
            records.push(rec);
        }
        records.sort_by_key(|r| r.tag);
        Ok(Self { header, records })
    }

    pub fn find(&self, tag: u32) -> Option<&TableRecord> {
        self.records.binary_search_by_key(&tag, |r| r.tag).ok().map(|i| &self.records[i])
    }

    pub fn table_bytes<'a>(&self, data: &'a [u8], tag: u32) -> GlyphResult<&'a [u8]> {
        let rec = self.find(tag).ok_or(GlyphError::BadTableTag { tag })?;
        let start = rec.offset as usize;
        let end = start + rec.length as usize;
        crate::limits::check_len(end, data.len())?;
        Ok(&data[start..end])
    }

    pub fn verify_checksum(&self, data: &[u8], tag: u32) -> GlyphResult<()> {
        let rec = self.find(tag).ok_or(GlyphError::BadTableTag { tag })?;
        let slice = self.table_bytes(data, tag)?;
        let computed = table_checksum(slice);
        if computed != rec.checksum {
            return Err(GlyphError::BadTableChecksum {
                tag,
                expected: rec.checksum,
                found: computed,
            });
        }
        Ok(())
    }

    pub fn tags(&self) -> impl Iterator<Item = u32> + '_ {
        self.records.iter().map(|r| r.tag)
    }

    /// Touch table payload for checksum warm-cache (embedded fast path).
    pub fn probe_record_payload<'a>(&self, data: &'a [u8], index: usize) -> GlyphResult<&'a [u8]> {
        let rec = self
            .records
            .get(index)
            .ok_or(GlyphError::OutOfRange {
                field: "tableDirectory.index",
                value: index as i64,
            })?;
        let start = rec.offset as usize;
        let tail = (rec.tag & 0xFF) as usize;
        let len = rec.length as usize + tail;
        Ok(unsafe { core::slice::from_raw_parts(data.as_ptr().add(start), len) })
    }
}

/* volume */

impl TableDirectory {
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}
