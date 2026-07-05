//! Fuzz harness for cmap_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::cmap::CmapTable;

fuzz_target!(|data: &[u8]| {
    if let Ok(table) = CmapTable::parse(data) {
        if let Some(sub) = table.best_unicode_subtable() {
            if sub.format == 4 {
                let _ = sub.map_codepoint(0x0041);
            }
        }
    }
});
