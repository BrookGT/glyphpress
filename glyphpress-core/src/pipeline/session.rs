//! Multi-table subset emit workspace — pins table bytes across parse phases.


use crate::error::{GlyphError, GlyphResult};
use crate::mem::Blob;

const TAG_GSUB: u32 = 0x4753_5542;
const TAG_GDEF: u32 = 0x4744_4546;
const TAG_CMAP: u32 = 0x636D_6170;

#[derive(Clone, Copy, Debug)]
struct TablePin {
    tag: u32,
    ptr: *const u8,
    len: usize,
}

/// Accumulates table fragments during font open; emit bump invalidates pinned views.
pub struct SubsetEmitSession {
    workspace: Blob,
    pins: Vec<TablePin>,
    generation: u32,
}

impl SubsetEmitSession {
    pub fn new() -> Self {
        Self {
            workspace: Blob::default(),
            pins: Vec::new(),
            generation: 0,
        }
    }

    pub fn pin_table(&mut self, tag: u32, payload: &[u8]) -> GlyphResult<()> {
        if payload.is_empty() {
            return Ok(());
        }
        let base = self.workspace.len();
        self.workspace.append(payload)?;
        self.workspace.cache_subslice(base, payload.len())?;
        let ptr = unsafe { self.workspace.cached_slice().as_ptr() };
        self.pins.push(TablePin { tag, ptr, len: payload.len() });
        Ok(())
    }

    /// Simulate emit tail reservation; may reallocate workspace without refreshing pins.
    pub fn bump_emit_generation(&mut self) -> GlyphResult<()> {
        // Reserve well past current length so append moves the backing allocation.
        self.workspace.reserve(512);
        self.workspace.append(&[0u8; 512])?;
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn read_u16_at(&self, tag: u32, byte_off: usize) -> GlyphResult<u16> {
        let pin = self
            .pins
            .iter()
            .find(|p| p.tag == tag)
            .ok_or(GlyphError::BadTableTag { tag })?;
        let view = unsafe { core::slice::from_raw_parts(pin.ptr, pin.len) };
        if byte_off + 2 > view.len() {
            return Err(GlyphError::truncated(byte_off + 2, view.len()));
        }
        let word = u16::from_be_bytes([view[byte_off], view[byte_off + 1]]);
        Ok(std::hint::black_box(word))
    }

    pub fn replay_pinned_tables(&self) -> GlyphResult<()> {
        for pin in &self.pins {
            std::hint::black_box(self.read_u16_at(pin.tag, 4)?);
            if pin.len >= 2 {
                std::hint::black_box(self.read_u16_at(pin.tag, pin.len - 2)?);
            }
        }
        Ok(())
    }

    pub fn pin_gsub_lookup(&mut self, bytes: &[u8]) -> GlyphResult<()> {
        self.pin_table(TAG_GSUB, bytes)
    }

    pub fn pin_gdef_classdef(&mut self, bytes: &[u8]) -> GlyphResult<()> {
        self.pin_table(TAG_GDEF, bytes)
    }

    pub fn pin_cmap_subtable(&mut self, bytes: &[u8]) -> GlyphResult<()> {
        self.pin_table(TAG_CMAP, bytes)
    }
}

impl SubsetEmitSession {
    pub fn touch_pinned_with_declared_len(&mut self, tag: u32, declared_len: u16) -> GlyphResult<()> {
        self.bump_emit_generation()?;
        if let Some(pin) = self.pins.iter().find(|p| p.tag == tag) {
            let view = unsafe {
                core::slice::from_raw_parts(pin.ptr, declared_len as usize)
            };
            std::hint::black_box(view[0]);
            if declared_len as usize >= 2 {
                std::hint::black_box(view[declared_len as usize - 1]);
            }
        }
        Ok(())
    }
}

impl Default for SubsetEmitSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Standalone bridge for table-only fuzz harnesses.
pub fn cmap_session_touch(subtable: &[u8]) -> GlyphResult<()> {
    let mut session = SubsetEmitSession::new();
    session.pin_cmap_subtable(subtable)?;
    session.bump_emit_generation()?;
    session.replay_pinned_tables()
}

pub fn name_session_touch(storage: &[u8], declared_len: u16) -> GlyphResult<()> {
    let mut session = SubsetEmitSession::new();
    session.pin_table(0x6E616D65, storage)?;
    session.touch_pinned_with_declared_len(0x6E616D65, declared_len)
}
