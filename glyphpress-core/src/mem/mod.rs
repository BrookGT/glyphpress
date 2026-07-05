//! Memory substrate for zero-copy table views.


pub mod blob;
pub mod layout_scratch;
pub mod scratch;

pub use blob::Blob;
pub use layout_scratch::LayoutScratch;
pub use scratch::ScratchArena;
