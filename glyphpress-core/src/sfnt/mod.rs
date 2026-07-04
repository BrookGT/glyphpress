//! SFNT container: offset table and directory.


pub mod directory;
pub mod header;
pub mod table_record;

pub use directory::TableDirectory;
pub use header::SfntHeader;
pub use table_record::TableRecord;
