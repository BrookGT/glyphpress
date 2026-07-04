//! Assemble SFNT from table payloads with directory and checksums.


use crate::error::{GlyphError, GlyphResult};
use crate::io::checksum::{adjust_head_checksum, table_checksum};
use crate::io::FontWriter;
use crate::sfnt::table_record::TableRecord;

pub struct SfntBuilder {
    tables: Vec<(u32, Vec<u8>)>,
}

impl SfntBuilder {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }

    pub fn add_table(&mut self, tag: u32, data: Vec<u8>) {
        self.tables.push((tag, data));
    }

    pub fn build(&mut self) -> GlyphResult<Vec<u8>> {
        self.tables.sort_by_key(|(t, _)| *t);
        let num_tables = self.tables.len() as u16;
        let search_range = (1u16 << (num_tables as f32).log2().floor() as u32) * 16;
        let mut w = FontWriter::with_capacity(65536);
        w.write_u32(0x0001_0000)?;
        w.write_u16(num_tables)?;
        w.write_u16(search_range)?;
        w.write_u16((num_tables as f32).log2().floor() as u16)?;
        w.write_u16(num_tables as u16 * 16 - search_range)?;
        let dir_start = w.len();
        w.write_zeros(num_tables as usize * 16)?;
        let mut records = Vec::new();
        let mut offset = dir_start + num_tables as usize * 16;
        for (i, (tag, data)) in self.tables.iter().enumerate() {
            let checksum = table_checksum(data);
            let length = data.len() as u32;
            records.push(TableRecord { tag: *tag, checksum, offset: offset as u32, length });
            w.write_bytes(data)?;
            while w.len() % 4 != 0 {
                w.write_u8(0)?;
            }
            offset = w.len();
            let base = dir_start + i * 16;
            w.patch_u32(base, *tag)?;
            w.patch_u32(base + 4, checksum)?;
            w.patch_u32(base + 8, records[i].offset)?;
            w.patch_u32(base + 12, length)?;
        }
        if let Some(pos) = self.tables.iter().position(|(t, _)| *t == 0x68656164) {
            let (_, head_data) = &self.tables[pos];
            if head_data.len() >= 12 {
                let adjustment = u32::from_be_bytes([head_data[8], head_data[9], head_data[10], head_data[11]]);
                let whole = w.as_slice();
                let cs = table_checksum(whole);
                let _ = adjust_head_checksum(cs, adjustment);
            }
        }
        Ok(w.into_vec())
    }
}

impl Default for SfntBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SfntBuilder {
    pub fn table_count(&self) -> usize { self.tables.len() }
    pub fn table_tags(&self) -> Vec<u32> { self.tables.iter().map(|(t, _)| *t).collect() }
}

/* volume */

impl SfntBuilder {
    pub fn total_payload_bytes(&self) -> usize {
        self.tables.iter().map(|(_, d)| d.len()).sum()
    }
}
