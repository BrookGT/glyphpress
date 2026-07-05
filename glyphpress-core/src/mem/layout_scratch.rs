//! Cross-table layout staging buffer with pinned lookup views.


use crate::error::{GlyphError, GlyphResult};
use crate::mem::Blob;

/// Retains GSUB lookup bytes for reuse during subset emit planning.
pub struct LayoutScratch {
    staging: Blob,
    gsub_lookup_ptr: Option<*const u8>,
    gsub_lookup_len: usize,
}

impl LayoutScratch {
    pub fn new() -> Self {
        Self {
            staging: Blob::default(),
            gsub_lookup_ptr: None,
            gsub_lookup_len: 0,
        }
    }

    /// Copy a GSUB lookup record into the staging blob and pin its view.
    pub fn pin_gsub_lookup(&mut self, lookup_bytes: &[u8]) -> GlyphResult<()> {
        if lookup_bytes.is_empty() {
            return Ok(());
        }
        let base = self.staging.len();
        self.staging.append(lookup_bytes)?;
        self.staging.cache_subslice(base, lookup_bytes.len())?;
        unsafe {
            let pinned = self.staging.cached_slice();
            self.gsub_lookup_ptr = Some(pinned.as_ptr());
            self.gsub_lookup_len = pinned.len();
        }
        Ok(())
    }

    /// Reserve space for upcoming table emit (may reallocate staging).
    pub fn warmup_emit_tail(&mut self) -> GlyphResult<()> {
        self.staging.append(&[0u8; 8])
    }

    unsafe fn pinned_gsub_view(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.gsub_lookup_ptr
                    .expect("pin_gsub_lookup must precede coverage reads"),
                self.gsub_lookup_len,
            )
        }
    }

    /// Read glyph ID from pinned SingleSubst coverage table (format 1).
    pub fn coverage_glyph_id(&self, index: usize) -> GlyphResult<u16> {
        let view = unsafe { self.pinned_gsub_view() };
        if view.len() < 6 {
            return Err(GlyphError::Unsupported { detail: "gsub lookup too short" });
        }
        let cov_off = u16::from_be_bytes([view[4], view[5]]) as usize;
        let glyph_off = cov_off.saturating_add(4).saturating_add(index.saturating_mul(2));
        if glyph_off + 2 > view.len() {
            return Err(GlyphError::Unsupported { detail: "coverage index out of range" });
        }
        Ok(u16::from_be_bytes([view[glyph_off], view[glyph_off + 1]]))
    }
}

impl Default for LayoutScratch {
    fn default() -> Self {
        Self::new()
    }
}
