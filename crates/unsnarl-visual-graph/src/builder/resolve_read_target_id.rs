//! Mirrors `ts/src/visual-graph/builder/resolve-read-target-id.ts`.
//!
//! Pick the destination node id for a read edge:
//!
//! - When the reference participates in an ExpressionStatement
//!   container, the previously-emitted synthetic statement node
//!   (`ensure_expression_statement_node`) is the target.
//! - When there is no enclosing function (the reference reads at
//!   the module top level), the read lands on
//!   `module_root`.
//! - When the reference is part of a `return` / `throw` completion
//!   inside a function, the matching `ensure_*_use_node` produces
//!   the target (with `module_root` as a fallback if no host
//!   subgraph was found because the surrounding scope collapsed).
//! - A `normal` completion inside a function (no expression
//!   statement, no owner-target) lands on `module_root` so the
//!   edge still has a destination.

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
    r: &SerializedReference,
) -> String {
    if let Some(id) = expr_stmt_id {
        return id.to_string();
    }
    let Some(fn_var) = enclosing_fn_var_id else {
        return MODULE_ROOT_ID.to_string();
    };
    match &r.completion {
        SerializedCompletion::Return { .. } => ensure_return_use_node(arena, state, ctx, fn_var, r)
            .unwrap_or_else(|| MODULE_ROOT_ID.to_string()),
        SerializedCompletion::Throw { .. } => ensure_throw_use_node(arena, state, ctx, fn_var, r)
            .unwrap_or_else(|| MODULE_ROOT_ID.to_string()),
        SerializedCompletion::Normal => MODULE_ROOT_ID.to_string(),
    }
}

#[cfg(test)]
#[path = "resolve_read_target_id_test.rs"]
mod resolve_read_target_id_test;
