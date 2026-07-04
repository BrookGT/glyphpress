//! Structured errors for SFNT parsing and subsetting.


use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlyphError {
    Truncated { need: usize, have: usize },
    BadMagic { found: u32 },
    BadTableTag { tag: u32 },
    BadTableChecksum { tag: u32, expected: u32, found: u32 },
    DuplicateTable { tag: u32 },
    OutOfRange { field: &'static str, value: i64 },
    Unsupported { detail: &'static str },
    CmapNoUnicode,
    GlyphIndexOutOfRange { gid: u16, max: u16 },
    OutlineInvalid { gid: u16, reason: &'static str },
    SubsetEmpty,
    EmitOverflow { table: &'static str },
    NameDecode { platform: u16, encoding: u16 },
    MetricsMismatch { expected: u16, found: u16 },
    LocaFormatMismatch { head: i16, loca_len: usize },
    Consistency { detail: &'static str },
}

pub type GlyphResult<T> = Result<T, GlyphError>;

impl fmt::Display for GlyphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated { need, have } => {
                write!(f, "need {need} bytes, have {have}")
            }
            Self::BadMagic { found } => write!(f, "bad SFNT magic 0x{found:08X}"),
            Self::BadTableTag { tag } => {
                let t = tag.to_be_bytes();
                write!(f, "unknown table tag {:?}", core::str::from_utf8(&t))
            }
            Self::BadTableChecksum { tag, expected, found } => {
                write!(f, "checksum mismatch for tag 0x{tag:08X}: exp {expected:#010x} got {found:#010x}")
            }
            Self::DuplicateTable { tag } => write!(f, "duplicate table 0x{tag:08X}"),
            Self::OutOfRange { field, value } => {
                write!(f, "{field} out of range: {value}")
            }
            Self::Unsupported { detail } => write!(f, "unsupported: {detail}"),
            Self::CmapNoUnicode => write!(f, "no usable Unicode cmap subtable"),
            Self::GlyphIndexOutOfRange { gid, max } => {
                write!(f, "glyph id {gid} exceeds max {max}")
            }
            Self::OutlineInvalid { gid, reason } => {
                write!(f, "glyph {gid} outline invalid: {reason}")
            }
            Self::SubsetEmpty => write!(f, "subset plan contains no glyphs"),
            Self::EmitOverflow { table } => write!(f, "emit overflow in {table}"),
            Self::NameDecode { platform, encoding } => {
                write!(f, "name decode failed platform {platform} encoding {encoding}")
            }
            Self::MetricsMismatch { expected, found } => {
                write!(f, "metrics count mismatch exp {expected} got {found}")
            }
            Self::LocaFormatMismatch { head, loca_len } => {
                write!(f, "loca length {loca_len} incompatible with indexToLocFormat {head}")
            }
            Self::Consistency { detail } => write!(f, "consistency: {detail}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GlyphError {}

impl GlyphError {
    pub fn truncated(need: usize, have: usize) -> Self {
        Self::Truncated { need, have }
    }
}
