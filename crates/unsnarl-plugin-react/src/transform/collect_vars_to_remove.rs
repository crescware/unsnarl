//! Decide which hook-import variables to drop.
//!
//! A hook import (`useCallback` / `useMemo`) is dropped iff every
//! reference to it was inside a wrapped binding's body, leaving no
//! surviving references after `collect_refs_to_remove` runs.

use std::collections::{HashMap, HashSet};

use super::hook_kind::HookKind;

pub fn collect_vars_to_remove(
    hook_imports: &HashMap<String, HookKind>,
    refs_retained_by_var: &HashMap<String, u32>,
) -> HashSet<String> {
    let mut out = HashSet::new();
    for id in hook_imports.keys() {
        if !refs_retained_by_var.contains_key(id) {
            out.insert(id.clone());
        }
    }
    out
}
