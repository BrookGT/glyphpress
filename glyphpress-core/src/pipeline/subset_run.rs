//! Run subset pipeline: plan, closure, emit.


use crate::cmap::encode::{build_format4_subtable, CmapMapping};
use crate::emit::builder::SfntBuilder;
use crate::error::GlyphResult;
use crate::pipeline::load::LoadedFont;
use crate::subset::plan::SubsetPlan;
use crate::subset::remap::RemapTable;
use crate::tables::glyf::GlyfTable;

#[derive(Clone, Debug, Default)]
pub struct SubsetOptions {
    pub retain_gids: Vec<u16>,
    pub codepoints: Vec<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct SubsetReport {
    pub input_glyphs: u16,
    pub output_glyphs: u16,
    pub bytes_written: usize,
}

pub fn run_subset(font: &LoadedFont<'_>, options: &SubsetOptions) -> GlyphResult<(Vec<u8>, SubsetReport)> {
    let mut plan = SubsetPlan::new();
    for &gid in &options.retain_gids {
        plan.add_glyph(gid);
    }
    for &cp in &options.codepoints {
        plan.add_codepoint(cp);
        if let Some(gid) = font.cmap.map_codepoint(cp)? {
            plan.add_glyph(gid);
        }
    }
    plan.ensure_not_empty()?;
    plan.close_over_composites(|gid| {
        let (s, e) = font.loca.glyph_range(gid)?;
        match font.glyf.parse_glyph(s, e, gid)? {
            crate::tables::glyf::GlyphOutline::Composite { components, .. } => {
                Ok(components.iter().map(|c| c.glyph_index).collect())
            }
            _ => Ok(Vec::new()),
        }
    })?;
    let glyphs = plan.glyph_list();
    let remap = RemapTable::from_glyph_list(&glyphs);
    let mut mappings = Vec::new();
    for &cp in &options.codepoints {
        if let Some(old) = font.cmap.map_codepoint(cp)? {
            if let Some(new_gid) = remap.remap(old) {
                mappings.push(CmapMapping { codepoint: cp, glyph_id: new_gid });
            }
        }
    }
    let cmap_sub = build_format4_subtable(&mappings)?;
    let mut builder = SfntBuilder::new();
    builder.add_table(0x636D6170, cmap_sub);
    let bytes = builder.build()?;
    let report = SubsetReport {
        input_glyphs: font.maxp.num_glyphs,
        output_glyphs: glyphs.len() as u16,
        bytes_written: bytes.len(),
    };
    Ok((bytes, report))
}

/* volume */

impl SubsetOptions {
    pub fn is_empty(&self) -> bool {
        self.retain_gids.is_empty() && self.codepoints.is_empty()
    }
}
