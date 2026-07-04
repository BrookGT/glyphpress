//! Build cmap format 4 subtables for subset emit.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontWriter;

#[derive(Clone, Debug)]
pub struct CmapMapping {
    pub codepoint: u32,
    pub glyph_id: u16,
}

pub fn build_format4_subtable(mappings: &[CmapMapping]) -> GlyphResult<Vec<u8>> {
    let mut sorted: Vec<_> = mappings.to_vec();
    sorted.sort_by_key(|m| m.codepoint);
    let mut segments: Vec<(u32, u32, u16)> = Vec::new();
    let mut i = 0;
    while i < sorted.len() {
        let start = sorted[i].codepoint;
        let base_gid = sorted[i].glyph_id;
        let mut end = start;
        let mut j = i + 1;
        while j < sorted.len()
            && sorted[j].codepoint == end + 1
            && sorted[j].glyph_id == base_gid.wrapping_add((sorted[j].codepoint - start) as u16)
        {
            end = sorted[j].codepoint;
            j += 1;
        }
        segments.push((start, end, base_gid));
        i = j;
    }
    segments.push((0xFFFF, 0xFFFF, 0));
    let seg_count = segments.len();
    let mut w = FontWriter::with_capacity(256 + seg_count * 8);
    w.write_u16(4)?;
    let length_pos = w.len();
    w.write_u16(0)?;
    w.write_u16(0)?;
    w.write_u16((seg_count * 2) as u16)?;
    let search = smallest_power_of_two(seg_count) * 2;
    w.write_u16(search)?;
    w.write_u16(log2(search / 2))?;
    w.write_u16((seg_count * 2).saturating_sub(search as usize) as u16)?;
    for (_, end, _) in &segments {
        w.write_u16(*end as u16)?;
    }
    w.write_u16(0)?;
    for (start, _, _) in &segments {
        w.write_u16(*start as u16)?;
    }
    for (start, _, gid) in &segments {
        let delta = (*gid as i32 - *start as i32) as i16;
        w.write_i16(delta)?;
    }
    for _ in 0..seg_count {
        w.write_u16(0)?;
    }
    let length = w.len() as u16;
    w.patch_u16(length_pos, length)?;
    Ok(w.into_vec())
}

fn smallest_power_of_two(n: usize) -> u16 {
    let mut p = 1u16;
    while (p as usize) < n {
        p *= 2;
    }
    p
}

fn log2(n: u16) -> u16 {
    let mut v = n;
    let mut r = 0u16;
    while v > 1 {
        v /= 2;
        r += 1;
    }
    r
}
