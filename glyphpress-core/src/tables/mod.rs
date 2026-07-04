//! OpenType table parsers.


pub mod cmap;
pub mod gdef;
pub mod glyf;
pub mod gpos;
pub mod gsub;
pub mod opentype_reference;
pub mod spec_corpus;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod layout_common;
pub mod kern;
pub mod macintosh;
pub mod loca;
pub mod maxp;
pub mod name;
pub mod os2;
pub mod post;

pub use head::HeadTable;
pub use hhea::HheaTable;
pub use maxp::MaxpTable;
pub use cmap::CmapTable;
pub use glyf::GlyfTable;
pub use loca::LocaTable;
pub use hmtx::HmtxTable;
pub use name::NameTable;
pub use os2::Os2Table;
pub use post::PostTable;
pub use kern::KernTable;
pub use gdef::GdefTable;
pub use gpos::GposTable;
pub use gsub::GsubTable;
