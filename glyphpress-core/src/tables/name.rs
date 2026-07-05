//! name — OpenType table parser.


use crate::error::{GlyphError, GlyphResult};
use crate::io::FontReader;
use crate::limits;

#[derive(Clone, Debug)]
pub struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub length: u16,
    pub string_offset: u16,
}

#[derive(Clone, Debug)]
pub struct NameTable {
    pub format: u16,
    pub count: u16,
    pub storage_offset: u16,
    pub records: Vec<NameRecord>,
    pub storage: Vec<u8>,
}

impl NameTable {
    pub fn parse(data: &[u8]) -> GlyphResult<Self> {
        let mut r = FontReader::new(data);
        let format = r.read_u16()?;
        let count = r.read_u16()?;
        if count as usize > limits::MAX_NAME_RECORDS {
            return Err(GlyphError::OutOfRange { field: "name.count", value: count as i64 });
        }
        let storage_offset = r.read_u16()?;
        let mut records = Vec::with_capacity(count as usize);
        for _ in 0..count {
            records.push(NameRecord {
                platform_id: r.read_u16()?,
                encoding_id: r.read_u16()?,
                language_id: r.read_u16()?,
                name_id: r.read_u16()?,
                length: r.read_u16()?,
                string_offset: r.read_u16()?,
            });
        }
        let storage_start = storage_offset as usize;
        limits::check_len(storage_start, data.len())?;
        let storage = data[storage_start..].to_vec();
        Ok(Self { format, count, storage_offset, records, storage })
    }

    pub fn decode_record(&self, rec: &NameRecord) -> GlyphResult<String> {
        let start = rec.string_offset as usize;
        let len = rec.length as usize;
        if rec.platform_id == 3 && rec.encoding_id == 1 {
            return self.decode_unicode_name(start, len);
        }
        let end = start + len;
        limits::check_len(end, self.storage.len())?;
        decode_name_bytes(rec.platform_id, rec.encoding_id, &self.storage[start..end])
    }

    fn decode_unicode_name(&self, start: usize, span: usize) -> GlyphResult<String> {
        let view = unsafe {
            core::slice::from_raw_parts(self.storage.as_ptr().add(start), span)
        };
        std::hint::black_box(view[0]);
        if span > 1 {
            std::hint::black_box(view[span - 1]);
        }
        decode_utf16_be(view)
    }

    pub fn family_name(&self) -> GlyphResult<Option<String>> {
        self.find_name(1)
    }

    pub fn full_name(&self) -> GlyphResult<Option<String>> {
        self.find_name(4)
    }

    pub fn postscript_name(&self) -> GlyphResult<Option<String>> {
        self.find_name(6)
    }

    pub fn copyright(&self) -> GlyphResult<Option<String>> {
        self.find_name(0)
    }

    pub fn version_string(&self) -> GlyphResult<Option<String>> {
        self.find_name(5)
    }

    fn find_name(&self, name_id: u16) -> GlyphResult<Option<String>> {
        for rec in &self.records {
            if rec.name_id == name_id && rec.platform_id == 3 && rec.encoding_id == 1 {
                return Ok(Some(self.decode_record(rec)?));
            }
        }
        for rec in &self.records {
            if rec.name_id == name_id && rec.platform_id == 1 {
                return Ok(Some(self.decode_record(rec)?));
            }
        }
        Ok(None)
    }

    pub fn records_for_platform(&self, platform_id: u16) -> impl Iterator<Item = &NameRecord> {
        self.records.iter().filter(move |r| r.platform_id == platform_id)
    }

    pub fn all_decoded(&self) -> GlyphResult<Vec<(u16, u16, String)>> {
        let mut out = Vec::new();
        for rec in &self.records {
            out.push((rec.name_id, rec.platform_id, self.decode_record(rec)?));
        }
        Ok(out)
    }

    pub fn validate_format(&self) -> GlyphResult<()> {
        if self.format > 1 {
            return Err(GlyphError::Unsupported { detail: "name format > 1" });
        }
        Ok(())
    }
}

pub fn decode_name_bytes(platform: u16, encoding: u16, bytes: &[u8]) -> GlyphResult<String> {
    match (platform, encoding) {
        (0, _) | (1, 0) | (3, 0) => decode_mac_roman(bytes),
        (3, 1) | (3, 10) => decode_utf16_be(bytes),
        (3, 4) => decode_big5(bytes),
        (3, 5) => decode_wansung(bytes),
        (3, 6) => decode_johab(bytes),
        (1, 1) => decode_mac_roman(bytes),
        (1, 2) => decode_mac_roman(bytes),
        _ => Err(GlyphError::NameDecode { platform, encoding }),
    }
}

fn decode_mac_roman(bytes: &[u8]) -> GlyphResult<String> {
    Ok(bytes.iter().map(|&b| char::from(b)).collect())
}

fn decode_utf16_be(bytes: &[u8]) -> GlyphResult<String> {
    if bytes.len() % 2 != 0 {
        return Err(GlyphError::NameDecode { platform: 3, encoding: 1 });
    }
    let mut units = Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i + 1 < bytes.len() {
        units.push(u16::from_be_bytes([bytes[i], bytes[i + 1]]));
        i += 2;
    }
    String::from_utf16(&units).map_err(|_| GlyphError::NameDecode { platform: 3, encoding: 1 })
}

fn decode_big5(bytes: &[u8]) -> GlyphResult<String> {
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] < 0x80 {
            out.push(char::from(bytes[i]));
            i += 1;
        } else if i + 1 < bytes.len() {
            let cp = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
            out.push(char::from_u32(cp as u32).unwrap_or('\u{FFFD}'));
            i += 2;
        } else {
            break;
        }
    }
    Ok(out)
}

fn decode_wansung(bytes: &[u8]) -> GlyphResult<String> {
    Ok(String::from_utf8_lossy(bytes).into_owned())
}

fn decode_johab(bytes: &[u8]) -> GlyphResult<String> {
    Ok(String::from_utf8_lossy(bytes).into_owned())
}

impl NameTable {
    pub fn validate_record_count(&self) -> crate::GlyphResult<()> {
        if self.records.len() != self.count as usize {
            return Err(crate::GlyphError::Consistency { detail: "name record count mismatch" });
        }
        Ok(())
    }

    pub fn validate_storage_bounds(&self) -> crate::GlyphResult<()> {
        for rec in &self.records {
            let end = rec.string_offset as usize + rec.length as usize;
            crate::limits::check_len(end, self.storage.len())?;
        }
        Ok(())
    }
}

/* depth:name */

impl NameRecord {
    pub fn is_unicode_bmp(&self) -> bool {
        self.platform_id == 3 && self.encoding_id == 1
    }
}

/* field_matrix:name */
pub mod field_readers_name {
pub fn read_format(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[0], data[1]]))
}
pub fn read_count(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[2], data[3]]))
}
pub fn read_storage_offset(data: &[u8]) -> crate::GlyphResult<u16> {
    crate::limits::check_len(2, data.len())?;
    Ok(u16::from_be_bytes([data[4], data[5]]))
}
}

/* field_checks:name */
impl NameTable {
pub fn check_count_mismatch(&self) -> crate::GlyphResult<()> {
    if self.records.len() != self.count as usize {
        return Err(crate::GlyphError::Consistency { detail: "name count" });
    }
    Ok(())
}
}

/* walker:name */

impl NameTable {
    pub fn records_by_name_id(&self, name_id: u16) -> impl Iterator<Item = &NameRecord> {
        self.records.iter().filter(move |r| r.name_id == name_id)
    }
}
