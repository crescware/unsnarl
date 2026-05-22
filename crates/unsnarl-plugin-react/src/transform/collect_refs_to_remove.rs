//! Collect references owned by a wrapped variable that should drop
//! out of the IR.
//!
//! Iterates every non-init reference; if any of its `owners` is in
//! [`wrapped_var_ids`], the reference is queued for removal. These
//! are the `() => ...` body and the dependency-array references.

use std::collections::HashSet;

use unsnarl_ir::serialized::SerializedIR;

pub fn collect_refs_to_remove(
    ir: &SerializedIR,
    wrapped_var_ids: &HashSet<String>,
) -> HashSet<String> {
    let mut out = HashSet::new();
    for r in &ir.references {
        if r.init {
            continue;
        }
        for o in &r.owners {
            if wrapped_var_ids.contains(o.value()) {
                out.insert(r.id.value().to_string());
                break;
            }
        }
    }
    out
}
