//! Binary I/O helpers for big-endian SFNT data.


pub mod checksum;
pub mod reader;
pub mod writer;

pub use reader::FontReader;
pub use writer::FontWriter;
