//! Fuzz harness for cmap_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::cmap::CmapTable;

fuzz_target!(|data: &[u8]| {
    if let Ok(table) = CmapTable::parse(data) {
        let _ = table.map_codepoint(0x41);
        if data.len() >= 2 {
            let cp = u32::from(u16::from_be_bytes([data[0], data[1]]));
            let _ = table.map_codepoint(cp);
        }
    }
});
