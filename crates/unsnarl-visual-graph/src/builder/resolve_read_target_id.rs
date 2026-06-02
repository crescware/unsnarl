//! Pick the destination node id for a read edge:
//!
//! - When the reference participates in an ExpressionStatement
//!   container, the previously-emitted synthetic statement node
//!   (`ensure_expression_statement_node`) is the target.
//! - When the reference is part of a `return` / `throw` completion,
//!   the matching `ensure_*_use_node` produces the target. This
//!   holds whether or not the enclosing function is owned by a
//!   variable: an owner-var-less callback (`const Panel =
//!   wrap(arrow)`) keys its wrapping subgraph by the host scope id
//!   (`enclosing_fn_scope_id`) instead. `module_root` remains the
//!   fallback when no host subgraph was found (a true module-top
//!   `return` / `throw`, or a collapsed surrounding scope).
//! - A `normal` completion (no expression statement) lands on
//!   `module_root` so the edge still has a destination. This is the
//!   sole remaining unconditional `module_root` path and covers
//!   module-top-level reads.

use unsnarl_ir::serialized::{SerializedCompletion, SerializedReference};

use super::arena::BuildArena;
use super::context::BuilderContext;
use super::ensure_return_use_node::ensure_return_use_node;
use super::ensure_throw_use_node::ensure_throw_use_node;
use super::module_root_id::MODULE_ROOT_ID;
use super::state::BuildState;

pub fn resolve_read_target_id(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    expr_stmt_id: Option<&str>,
    enclosing_fn_var_id: Option<&str>,
    enclosing_fn_scope_id: Option<&str>,
    r: &SerializedReference,
) -> String {
    // A returned value resolves to its return-use node (or the
    // returned-call proxy) *before* the enclosing statement's synthetic
    // node, even when the read also sits inside an ExpressionStatement.
    // An arrow's implicit return inside a statement-level method chain
    // (`arr.map((v) => v).filter(...);`, `y = arr.map((v) => v)...`)
    // belongs to its own call's CallProxy, not the whole statement; the
    // statement claim would otherwise pull every chain stage's return
    // onto the outermost proxy. A declaration-bound chain
    // (`const xs = arr.map(...)`) carries no statement claim and already
    // reaches this same return-use path.
    if let SerializedCompletion::Return {
        start_span,
        end_span,
    } = &r.completion
    {
        // A `return <call>(cb)` is wrapped in a CallProxy that contains
        // the callback; the returned call's inputs land on that proxy's
        // border rather than a return-use node.
        let container_key = format!("{}-{}", start_span.offset.0, end_span.offset.0);
        if let Some(proxy) = state.return_proxy_by_span.get(&container_key) {
            return proxy.clone();
        }
        if let Some(id) = ensure_return_use_node(
            arena,
            state,
            ctx,
            enclosing_fn_var_id,
            enclosing_fn_scope_id,
            r,
        ) {
            return id;
        }
    }
    if let Some(id) = expr_stmt_id {
        return id.to_string();
    }
    match &r.completion {
        SerializedCompletion::Return { .. } => MODULE_ROOT_ID.to_string(),
        SerializedCompletion::Throw { .. } => ensure_throw_use_node(
            arena,
            state,
            ctx,
            enclosing_fn_var_id,
            enclosing_fn_scope_id,
            r,
        )
        .unwrap_or_else(|| MODULE_ROOT_ID.to_string()),
        SerializedCompletion::Normal => MODULE_ROOT_ID.to_string(),
    }
}

#[cfg(test)]
#[path = "resolve_read_target_id_test.rs"]
mod resolve_read_target_id_test;
