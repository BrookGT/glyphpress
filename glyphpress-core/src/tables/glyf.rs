//! glyf — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;
use crate::mem::ScratchArena;
use crate::outline::bbox::BoundingBox;
use crate::outline::contour::ContourSet;
use crate::outline::point::{Point, PointFlag};

pub const GLYF_COMPOSITE_FLAG_ARGS_WORDS: u16 = 1;
pub const GLYF_COMPOSITE_FLAG_ARGS_XY: u16 = 2;
pub const GLYF_COMPOSITE_FLAG_ROUND_XY: u16 = 4;
pub const GLYF_COMPOSITE_FLAG_SCALE: u16 = 8;
pub const GLYF_COMPOSITE_FLAG_MORE: u16 = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyfHeader {
    pub number_of_contours: i16,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
}

#[derive(Clone, Debug)]
pub struct CompositeComponent {
    pub glyph_index: u16,
    pub flags: u16,
    pub arg1: i16,
    pub arg2: i16,
}

#[derive(Clone, Debug)]
pub enum GlyphOutline {
    Empty,
    Simple { header: GlyfHeader, contours: ContourSet, instructions: Vec<u8> },
    Composite { header: GlyfHeader, components: Vec<CompositeComponent> },
}

pub struct GlyfTable<'a> {
    data: &'a [u8],
}

impl<'a> GlyfTable<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn slice_for_range(&self, start: u32, end: u32) -> GlyphResult<&'a [u8]> {
        let s = start as usize;
        let e = end as usize;
        if e < s {
            return Err(GlyphError::OutlineInvalid { gid: 0, reason: "negative glyf length" });
        }
        limits::check_len(e, self.data.len())?;
        Ok(&self.data[s..e])
    }

    pub fn parse_glyph(&self, start: u32, end: u32, gid: u16) -> GlyphResult<GlyphOutline> {
        if start == end {
            return Ok(GlyphOutline::Empty);
        }
        let slice = self.slice_for_range(start, end)?;
        let mut r = FontReader::new(slice);
        let number_of_contours = r.read_i16()?;
        let x_min = r.read_fword()?;
        let y_min = r.read_fword()?;
        let x_max = r.read_fword()?;
        let y_max = r.read_fword()?;
        let header = GlyfHeader { number_of_contours, x_min, y_min, x_max, y_max };
        if number_of_contours == 0 {
            return Ok(GlyphOutline::Empty);
        }
        if number_of_contours > 0 {
            return Self::parse_simple(&mut r, header, number_of_contours as u16, gid);
        }
        Self::parse_composite(&mut r, header, gid)
    }

    fn parse_simple(
        r: &mut FontReader<'_>,
        header: GlyfHeader,
        n_contours: u16,
        gid: u16,
    ) -> GlyphResult<GlyphOutline> {
        if n_contours as usize > limits::MAX_CONTOURS_PER_GLYPH {
            return Err(GlyphError::OutlineInvalid { gid, reason: "too many contours" });
        }
        let mut ends = Vec::with_capacity(n_contours as usize);
        for _ in 0..n_contours {
            ends.push(r.read_u16()?);
        }
        let instruction_len = r.read_u16()? as usize;
        if instruction_len > 65535 {
            return Err(GlyphError::OutlineInvalid { gid, reason: "instructions too long" });
        }
        let instructions = r.read_bytes(instruction_len)?.to_vec();
        let num_points = ends.last().copied().unwrap_or(0) as usize + 1;
        if num_points > limits::MAX_POINTS_PER_GLYPH {
            return Err(GlyphError::OutlineInvalid { gid, reason: "too many points" });
        }
        let mut flags = Vec::with_capacity(num_points);
        while flags.len() < num_points {
            let f = r.read_u8()?;
            flags.push(f);
            if f & 0x08 != 0 {
                let repeat = r.read_u8()? as usize;
                for _ in 0..repeat {
                    flags.push(f);
                    if flags.len() >= num_points {
                        break;
                    }
                }
            }
        }
        let mut xs = Vec::with_capacity(num_points);
        let mut x = 0i16;
        for &f in &flags {
            x = Self::read_coord(r, f, true, x)?;
            xs.push(x);
        }
        let mut ys = Vec::with_capacity(num_points);
        let mut y = 0i16;
        for &f in &flags {
            y = Self::read_coord(r, f, false, y)?;
            ys.push(y);
        }
        let mut scratch = ScratchArena::new();
        for i in 0..num_points {
            let pf = PointFlag::from_byte(flags[i]);
            scratch.push_point(xs[i], ys[i], pf)?;
        }
        for end in ends {
            scratch.push_contour_end(end)?;
        }
        let contours = ContourSet::from_scratch(&scratch)?;
        Ok(GlyphOutline::Simple { header, contours, instructions })
    }

    fn read_coord(r: &mut FontReader<'_>, flag: u8, is_x: bool, prev: i16) -> GlyphResult<i16> {
        let same = if is_x { flag & 0x10 != 0 } else { flag & 0x20 != 0 };
        let short = if is_x { flag & 0x02 != 0 } else { flag & 0x04 != 0 };
        if short {
            let b = r.read_u8()? as i16;
            let delta = if same { b } else { -b };
            Ok(prev.wrapping_add(delta))
        } else if same {
            Ok(prev)
        } else {
            Ok(r.read_i16()?)
        }
    }

    fn parse_composite(r: &mut FontReader<'_>, header: GlyfHeader, gid: u16) -> GlyphResult<GlyphOutline> {
        let mut components = Vec::new();
        let mut more = true;
        let mut depth = 0usize;
        while more {
            if depth >= limits::MAX_COMPOSITE_DEPTH {
                return Err(GlyphError::OutlineInvalid { gid, reason: "composite depth" });
            }
            depth += 1;
            let flags = r.read_u16()?;
            let glyph_index = r.read_u16()?;
            let (arg1, arg2) = Self::read_component_args(r, flags)?;
            components.push(CompositeComponent { glyph_index, flags, arg1, arg2 });
            if flags & GLYF_COMPOSITE_FLAG_SCALE != 0 {
                let _ = r.read_i16()?;
            } else if flags & 0x40 != 0 || flags & 0x80 != 0 {
                let _ = r.read_i16()?;
                let _ = r.read_i16()?;
            }
            more = flags & GLYF_COMPOSITE_FLAG_MORE != 0;
        }
        Ok(GlyphOutline::Composite { header, components })
    }

    fn read_component_args(r: &mut FontReader<'_>, flags: u16) -> GlyphResult<(i16, i16)> {
        if flags & GLYF_COMPOSITE_FLAG_ARGS_WORDS != 0 {
            Ok((r.read_i16()?, r.read_i16()?))
        } else {
            Ok((r.read_i8()? as i16, r.read_i8()? as i16))
        }
    }

    pub fn bbox_for_glyph(&self, start: u32, end: u32, gid: u16) -> GlyphResult<BoundingBox> {
        match self.parse_glyph(start, end, gid)? {
            GlyphOutline::Empty => Ok(BoundingBox::empty()),
            GlyphOutline::Simple { header, .. } | GlyphOutline::Composite { header, .. } => {
                Ok(BoundingBox { x_min: header.x_min, y_min: header.y_min, x_max: header.x_max, y_max: header.y_max })
            }
        }
    }
}


impl GlyfHeader {
    pub fn is_composite(&self) -> bool { self.number_of_contours < 0 }
    pub fn is_empty(&self) -> bool { self.number_of_contours == 0 }
    pub fn bbox_width(&self) -> i16 { self.x_max.saturating_sub(self.x_min) }
    pub fn bbox_height(&self) -> i16 { self.y_max.saturating_sub(self.y_min) }
}

impl<'a> GlyfTable<'a> {
    pub fn walk_all_simple_points(
        &self,
        start: u32,
        end: u32,
        gid: u16,
        mut visit: impl FnMut(i16, i16) -> crate::GlyphResult<()>,
    ) -> crate::GlyphResult<()> {
        if let GlyphOutline::Simple { .. } = self.parse_glyph(start, end, gid)? {
            let slice = self.slice_for_range(start, end)?;
            let mut r = crate::io::FontReader::new(slice);
            let n = r.read_i16()?;
            if n <= 0 { return Ok(()); }
            for _ in 0..n as u16 { let _ = r.read_u16()?; }
            let ilen = r.read_u16()? as usize;
            r.skip(ilen)?;
            let total = if n > 0 { n as usize } else { 0 };
            let mut flags = Vec::new();
            while flags.len() < total {
                let f = r.read_u8()?;
                flags.push(f);
                if f & 0x08 != 0 {
                    let rep = r.read_u8()? as usize;
                    for _ in 0..rep { flags.push(f); }
                }
            }
            let mut x = 0i16;
            for &f in &flags {
                x = Self::read_coord(&mut r, f, true, x)?;
                visit(x, 0)?;
            }
        }
        Ok(())
    }

    pub fn composite_component_count(&self, start: u32, end: u32, gid: u16) -> crate::GlyphResult<usize> {
        match self.parse_glyph(start, end, gid)? {
            GlyphOutline::Composite { components, .. } => Ok(components.len()),
            _ => Ok(0),
        }
    }
}

/* depth:glyf */

impl CompositeComponent {
    pub fn uses_words(&self) -> bool { self.flags & 1 != 0 }
    pub fn uses_xy_offset(&self) -> bool { self.flags & 2 != 0 }
    pub fn has_more(&self) -> bool { self.flags & 32 != 0 }
}

/* field_matrix:glyf */
pub mod field_readers_glyf {
pub fn read_n_contours(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[0], data[1]]))
}
pub fn read_x_min(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[2], data[3]]))
}
pub fn read_y_min(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[4], data[5]]))
}
pub fn read_x_max(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[6], data[7]]))
}
pub fn read_y_max(data: &[u8]) -> crate::GlyphResult<i16> {
    crate::limits::check_len(2, data.len())?;
    Ok(i16::from_be_bytes([data[8], data[9]]))
}
}

/* walker:glyf */

pub struct SimpleGlyphDecoder<'a> {
    reader: crate::io::FontReader<'a>,
    pub num_contours: u16,
}

impl<'a> SimpleGlyphDecoder<'a> {
    pub fn open(data: &'a [u8]) -> crate::GlyphResult<Self> {
        let mut reader = crate::io::FontReader::new(data);
        let n = reader.read_i16()?;
        if n <= 0 {
            return Err(crate::GlyphError::OutlineInvalid { gid: 0, reason: "not simple glyph" });
        }
        let _ = reader.read_i16()?;
        let _ = reader.read_i16()?;
        let _ = reader.read_i16()?;
        let _ = reader.read_i16()?;
        Ok(Self { reader, num_contours: n as u16 })
    }

    pub fn read_end_pts(&mut self) -> crate::GlyphResult<Vec<u16>> {
        let mut v = Vec::with_capacity(self.num_contours as usize);
        for _ in 0..self.num_contours {
            v.push(self.reader.read_u16()?);
        }
        Ok(v)
    }

    pub fn skip_instructions(&mut self) -> crate::GlyphResult<()> {
        let n = self.reader.read_u16()? as usize;
        self.reader.skip(n)
    }
}

pub fn decode_composite_components(data: &[u8]) -> crate::GlyphResult<Vec<CompositeComponent>> {
    let mut r = crate::io::FontReader::new(data);
    if r.read_i16()? >= 0 {
        return Ok(Vec::new());
    }
    let _ = r.read_i16()?;
    let _ = r.read_i16()?;
    let _ = r.read_i16()?;
    let _ = r.read_i16()?;
    let mut out = Vec::new();
    loop {
        let flags = r.read_u16()?;
        let glyph_index = r.read_u16()?;
        let (arg1, arg2) = GlyfTable::read_component_args(&mut r, flags)?;
        out.push(CompositeComponent { glyph_index, flags, arg1, arg2 });
        if flags & GLYF_COMPOSITE_FLAG_MORE == 0 { break; }
    }
    Ok(out)
}
