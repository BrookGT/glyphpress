//! Contour assembly from scratch arena.


use crate::error::{GlyphError, GlyphResult};
use crate::mem::ScratchArena;
use crate::outline::point::Point;

#[derive(Clone, Debug, Default)]
pub struct ContourSet {
    pub points: Vec<Point>,
    pub ends: Vec<u16>,
}

impl ContourSet {
    pub fn from_scratch(arena: &ScratchArena) -> GlyphResult<Self> {
        let n = arena.point_count();
        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            points.push(arena.point_at(i)?);
        }
        Ok(Self { points, ends: arena.contours().to_vec() })
    }

    pub fn contour_count(&self) -> usize {
        self.ends.len()
    }

    pub fn contour_points(&self, idx: usize) -> GlyphResult<&[Point]> {
        if idx >= self.ends.len() {
            return Err(GlyphError::OutlineInvalid { gid: 0, reason: "contour index" });
        }
        let start = if idx == 0 { 0 } else { self.ends[idx - 1] as usize + 1 };
        let end = self.ends[idx] as usize + 1;
        Ok(&self.points[start..end])
    }

    pub fn transform(&mut self, tx: i32, ty: i32) {
        for p in &mut self.points {
            p.x = p.x.wrapping_add(tx as i16);
            p.y = p.y.wrapping_add(ty as i16);
        }
    }
}
