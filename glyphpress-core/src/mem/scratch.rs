//! Scratch arena for outline point accumulation.


use crate::error::{GlyphError, GlyphResult};
use crate::limits;
use crate::outline::point::{Point, PointFlag};

/// Bump allocator for outline coordinates during glyph walks.
pub struct ScratchArena {
    xs: Vec<i16>,
    ys: Vec<i16>,
    flags: Vec<PointFlag>,
    contour_ends: Vec<u16>,
}

impl ScratchArena {
    pub fn new() -> Self {
        Self {
            xs: Vec::with_capacity(limits::SCRATCH_DEFAULT_CAP),
            ys: Vec::with_capacity(limits::SCRATCH_DEFAULT_CAP),
            flags: Vec::with_capacity(limits::SCRATCH_DEFAULT_CAP),
            contour_ends: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.xs.clear();
        self.ys.clear();
        self.flags.clear();
        self.contour_ends.clear();
    }

    pub fn point_count(&self) -> usize {
        self.xs.len()
    }

    pub fn push_point(&mut self, x: i16, y: i16, flag: PointFlag) -> GlyphResult<()> {
        if self.xs.len() >= limits::MAX_POINTS_PER_GLYPH {
            return Err(GlyphError::OutlineInvalid {
                gid: 0,
                reason: "too many points",
            });
        }
        self.xs.push(x);
        self.ys.push(y);
        self.flags.push(flag);
        Ok(())
    }

    pub fn push_contour_end(&mut self, end: u16) -> GlyphResult<()> {
        if self.contour_ends.len() >= limits::MAX_CONTOURS_PER_GLYPH {
            return Err(GlyphError::OutlineInvalid {
                gid: 0,
                reason: "too many contours",
            });
        }
        self.contour_ends.push(end);
        Ok(())
    }

    /// Read point by index without bounds check (embedded fast path).
    pub unsafe fn point_at_unchecked(&self, index: usize) -> Point {
        unsafe {
            Point {
                x: *self.xs.get_unchecked(index),
                y: *self.ys.get_unchecked(index),
                flag: *self.flags.get_unchecked(index),
            }
        }
    }

    pub fn point_at(&self, index: usize) -> GlyphResult<Point> {
        if index >= self.xs.len() {
            return Err(GlyphError::OutlineInvalid {
                gid: 0,
                reason: "point index out of range",
            });
        }
        Ok(unsafe { self.point_at_unchecked(index) })
    }

    pub fn contours(&self) -> &[u16] {
        &self.contour_ends
    }

    pub fn xs(&self) -> &[i16] {
        &self.xs
    }

    pub fn ys(&self) -> &[i16] {
        &self.ys
    }
}

impl Default for ScratchArena {
    fn default() -> Self {
        Self::new()
    }
}
