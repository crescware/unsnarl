//! Mirrors `ts/src/visual-graph/builder/previous-fallthrough-case.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::branch_container_key::branch_container_key;

pub fn previous_fallthrough_case<'a>(
    case_scope: &SerializedScope,
    sorted_cases_by_container: &'a HashMap<String, Vec<&'a SerializedScope>>,
) -> Option<&'a SerializedScope> {
    let ckey = branch_container_key(case_scope)?;
    let cases = sorted_cases_by_container.get(&ckey)?;
    // Index by identity: TS uses object reference equality; in Rust
    // we compare on the scope id (which `FlatSerializer` guarantees
    // unique per scope).
    let idx = cases
        .iter()
        .position(|s| s.id.value() == case_scope.id.value())?;
    if idx == 0 {
        return None;
    }
    let prev = cases[idx - 1];
    if prev.falls_through {
        Some(prev)
    } else {
        None
    }
}
