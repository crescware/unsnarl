//! Mirrors `ts/src/visual-graph/builder/find-host-subgraph.ts`.
//!
//! Walk `ref.from` upward to the closest scope that materialised a
//! subgraph during the top-down build and return its handle. When
//! no scope-direct hit is found, fall back to the function
//! subgraph keyed by `enclosing_fn_var_id`. Returns `None` when
//! neither lookup yields a surviving subgraph (callers treat that
//! as "no host", which turns ensure-return/throw-use into a no-op).

use std::collections::HashMap;

use unsnarl_ir::serialized::{SerializedReference, SerializedScope};

use super::arena::SubgraphIdx;
use super::state::BuildState;

pub fn find_host_subgraph(
    r: &SerializedReference,
    enclosing_fn_var_id: Option<&str>,
    scope_map: &HashMap<&str, &SerializedScope>,
    state: &BuildState,
) -> Option<SubgraphIdx> {
    let mut cur = scope_map.get(r.from.value()).copied();
    while let Some(scope) = cur {
        if let Some(&idx) = state.subgraph_by_scope.get(scope.id.value()) {
            return Some(idx);
        }
        let Some(upper) = scope.upper.as_ref() else {
            break;
        };
        cur = scope_map.get(upper.value()).copied();
    }
    let fn_var = enclosing_fn_var_id?;
    state.function_subgraph_by_fn.get(fn_var).copied()
}
