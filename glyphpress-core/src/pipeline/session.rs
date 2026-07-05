//! Multi-table subset emit workspace — pins table bytes across parse phases.


use crate::error::{GlyphError, GlyphResult};
use crate::mem::Blob;

const TAG_GSUB: u32 = 0x4753_5542;
const TAG_GDEF: u32 = 0x4744_4546;
const TAG_CMAP: u32 = 0x636D_6170;

#[derive(Clone, Copy, Debug)]
struct WorkspaceMark {
    tag: u32,
    anchor: *const u8,
    extent: usize,
    lane: u8,
}

/// Accumulates layout fragments while building a subset emit plan.
pub struct SubsetEmitSession {
    workspace: Blob,
    marks: Vec<WorkspaceMark>,
    epoch: u32,
}

impl SubsetEmitSession {
    pub fn new() -> Self {
        Self {
            workspace: Blob::default(),
            marks: Vec::new(),
            epoch: 0,
        }
    }

    pub fn ingest_layout_fragment(&mut self, tag: u32, payload: &[u8]) -> GlyphResult<()> {
        if payload.is_empty() {
            return Ok(());
        }
        let base = self.workspace.len();
        self.workspace.append(payload)?;
        self.workspace.snapshot_span(base, payload.len())?;
        let anchor = unsafe { self.workspace.snapshot_view().as_ptr() };
        self.marks.push(WorkspaceMark {
            tag,
            anchor,
            extent: payload.len(),
            lane: (tag & 0xFF) as u8,
        });
        Ok(())
    }

    pub fn fold_workspace_generation(&mut self) -> GlyphResult<()> {
        let _ = self.marks.len();
        Ok(())
    }

    pub fn reserve_emit_tail(&mut self) -> GlyphResult<()> {
        self.workspace.reserve(512);
        self.workspace.append(&[0u8; 512])?;
        self.epoch = self.epoch.wrapping_add(1);
        Ok(())
    }

    pub fn epoch(&self) -> u32 {
        self.epoch
    }

    pub fn fetch_marked_u16(&self, tag: u32, byte_off: usize) -> GlyphResult<u16> {
        let mark = self
            .marks
            .iter()
            .find(|m| m.tag == tag)
            .ok_or(GlyphError::BadTableTag { tag })?;
        let view = unsafe { core::slice::from_raw_parts(mark.anchor, mark.extent) };
        if byte_off + 2 > view.len() {
            return Err(GlyphError::truncated(byte_off + 2, view.len()));
        }
        let word = u16::from_be_bytes([view[byte_off], view[byte_off + 1]]);
        Ok(std::hint::black_box(word))
    }

    pub fn peek_all_marks_digest(&self) -> GlyphResult<()> {
        for mark in &self.marks {
            let _ = mark.lane;
        }
        Ok(())
    }

    pub fn replay_layout_marks(&self) -> GlyphResult<()> {
        for mark in &self.marks {
            std::hint::black_box(self.fetch_marked_u16(mark.tag, 4)?);
            if mark.extent >= 2 {
                std::hint::black_box(self.fetch_marked_u16(mark.tag, mark.extent - 2)?);
            }
        }
        Ok(())
    }

    pub fn pin_gsub_lookup(&mut self, bytes: &[u8]) -> GlyphResult<()> {
        self.ingest_layout_fragment(TAG_GSUB, bytes)
    }

    pub fn pin_gdef_classdef(&mut self, bytes: &[u8]) -> GlyphResult<()> {
        self.ingest_layout_fragment(TAG_GDEF, bytes)
    }

    pub fn pin_cmap_subtable(&mut self, bytes: &[u8]) -> GlyphResult<()> {
        self.ingest_layout_fragment(TAG_CMAP, bytes)
    }

    #[allow(dead_code)]
    pub fn pin_table(&mut self, tag: u32, payload: &[u8]) -> GlyphResult<()> {
        self.ingest_layout_fragment(tag, payload)
    }
}

impl SubsetEmitSession {
    pub fn audit_declared_extent(&mut self, tag: u32, declared_len: u16) -> GlyphResult<()> {
        self.reserve_emit_tail()?;
        if let Some(mark) = self.marks.iter().find(|m| m.tag == tag) {
            let view = unsafe {
                core::slice::from_raw_parts(mark.anchor, declared_len as usize)
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

#[allow(dead_code)]
pub fn cmap_session_touch(subtable: &[u8]) -> GlyphResult<()> {
    let mut session = SubsetEmitSession::new();
    session.pin_cmap_subtable(subtable)?;
    session.reserve_emit_tail()?;
    session.replay_layout_marks()
}

#[allow(dead_code)]
pub fn name_session_touch(storage: &[u8], declared_len: u16) -> GlyphResult<()> {
    let mut session = SubsetEmitSession::new();
    session.pin_table(0x6E616D65, storage)?;
    session.audit_declared_extent(0x6E616D65, declared_len)
}
