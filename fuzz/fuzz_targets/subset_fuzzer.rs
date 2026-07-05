//! Fuzz harness for subset_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::pipeline::load::LoadedFont;

fuzz_target!(|data: &[u8]| {
    if let Ok(font) = LoadedFont::open(data) {
        let _ = font.run_subset_emit_plan();
    }
});
