//! Fuzz harness for sfnt_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::sfnt::directory::TableDirectory;

fuzz_target!(|data: &[u8]| {
    if let Ok(dir) = TableDirectory::parse(data) {
        for i in 0..dir.record_count() {
            let _ = dir.probe_record_payload(data, i);
        }
    }
});
