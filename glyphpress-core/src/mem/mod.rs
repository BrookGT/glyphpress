//! Memory substrate for zero-copy table views.


pub mod blob;
pub mod scratch;

pub use blob::Blob;
pub use scratch::ScratchArena;
