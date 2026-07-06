//! Fuzz harness for post format-2 name arena lifecycle.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::post::PostTable;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }
    let num_glyphs = u16::from_be_bytes([data[0], data[1]]);
    let _ = PostTable::parse(&data[2..], num_glyphs);
});
