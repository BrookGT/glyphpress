//! Fuzz harness for glyf_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::glyf::GlyfTable;

fuzz_target!(|data: &[u8]| {
    let t = GlyfTable::new(data);
    let end = data.len().min(256) as u32;
    let _ = t.slice_for_range(0, end);
    let _ = t.parse_glyph(0, end, 0);
});
