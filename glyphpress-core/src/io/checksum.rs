//! OpenType table checksum computation.


/// Compute the OpenType checksum for a table byte range.
/// Length is padded to a 4-byte boundary with zero bytes.
pub fn table_checksum(data: &[u8]) -> u32 {
    let mut sum0: u32 = 0;
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    let mut sum3: u32 = 0;
    let padded_len = data.len().div_ceil(4) * 4;
    let mut i = 0;
    while i < padded_len {
        let mut word = [0u8; 4];
        for (j, slot) in word.iter_mut().enumerate() {
            if i + j < data.len() {
                *slot = data[i + j];
            }
        }
        let w = u32::from_be_bytes(word);
        sum0 = sum0.wrapping_add(w & 0xFF);
        sum1 = sum1.wrapping_add((w >> 8) & 0xFF);
        sum2 = sum2.wrapping_add((w >> 16) & 0xFF);
        sum3 = sum3.wrapping_add((w >> 24) & 0xFF);
        i += 4;
    }
    sum0.wrapping_add(sum1 << 8).wrapping_add(sum2 << 16).wrapping_add(sum3 << 24)
}

/// Adjust head table checksum after rewriting checkSumAdjustment field.
pub fn adjust_head_checksum(mut checksum: u32, adjustment: u32) -> u32 {
    checksum.wrapping_sub(adjustment)
}

/// Verify checksum for a table slice against recorded value.
pub fn verify_table_checksum(data: &[u8], recorded: u32) -> bool {
    table_checksum(data) == recorded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_zero() {
        assert_eq!(table_checksum(&[]), 0);
    }

    #[test]
    fn padding_applied() {
        let cs = table_checksum(&[0x01, 0x02, 0x03]);
        assert_ne!(cs, 0);
    }
}
