//! Look up (or create on first sight) the result-bound `CallProxy` for
//! a callback whose enclosing call's result is bound to a variable.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedCallbackHost;

use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::context::BuilderContext;
use super::node_id::node_id;
use super::push_edge::push_edge;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;

/// Look up (or create on first sight) the `CallProxy` for a callback
/// whose enclosing call's result is bound to `result_var`
/// (`const xs = arr.map(cb)`, including a call nested in arguments
/// `const x = foo(data.map(cb))`) -- the non-statement counterpart of
/// [`super::ensure_call_proxy_wrapper::ensure_call_proxy_wrapper`].
///
/// The proxy is labelled with the host head, so it stands for the whole
/// bound call (`foo(...)`), not just the inner callback's call. The
/// result feeds the bound variable through the same `read` a plain
/// `const b = a` produces, which keeps the variable an ordinary sibling
/// node and the dataflow backbone (`arr -> arr.map() -> a -> ...`)
/// unbroken instead of nesting boxes. Every callback within the bound
/// expression shares one proxy, keyed by the host's start offset.
pub fn ensure_host_call_proxy(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_host: &mut HashMap<u32, SubgraphIdx>,
    host: &SerializedCallbackHost,
    result_var: &str,
) -> SubgraphIdx {
    let key = host.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_host.get(&key) {
        return idx;
    }
    let id = format!("call_proxy_{key}");
    // Inputs of this call own the result variable; their init-time
    // owner edges retarget from the result-variable node to this proxy
    // (see `result_proxy_by_var`).
    state
        .result_proxy_by_var
        .insert(result_var.to_string(), id.clone());
    // Bind the call result to the result variable with an edge from the
    // proxy border, completing the backbone `<input> -> proxy -> <var>`.
    push_edge(
        &mut state.emitted_edges,
        &mut state.edges,
        &id,
        "read",
        &node_id(result_var),
    );
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
