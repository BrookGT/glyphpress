//! Fuzz harness for subset_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::pipeline::load::LoadedFont;
use glyphpress_core::pipeline::subset_run::{run_subset, SubsetOptions};

fuzz_target!(|data: &[u8]| {
    if let Ok(font) = LoadedFont::open(data) {
        let mut opts = SubsetOptions::default();
        opts.codepoints.push(0x41);
        if data.len() >= 2 {
            opts.codepoints.push(u32::from(u16::from_be_bytes([data[0], data[1]])));
        }
        let _ = run_subset(&font, &opts);
    }
});
