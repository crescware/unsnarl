//! Per-variable retained-reference counter.
//!
//! Mirrors `countRetainedRefsByResolved` in
//! `ts/src/plugins/unsnarl-plugin-react/index.ts`. Walks every
//! reference that is NOT in [`refs_to_remove`] and bumps the count
//! for its `resolved` variable id. The result is used to decide
//! whether a hook import has any live use sites remaining; if not,
//! the import binding itself can be dropped.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::serialized::SerializedIR;

pub fn count_retained_refs_by_resolved(
    ir: &SerializedIR,
    refs_to_remove: &HashSet<String>,
) -> HashMap<String, u32> {
    let mut out: HashMap<String, u32> = HashMap::new();
    for r in &ir.references {
        if refs_to_remove.contains(r.id.value()) {
            continue;
        }
        let Some(resolved) = r.resolved.as_ref() else {
            continue;
        };
        *out.entry(resolved.value().to_string()).or_insert(0) += 1;
    }
    out
}
