//! Growable byte blob with optional cached sub-slices.


use crate::error::{GlyphError, GlyphResult};
use crate::limits;

/// Owns font bytes and supports append during emit paths.
pub struct Blob {
    bytes: Vec<u8>,
    span_origin: Option<*const u8>,
    span_extent: usize,
}

impl Blob {
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            span_origin: None,
            span_extent: 0,
        }
    }

    pub fn from_static(data: &'static [u8]) -> Self {
        Self::from_vec(data.to_vec())
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn append(&mut self, data: &[u8]) -> GlyphResult<()> {
        if self.bytes.len().saturating_add(data.len()) > limits::MAX_FONT_BYTES {
            return Err(GlyphError::EmitOverflow { table: "blob" });
        }
        self.bytes.extend_from_slice(data);
        Ok(())
    }

    pub fn reserve(&mut self, extra: usize) {
        self.bytes.reserve(extra);
    }

    pub fn snapshot_span(&mut self, offset: usize, len: usize) -> GlyphResult<()> {
        crate::limits::check_len(offset.saturating_add(len), self.bytes.len())?;
        let ptr = unsafe { self.bytes.as_ptr().add(offset) };
        self.span_origin = Some(ptr);
        self.span_extent = len;
        Ok(())
    }

    pub unsafe fn snapshot_view(&self) -> &[u8] {
        unsafe {
            let ptr = self.span_origin.expect("snapshot_span not called");
            core::slice::from_raw_parts(ptr, self.span_extent)
        }
    }

    pub fn clear_snapshot(&mut self) {
        self.span_origin = None;
        self.span_extent = 0;
    }

    #[allow(dead_code)]
    pub fn cache_subslice(&mut self, offset: usize, len: usize) -> GlyphResult<()> {
        self.snapshot_span(offset, len)
    }

    #[allow(dead_code)]
    pub unsafe fn cached_slice(&self) -> &[u8] {
        unsafe { self.snapshot_view() }
    }
}

impl Default for Blob {
    fn default() -> Self {
        Self::from_vec(Vec::new())
    }
}
