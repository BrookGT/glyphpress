//! Fuzz harness for pipeline_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::pipeline::load::LoadedFont;

fuzz_target!(|data: &[u8]| {
    if let Ok(font) = LoadedFont::open(data) {
        let ng = font.maxp.num_glyphs;
        let _ = font.hmtx.lsb(ng);
        let _ = font.loca.glyph_range(ng);
        if ng > 0 {
            let _ = font.glyph_bytes(0);
        }
    }
});
