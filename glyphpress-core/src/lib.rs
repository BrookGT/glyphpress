//! glyphpress-core — SFNT font parsing, validation, and subsetting.


#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(clippy::too_many_arguments)]

pub mod check;
pub mod cmap;
pub mod emit;
pub mod error;
pub mod io;
pub mod limits;
pub mod mem;
pub mod outline;
pub mod pipeline;
pub mod sfnt;
pub mod subset;
pub mod tables;

pub use error::{GlyphError, GlyphResult};
pub use pipeline::load::LoadedFont;
pub use pipeline::subset_run::{SubsetOptions, SubsetReport};
pub use subset::plan::SubsetPlan;

/// Build identifier string for diagnostics.
pub const GLYPHPRESS_BUILD: &str = env!("CARGO_PKG_VERSION");

/// Four-byte OpenType table tag as big-endian u32.
#[inline]
pub fn tag_from_bytes(b: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*b)
}

/// Render a table tag for diagnostics.
pub fn tag_to_str(tag: u32) -> [u8; 4] {
    tag.to_be_bytes()
}
