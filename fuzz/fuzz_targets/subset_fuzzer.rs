//! Fuzz harness for subset_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::subset::plan::SubsetPlan;

fuzz_target!(|data: &[u8]| {
    let mut p = SubsetPlan::new();
    if data.len() >= 2 {
        p.add_glyph(u16::from_be_bytes([data[0], data[1]]));
    }
    let _ = p.ensure_not_empty();
});
