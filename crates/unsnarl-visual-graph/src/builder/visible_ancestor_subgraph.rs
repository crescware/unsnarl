//! Walk `scope.upper` upward until a surviving subgraph is found
//! and return its handle. Collapsed ancestors are skipped (they
//! never recorded a subgraph in `state.subgraph_by_scope`). Returns
//! `None` when the chain reaches the module / global root without
//! crossing a visible subgraph — callers treat that as a drop
//! signal for the corresponding edge.

use unsnarl_ir::serialized::SerializedScope;

use super::arena::SubgraphIdx;
use super::context::BuilderContext;
use super::state::BuildState;

pub fn visible_ancestor_subgraph(
    scope: &SerializedScope,
    ctx: &BuilderContext<'_>,
    state: &BuildState,
) -> Option<SubgraphIdx> {
    let mut parent_id = scope.upper.as_ref().map(|id| id.value());
    while let Some(id) = parent_id {
        if let Some(&idx) = state.subgraph_by_scope.get(id) {
            return Some(idx);
        }
        let parent = ctx.scope_map.get(id).copied()?;
        parent_id = parent.upper.as_ref().map(|i| i.value());
    }
    None
}
