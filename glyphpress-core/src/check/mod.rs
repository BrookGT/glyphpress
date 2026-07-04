//! Cross-table consistency checks.


pub mod bounds;
pub mod consistency;

pub use bounds::check_bbox_consistency;
pub use consistency::validate_font_consistency;
