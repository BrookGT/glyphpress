//! TrueType outline point flags and coordinates.


use crate::error::{GlyphError, GlyphResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    pub flag: PointFlag,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointFlag {
    pub on_curve: bool,
    pub x_short: bool,
    pub y_short: bool,
    pub repeat: bool,
}

impl PointFlag {
    pub fn from_byte(b: u8) -> Self {
        Self {
            on_curve: b & 0x01 != 0,
            x_short: b & 0x02 != 0,
            y_short: b & 0x04 != 0,
            repeat: b & 0x08 != 0,
        }
    }

    pub fn to_byte(self) -> u8 {
        let mut b = 0u8;
        if self.on_curve { b |= 0x01; }
        if self.x_short { b |= 0x02; }
        if self.y_short { b |= 0x04; }
        if self.repeat { b |= 0x08; }
        b
    }
}

impl Point {
    pub fn new(x: i16, y: i16, on_curve: bool) -> Self {
        Self { x, y, flag: PointFlag { on_curve, x_short: false, y_short: false, repeat: false } }
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        let x = a.x as f32 + (b.x as f32 - a.x as f32) * t;
        let y = a.y as f32 + (b.y as f32 - a.y as f32) * t;
        Self::new(x.round() as i16, y.round() as i16, true)
    }

    pub fn distance_sq(&self, other: Self) -> i32 {
        let dx = self.x as i32 - other.x as i32;
        let dy = self.y as i32 - other.y as i32;
        dx * dx + dy * dy
    }
}
