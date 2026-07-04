//! Glyph subset planning and remapping.


pub mod plan;
pub mod prune;
pub mod remap;
pub mod unicode_ranges;

pub use plan::SubsetPlan;
pub use prune::prune_unused_glyphs;
pub use remap::RemapTable;
