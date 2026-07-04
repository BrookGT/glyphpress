//! Growable byte blob with optional cached sub-slices.


use crate::error::{GlyphError, GlyphResult};
use crate::limits;

/// Owns font bytes and supports append during emit paths.
pub struct Blob {
    bytes: Vec<u8>,
    /// Cached raw pointer for fast cmap rereads — invalidated on grow unless refreshed.
    cache_ptr: Option<*const u8>,
    cache_len: usize,
}

impl Blob {
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            cache_ptr: None,
            cache_len: 0,
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

    /// Cache a sub-range for repeated decode; pointer survives until next append.
    pub fn cache_subslice(&mut self, offset: usize, len: usize) -> GlyphResult<()> {
        crate::limits::check_len(offset.saturating_add(len), self.bytes.len())?;
        let ptr = unsafe { self.bytes.as_ptr().add(offset) };
        self.cache_ptr = Some(ptr);
        self.cache_len = len;
        Ok(())
    }

    /// Return cached slice without revalidating against current buffer (BUG: stale after grow).
    pub unsafe fn cached_slice(&self) -> &[u8] {
        unsafe {
            let ptr = self.cache_ptr.expect("cache_subslice not called");
            core::slice::from_raw_parts(ptr, self.cache_len)
        }
    }

    pub fn clear_cache(&mut self) {
        self.cache_ptr = None;
        self.cache_len = 0;
    }
}

impl Default for Blob {
    fn default() -> Self {
        Self::from_vec(Vec::new())
    }
}
