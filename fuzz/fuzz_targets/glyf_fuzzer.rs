//! Fuzz harness for glyf_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::glyf::GlyfTable;

fuzz_target!(|data: &[u8]| {
    if data.len() < 24 {
        return;
    }
    let t = GlyfTable::new(data);
    let split = if data.len() > 32 {
        16usize
    } else {
        data.len() / 2
    };
    let _ = t.parse_glyph(0, split as u32, 0);
    let _ = t.parse_glyph(split as u32, data.len() as u32, 1);
    let _ = t.finalize_outline_cache();
});
