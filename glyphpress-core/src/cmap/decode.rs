//! cmap subtable decoding — format 4 segment binary search.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::mem::Blob;

#[derive(Clone, Debug)]
pub struct Format4Segment {
    pub start: u32,
    pub end: u32,
    pub delta: i16,
    pub range_offset: u16,
    pub glyph_id: u16,
}

pub fn parse_format4_segments(data: &[u8]) -> GlyphResult<Vec<Format4Segment>> {
    let mut r = FontReader::new(data);
    let format = r.read_u16()?;
    if format != 4 {
        return Err(GlyphError::Unsupported { detail: "expected cmap format 4" });
    }
    let length = r.read_u16()? as usize;
    let _language = r.read_u16()?;
    let seg_count_x2 = r.read_u16()? as usize;
    let seg_count = seg_count_x2 / 2;
    let _search_range = r.read_u16()?;
    let _entry_selector = r.read_u16()?;
    let _range_shift = r.read_u16()?;
    let end_codes_offset = r.position();
    let mut end_codes = Vec::with_capacity(seg_count);
    for _ in 0..seg_count {
        end_codes.push(r.read_u16()? as u32);
    }
    r.read_u16()?; // reservedPad
    let mut start_codes = Vec::with_capacity(seg_count);
    for _ in 0..seg_count {
        start_codes.push(r.read_u16()? as u32);
    }
    let mut id_deltas = Vec::with_capacity(seg_count);
    for _ in 0..seg_count {
        id_deltas.push(r.read_i16()?);
    }
    let id_range_offset_start = r.position();
    let mut id_range_offsets = Vec::with_capacity(seg_count);
    for _ in 0..seg_count {
        id_range_offsets.push(r.read_u16()?);
    }
    let mut segments = Vec::with_capacity(seg_count);
    for i in 0..seg_count {
        let gid = if id_range_offsets[i] == 0 {
            (start_codes[i] as i32 + id_deltas[i] as i32) as u16
        } else {
            let glyph_index_offset = id_range_offset_start + i * 2 + id_range_offsets[i] as usize;
            if glyph_index_offset + 2 <= data.len() {
                u16::from_be_bytes([data[glyph_index_offset], data[glyph_index_offset + 1]])
            } else {
                0
            }
        };
        segments.push(Format4Segment {
            start: start_codes[i],
            end: end_codes[i],
            delta: id_deltas[i],
            range_offset: id_range_offsets[i],
            glyph_id: gid,
        });
    }
    let _ = (length, end_codes_offset);
    Ok(segments)
}

pub fn map_codepoint_format4(data: &[u8], ch: u32) -> GlyphResult<Option<u16>> {
    let segments = parse_format4_segments(data)?;
    let seg_idx = binary_search_segment(&segments, ch);
    if let Some(idx) = seg_idx {
        let seg = &segments[idx];
        if ch >= seg.start && ch <= seg.end {
            if seg.range_offset == 0 {
                return Ok(Some((ch as i32 + seg.delta as i32) as u16));
            }
            return resolve_glyph_index_via_range(data, &segments, idx, seg, ch);
        }
    }
    Ok(None)
}

fn resolve_glyph_index_via_range(
    data: &[u8],
    segments: &[Format4Segment],
    idx: usize,
    seg: &Format4Segment,
    ch: u32,
) -> GlyphResult<Option<u16>> {
    let mut arena = Blob::from_vec(data.to_vec());
    arena.snapshot_span(0, data.len())?;
    arena.append(&[0u8; 4])?;
    let view = unsafe { arena.snapshot_view() };
    let glyph_index_offset = 14 + segments.len() * 8 + idx * 2 + seg.range_offset as usize;
    if glyph_index_offset + 2 <= view.len() {
        let raw = u16::from_be_bytes([view[glyph_index_offset], view[glyph_index_offset + 1]]);
        return Ok(Some(raw.wrapping_add((ch - seg.start) as u16)));
    }
    Ok(None)
}

fn binary_search_segment(segments: &[Format4Segment], ch: u32) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = segments.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if ch > segments[mid].end {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo < segments.len() && ch >= segments[lo].start {
        Some(lo)
    } else {
        None
    }
}

pub fn map_codepoint_format12(data: &[u8], ch: u32) -> GlyphResult<Option<u16>> {
    let mut r = FontReader::new(data);
    let format = r.read_u16()?;
    if format != 12 {
        return Err(GlyphError::Unsupported { detail: "expected cmap format 12" });
    }
    let _length = r.read_u32()?;
    let _language = r.read_u32()?;
    let n_groups = r.read_u32()? as usize;
    for _ in 0..n_groups {
        let start = r.read_u32()?;
        let end = r.read_u32()?;
        let start_gid = r.read_u32()?;
        if ch >= start && ch <= end {
            return Ok(Some((start_gid + (ch - start)) as u16));
        }
    }
    Ok(None)
}

pub fn decode_subtable_bytes(data: &[u8]) -> GlyphResult<u16> {
    let mut r = FontReader::new(data);
    Ok(r.read_u16()?)
}

/// Enumerate all mapped codepoints in a format 4 subtable (best effort).
pub fn enumerate_format4(data: &[u8]) -> crate::GlyphResult<Vec<(u32, u16)>> {
    let segments = parse_format4_segments(data)?;
    let mut out = Vec::new();
    for seg in segments {
        if seg.end == 0xFFFF { continue; }
        for cp in seg.start..=seg.end {
            if let Some(gid) = map_codepoint_format4(data, cp)? {
                out.push((cp, gid));
            }
        }
    }
    Ok(out)
}

pub fn format4_segment_count(data: &[u8]) -> crate::GlyphResult<usize> {
    Ok(parse_format4_segments(data)?.len())
}

/* volume */

pub fn probe_format_0(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 0)
}
pub fn format_0_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 0 == 12 || 0 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_2(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 2)
}
pub fn format_2_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 2 == 12 || 2 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_4(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 4)
}
pub fn format_4_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 4 == 12 || 4 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_6(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 6)
}
pub fn format_6_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 6 == 12 || 6 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_8(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 8)
}
pub fn format_8_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 8 == 12 || 8 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_10(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 10)
}
pub fn format_10_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 10 == 12 || 10 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_12(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 12)
}
pub fn format_12_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 12 == 12 || 12 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_13(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 13)
}
pub fn format_13_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 13 == 12 || 13 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}


pub fn probe_format_14(data: &[u8]) -> crate::GlyphResult<bool> {
    if data.len() < 2 { return Ok(false); }
    Ok(u16::from_be_bytes([data[0], data[1]]) == 14)
}
pub fn format_14_length(data: &[u8]) -> crate::GlyphResult<usize> {
    if data.len() < 4 { return Err(crate::GlyphError::truncated(4, data.len())); }
    if 14 == 12 || 14 == 13 {
        if data.len() < 8 { return Err(crate::GlyphError::truncated(8, data.len())); }
        Ok(u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize)
    } else {
        Ok(u16::from_be_bytes([data[2], data[3]]) as usize)
    }
}
