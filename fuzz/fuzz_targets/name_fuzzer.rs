//! Fuzz harness for name_fuzzer.

use libfuzzer_sys::fuzz_target;
use glyphpress_core::tables::name::NameTable;

fuzz_target!(|data: &[u8]| {
    let _ = NameTable::parse(data);
});
