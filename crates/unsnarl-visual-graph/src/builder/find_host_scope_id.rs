//! Walk `ref.from` upward to the closest scope that materialised a
//! subgraph during the top-down build and return *that scope's id*
//! (borrowed from `scope_map`). This mirrors the primary walk of
//! [`super::find_host_subgraph::find_host_subgraph`] exactly, but
//! yields the scope id rather than the [`SubgraphIdx`] handle.
//!
//! It exists for the owner-var-less return/throw path: when a
//! function expression is passed as a call argument and bound to a
//! variable (`const Panel = wrap(arrow)`), the callback's scope has
//! no owning variable, so there is no owner-var string to key the
//! wrapping Return / Throw subgraph by. The scope id of the host
//! subgraph -- the very container the Return / Throw subgraph is
//! appended into -- is used as the key instead, keeping key and host
//! in lockstep. The owner-var fallback that `find_host_subgraph`
//! performs is intentionally omitted: callers reach for this only
//! when there is no owner var.

use std::collections::HashMap;

use unsnarl_ir::serialized::{SerializedReference, SerializedScope};

use super::state::BuildState;

pub fn find_host_scope_id<'a>(
    r: &SerializedReference,
    scope_map: &'a HashMap<&'a str, &'a SerializedScope>,
    state: &BuildState,
) -> Option<&'a str> {
    let mut cur = scope_map.get(r.from.value()).copied();
    while let Some(scope) = cur {
        if state.subgraph_by_scope.contains_key(scope.id.value()) {
            // The scope ids stored in `scope_map`'s values share the
            // IR arena's lifetime with the map's keys, so handing back
            // the key borrow ties the result to `'a` without a fresh
            // allocation (same pattern as
            // `find_enclosing_subgraph_scope_borrowed`).
            return scope_map.get_key_value(scope.id.value()).map(|(k, _)| *k);
        }
        let upper = scope.upper.as_ref()?;
        cur = scope_map.get(upper.value()).copied();
    }
    None
}

#[cfg(test)]
#[path = "find_host_scope_id_test.rs"]
mod find_host_scope_id_test;
