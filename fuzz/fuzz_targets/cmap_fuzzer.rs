//! Fuzz harness for cmap_fuzzer.

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::cmap::CmapTable;

fuzz_target!(|data: &[u8]| {
    let _ = CmapTable::parse(data);
});
