//! The proxy a callback should be built into, given its host proxy,
//! expanding a method chain into a sibling backbone of `CallProxy`es.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedHeadExpression;

use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::call_node_extent::call_node_extent;
use super::context::BuilderContext;
use super::push_edge::push_edge;
use super::receiver_call_chain::receiver_call_chain;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;

/// The host call a chain of callbacks routes against: the outermost
/// proxy already materialised for it and the head expression whose
/// receiver chain drives the per-call split.
pub struct ChainHost<'a> {
    pub proxy: SubgraphIdx,
    pub head: &'a SerializedHeadExpression,
}

/// The proxy a callback should be built into, given its host proxy
/// (the outermost, host-bound call), and -- as a side effect -- the
/// sibling backbone of `CallProxy`es a method chain expands into.
///
/// For a method chain `arr.map(f).filter(g)` the callbacks `f` and `g`
/// belong to *different* calls. Each call that hosts a callback gets its
/// own `CallProxy`; the per-call proxies are laid out as **siblings** of
/// the host (in `container`) and wired together by `read` edges that
/// mirror the receiver chain -- `arr.map()` feeds `arr.map().filter()` --
/// instead of being nested. That is the same dataflow-by-edge rule the
/// result binding uses, so a chain reads as one straight backbone
/// (`arr -> arr.map() -> arr.map().filter() -> ...`) rather than a stack
/// of boxes. `f` is routed into the innermost call's proxy (`arr.map()`),
/// `g` into the host proxy (`arr.map().filter()`). A single (non-chained)
/// call has only the outermost node, so its callback stays in the host
/// proxy and no extra proxy or edge is created.
///
/// The receiver edge points from the inner proxy to its outer neighbour
/// (the call whose receiver it is). The chain's input -- the innermost
/// receiver, e.g. `arr` -- is routed to the innermost proxy by the caller
/// via the host's result routing (see
/// [`super::innermost_chain_proxy_id::innermost_chain_proxy_id`]).
pub fn callback_chain_target(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    host: ChainHost<'_>,
    block: (u32, u32),
    nested_call_proxy: &mut HashMap<(u32, u32), SubgraphIdx>,
) -> SubgraphIdx {
    let (block_start, block_end) = block;
    let mut parent = host.proxy;
    let mut target = host.proxy;
    for (depth, &call_node) in receiver_call_chain(host.head).iter().enumerate() {
        let Some((start_offset, end_offset, start_line, end_line)) = call_node_extent(call_node)
        else {
            break;
        };
        // Calls are listed outermost first; once one no longer contains
        // the callback block, no deeper (narrower) call can either.
        if !(start_offset <= block_start && block_end <= end_offset) {
            break;
        }
        if depth == 0 {
            // The outermost call is the host's bound expression itself,
            // already materialised as the host proxy.
            nested_call_proxy
                .entry((start_offset, end_offset))
                .or_insert(host.proxy);
        } else if let Some(&idx) = nested_call_proxy.get(&(start_offset, end_offset)) {
            target = idx;
        } else {
            // One inner call of the chain gets its own CallProxy, a
            // sibling of the host (appended to `container`, not nested),
            // labelled with the call head itself (`arr.map()`), distinct
            // from the host proxy's whole-chain label
            // (`arr.map().filter()`). Keyed by call span so every callback
            // of that call shares it. A `read` edge from this proxy to its
            // receiver's proxy (`parent`, the next call out) carries the
            // chain's dataflow by edge -- the same way the result binding
            // is drawn -- so the chain renders as a straight backbone.
            let id = format!("call_proxy_{start_offset}_{end_offset}");
            let name = render_head_expression(call_node, &ctx.source_index);
            let mut sg = OwnedVisualSubgraph::call_proxy(
                id.clone(),
                start_line,
                name,
                Vec::new(),
                Direction::RL,
            );
            sg.end_line = end_line;
            let idx = arena.push_subgraph(sg.into());
            arena.append_child(container, ElementHandle::Subgraph(idx));
            let parent_id = arena.subgraph(parent).descriptor.id().to_string();
            push_edge(
                &mut state.emitted_edges,
                &mut state.edges,
                &id,
                "read",
                &parent_id,
            );
            nested_call_proxy.insert((start_offset, end_offset), idx);
            target = idx;
        }
        parent = target;
    }
    target
}
