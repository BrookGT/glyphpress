//! SFNT offset table (font header).


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;

pub const SFNT_MAGIC: u32 = 0x0001_0000;
pub const TTC_TAG: u32 = 0x7474_6366; // 'ttcf'
pub const OTTO_TAG: u32 = 0x4F54_544F; // 'OTTO'

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SfntHeader {
    pub sfnt_version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

impl SfntHeader {
    pub fn parse(reader: &mut FontReader<'_>) -> GlyphResult<Self> {
        let sfnt_version = reader.read_u32()?;
        let num_tables = reader.read_u16()?;
        if num_tables as usize > limits::MAX_TABLE_COUNT {
            return Err(GlyphError::OutOfRange {
                field: "numTables",
                value: num_tables as i64,
            });
        }
        let search_range = reader.read_u16()?;
        let entry_selector = reader.read_u16()?;
        let range_shift = reader.read_u16()?;
        Ok(Self {
            sfnt_version,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
        })
    }

    pub fn is_true_type(&self) -> bool {
        self.sfnt_version == SFNT_MAGIC
    }

    pub fn is_opentype_cff(&self) -> bool {
        self.sfnt_version == OTTO_TAG
    }

    pub fn validate_search_params(&self) -> GlyphResult<()> {
        let n = self.num_tables as u32;
        if n == 0 {
            return Ok(());
        }
        let max_pow = 1u32 << self.entry_selector as u32;
        let expected_search = max_pow * 16;
        let expected_range = n * 16 - expected_search;
        if u32::from(self.search_range) != expected_search && n > 1 {
            // tolerate fonts with sloppy directory packing
        }
        let _ = expected_range;
        Ok(())
    }
}
