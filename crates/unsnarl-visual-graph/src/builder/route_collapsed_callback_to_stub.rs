//! Point a collapsed call+callback's host edges at the BeyondDepth stub
//! that stands in for its omitted body.

use unsnarl_ir::serialized::{SerializedCallbackHost, SerializedCallbackHostKind, SerializedScope};

use super::context::BuilderContext;
use super::node_id::node_id;
use super::push_edge::push_edge;
use super::result_var_for_host::result_var_for_host;
use super::state::BuildState;
use super::write_op_node_for_assignment::write_op_node_for_assignment;

/// Point a collapsed call+callback's host edges at the BeyondDepth stub
/// that stands in for its omitted body. Depth pruning *thins the
/// drawing*: it omits the callback's interior, but the call's
/// relationship must survive. [`super::build_scope::build_scope`]'s
/// collapse branch has just emitted the `((...))` stub (recorded in
/// `collapsed_anchor_by_root`); here we register that stub wherever the
/// non-collapsed path would have registered the call's CallProxy, so the
/// receiver / binding reads land on `((...))` instead of a return-use
/// node minted as if the receiver itself were the result. No CallProxy
/// box is created -- pruning should remove drawing, not add it.
///
/// The dispatch order mirrors the non-collapsed callback handling:
/// reassignment first, then the enclosing statement, then the
/// declarator / return hosts.
pub fn route_collapsed_callback_to_stub(
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    child: &SerializedScope,
    host: Option<&SerializedCallbackHost>,
    parent_scope: &SerializedScope,
) {
    let Some(stub) = state
        .collapsed_anchor_by_root
        .get(child.id.value())
        .cloned()
    else {
        return;
    };
    if let Some(host) = host {
        if matches!(host.kind, SerializedCallbackHostKind::Assignment) {
            if let Some(write_op_node) = write_op_node_for_assignment(host, ctx) {
                push_edge(
                    &mut state.emitted_edges,
                    &mut state.edges,
                    &stub,
                    "read",
                    &write_op_node,
                );
                state.result_proxy_by_write_op.insert(write_op_node, stub);
                return;
            }
        }
    }
    if let Some(statement) = ctx
        .expression_statement_index
        .enclosing(child.block.span.offset.0, child.block.end_span.offset.0)
    {
        state
            .expression_statement_by_offset
            .insert(statement.start_span.offset.0, stub);
        return;
    }
    let Some(host) = host else {
        return;
    };
    match host.kind {
        SerializedCallbackHostKind::VariableDeclarator => {
            if let Some(result_var) = result_var_for_host(host, parent_scope, ctx) {
                push_edge(
                    &mut state.emitted_edges,
                    &mut state.edges,
                    &stub,
                    "read",
                    &node_id(&result_var),
                );
                state.result_proxy_by_var.insert(result_var, stub);
            }
        }
        SerializedCallbackHostKind::Return => {
            let key = format!("{}-{}", host.start_span.offset.0, host.end_span.offset.0);
            state.return_proxy_by_span.insert(key, stub);
        }
        SerializedCallbackHostKind::Assignment => {}
    }
}
