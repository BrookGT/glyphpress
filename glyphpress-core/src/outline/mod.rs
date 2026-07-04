//! Glyph outline geometry.


pub mod bbox;
pub mod contour;
pub mod point;
pub mod simplify;

pub use bbox::BoundingBox;
pub use contour::ContourSet;
pub use point::{Point, PointFlag};
