//! Fuzz harness for name_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::pipeline::name_session_touch;
use glyphpress_core::tables::name::NameTable;

fuzz_target!(|data: &[u8]| {
    if let Ok(table) = NameTable::parse(data) {
        for rec in &table.records {
            let _ = name_session_touch(&table.storage, rec.length);
        }
    }
});
