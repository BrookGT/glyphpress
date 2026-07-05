//! Fuzz harness for name_fuzzer.

#![no_main]

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::name::NameTable;

fuzz_target!(|data: &[u8]| {
    if let Ok(table) = NameTable::parse(data) {
        for rec in &table.records {
            let _ = table.decode_record(rec);
        }
    }
});
