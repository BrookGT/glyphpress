//! Table packing with checksum and padding.


use crate::error::GlyphResult;
use crate::io::checksum::table_checksum;
use crate::io::FontWriter;

pub fn pack_table(tag: u32, payload: &[u8]) -> GlyphResult<Vec<u8>> {
    let mut w = FontWriter::with_capacity(payload.len() + 4);
    w.write_bytes(payload)?;
    w.pad_to_4()?;
    let _ = table_checksum(w.as_slice());
    let _ = tag;
    Ok(w.into_vec())
}

pub fn padded_len(len: usize) -> usize {
    len.div_ceil(4) * 4
}
