//! Old to new glyph index remapping.


use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct RemapTable {
    old_to_new: BTreeMap<u16, u16>,
}

impl RemapTable {
    pub fn from_glyph_list(glyphs: &[u16]) -> Self {
        let mut old_to_new = BTreeMap::new();
        for (new_idx, &old) in glyphs.iter().enumerate() {
            old_to_new.insert(old, new_idx as u16);
        }
        Self { old_to_new }
    }

    pub fn remap(&self, old: u16) -> Option<u16> {
        self.old_to_new.get(&old).copied()
    }

    pub fn len(&self) -> usize {
        self.old_to_new.len()
    }

    pub fn is_empty(&self) -> bool {
        self.old_to_new.is_empty()
    }
}
