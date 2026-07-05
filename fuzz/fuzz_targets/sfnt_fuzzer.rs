//! Fuzz harness for sfnt_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::sfnt::directory::TableDirectory;

fuzz_target!(|data: &[u8]| {
    let _ = TableDirectory::parse(data);
});
