//! Look up (or create on first sight) the `CallProxy` for a callback
//! whose enclosing call's result is reassigned to an existing variable.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedCallbackHost;

use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::context::BuilderContext;
use super::push_edge::push_edge;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;

/// What a reassignment `y = arr.map(cb)` ties its CallProxy to: the
/// reassignment's write-op node, and -- when the reassignment is itself
/// an ExpressionStatement -- that statement's offset, so the proxy can
/// claim it.
pub struct ReassignmentBinding {
    pub write_op_node: String,
    pub stmt_offset: Option<u32>,
}

/// Look up (or create on first sight) the `CallProxy` for a callback
/// whose enclosing call's result is reassigned to an existing variable
/// (`y = arr.map(cb)`) -- the reassignment counterpart of
/// [`super::ensure_host_call_proxy::ensure_host_call_proxy`].
///
/// It keys on the reassignment's *write-op* node rather than the result
/// variable's own node, because that node lives at the variable's
/// declaration site, elsewhere in the graph. The backbone is then the
/// same `input -> arr.map() -> write` shape the declarator case gives at
/// the variable's own node.
pub fn ensure_assignment_call_proxy(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_host: &mut HashMap<u32, SubgraphIdx>,
    host: &SerializedCallbackHost,
    binding: &ReassignmentBinding,
) -> SubgraphIdx {
    let key = host.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_host.get(&key) {
        return idx;
    }
    let id = format!("call_proxy_{key}");
    state
        .result_proxy_by_write_op
        .insert(binding.write_op_node.clone(), id.clone());
    // Bind the call result to the reassignment's write-op node with an
    // edge from the proxy border, the same backbone shape the declarator
    // case draws to the result variable's own node.
    push_edge(
        &mut state.emitted_edges,
        &mut state.edges,
        &id,
        "read",
        &binding.write_op_node,
    );
    // When the reassignment is itself an ExpressionStatement, claim that
    // statement's offset so the callback's statement-contained reads
    // (its return-completed body, owner-less) route into this proxy
    // rather than minting a separate `expr_stmt_<offset>` node -- the
    // same claim `ensure_call_proxy_wrapper` makes for statement-hosted
    // callbacks. A reassignment nested in a declarator (`const r = (y =
    // ...)`) has no enclosing statement, so this stays `None` and the
    // body keeps its own return-use node.
    if let Some(off) = binding.stmt_offset {
        state
            .expression_statement_by_offset
            .entry(off)
            .or_insert_with(|| id.clone());
    }
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
