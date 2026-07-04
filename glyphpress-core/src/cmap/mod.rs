//! Character map encoding and decoding.


pub mod decode;
pub mod encode;
pub mod subtable;

pub use decode::decode_subtable_bytes;
pub use encode::build_format4_subtable;
pub use subtable::CmapSubtable;
