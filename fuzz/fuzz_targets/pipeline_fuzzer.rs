//! Fuzz harness for pipeline_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::pipeline::load::LoadedFont;

fuzz_target!(|data: &[u8]| {
    if let Ok(font) = LoadedFont::open(data) {
        let _ = font.prepare_layout_emit();
        let ng = font.maxp.num_glyphs;
        let _ = font.hmtx.lsb(ng);
        let _ = font.loca.glyph_range(ng);
    }
});
