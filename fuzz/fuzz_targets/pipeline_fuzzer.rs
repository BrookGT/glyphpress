//! Fuzz harness for pipeline_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::pipeline::load::LoadedFont;

fuzz_target!(|data: &[u8]| {
    let _ = LoadedFont::open(data);
});
