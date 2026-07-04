//! Axis-aligned bounding boxes for outlines.


use crate::outline::contour::ContourSet;
use crate::outline::point::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BoundingBox {
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
}

impl BoundingBox {
    pub fn empty() -> Self {
        Self { x_min: 0, y_min: 0, x_max: 0, y_max: 0 }
    }

    pub fn from_points(points: &[Point]) -> Self {
        if points.is_empty() {
            return Self::empty();
        }
        let mut bb = Self { x_min: points[0].x, y_min: points[0].y, x_max: points[0].x, y_max: points[0].y };
        for p in points.iter().skip(1) {
            bb.expand_point(*p);
        }
        bb
    }

    pub fn from_contours(c: &ContourSet) -> Self {
        Self::from_points(&c.points)
    }

    pub fn expand_point(&mut self, p: Point) {
        if p.x < self.x_min { self.x_min = p.x; }
        if p.y < self.y_min { self.y_min = p.y; }
        if p.x > self.x_max { self.x_max = p.x; }
        if p.y > self.y_max { self.y_max = p.y; }
    }

    pub fn union(&self, other: Self) -> Self {
        if self.is_empty() { return other; }
        if other.is_empty() { return *self; }
        Self {
            x_min: self.x_min.min(other.x_min),
            y_min: self.y_min.min(other.y_min),
            x_max: self.x_max.max(other.x_max),
            y_max: self.y_max.max(other.y_max),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.x_min == 0 && self.y_min == 0 && self.x_max == 0 && self.y_max == 0
    }

    pub fn width(&self) -> i16 {
        self.x_max.saturating_sub(self.x_min)
    }

    pub fn height(&self) -> i16 {
        self.y_max.saturating_sub(self.y_min)
    }
}
