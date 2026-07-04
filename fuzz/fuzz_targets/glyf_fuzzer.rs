//! Fuzz harness for glyf_fuzzer.

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::glyf::GlyfTable;

fuzz_target!(|data: &[u8]| {
    let t = GlyfTable::new(data);
    let _ = t.slice_for_range(0, data.len().min(64) as u32);
});
