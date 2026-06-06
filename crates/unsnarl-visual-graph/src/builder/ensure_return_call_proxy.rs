//! Look up (or create on first sight) the `CallProxy` for a callback
//! returned from a function (`return arr.map(cb)`).

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedCallbackHost;

use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::context::BuilderContext;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;

/// Look up (or create on first sight) the `CallProxy` for a callback
/// returned from a function (`return arr.map(cb)`).
///
/// The return completion's inputs (the call's receiver / callee / args)
/// route here via `return_proxy_by_span` instead of a return-use node,
/// so the returned call's callback is not stranded as an island.
pub fn ensure_return_call_proxy(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_host: &mut HashMap<u32, SubgraphIdx>,
    host: &SerializedCallbackHost,
) -> SubgraphIdx {
    let key = host.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_host.get(&key) {
        return idx;
    }
    let id = format!("call_proxy_{key}");
    let container_key = format!("{}-{}", host.start_span.offset.0, host.end_span.offset.0);
    state.return_proxy_by_span.insert(container_key, id.clone());
    let name = render_head_expression(&host.head, &ctx.source_index);
    let start_line = host.start_span.line.0;
    let end_line = if host.end_span.line.0 != start_line {
        Some(host.end_span.line.0)
    } else {
        None
    };
    let mut sg = OwnedVisualSubgraph::call_proxy(id, start_line, name, Vec::new(), Direction::RL);
    sg.end_line = end_line;
    let idx = arena.push_subgraph(sg.into());
    arena.append_child(container, ElementHandle::Subgraph(idx));
    call_proxy_by_host.insert(key, idx);
    idx
}
