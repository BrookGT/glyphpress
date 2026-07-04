//! Subset plan with glyph closure over composites.


use crate::error::{GlyphError, GlyphResult};
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Debug, Default)]
pub struct SubsetPlan {
    pub glyphs: BTreeSet<u16>,
    pub codepoints: BTreeSet<u32>,
}

impl SubsetPlan {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_glyph(&mut self, gid: u16) {
        self.glyphs.insert(gid);
    }

    pub fn add_codepoint(&mut self, cp: u32) {
        self.codepoints.insert(cp);
    }

    pub fn ensure_not_empty(&self) -> GlyphResult<()> {
        if self.glyphs.is_empty() && self.codepoints.is_empty() {
            return Err(GlyphError::SubsetEmpty);
        }
        Ok(())
    }

    pub fn glyph_list(&self) -> Vec<u16> {
        self.glyphs.iter().copied().collect()
    }

    pub fn close_over_composites<F>(&mut self, mut components_of: F) -> GlyphResult<()>
    where
        F: FnMut(u16) -> GlyphResult<Vec<u16>>,
    {
        let mut queue: VecDeque<u16> = self.glyphs.iter().copied().collect();
        while let Some(gid) = queue.pop_front() {
            for comp in components_of(gid)? {
                if self.glyphs.insert(comp) {
                    queue.push_back(comp);
                }
            }
        }
        Ok(())
    }
}
