//! Remove glyphs not referenced by subset plan.


use crate::subset::plan::SubsetPlan;

pub fn prune_unused_glyphs(all_gids: &[u16], plan: &SubsetPlan) -> Vec<u16> {
    all_gids.iter().copied().filter(|g| plan.glyphs.contains(g)).collect()
}

pub fn prune_codepoints(plan: &mut SubsetPlan, used: &[u32]) {
    plan.codepoints.retain(|cp| used.contains(cp));
}
