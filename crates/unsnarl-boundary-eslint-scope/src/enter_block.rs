//! Push a `Block` scope for a `BlockStatement` and hoist its
//! declarations.
//!
//! Mirrors `enterBlock` in
//! `ts/src/boundary/eslint-scope/enter-block.ts`. The TS port pulls
//! `node["body"]` and checks `Array.isArray`; the Rust port takes
//! the typed `BlockStatement<'_>` whose `body: Vec<Statement<'_>>` is
//! structurally guaranteed.
//!
//! The `visitor.onScope?.(...)` callback is intentionally not invoked
//! yet — `AnalysisVisitor::on_scope` is added in a follow-up commit
//! (Step 9.7). Once it lands, the `state` / `parent` / `key` / `path`
//! arguments will be threaded through here exactly as TS threads them
//! through `enter_block`.

use oxc_ast::ast::BlockStatement;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::hoisting::hoist_declarations::hoist_declarations;
use crate::state::{push_scope, ScopeBuilderState};

pub(crate) fn enter_block(
    state: &mut ScopeBuilderState,
    block: &BlockStatement<'_>,
    raw: &str,
) -> ScopeId {
    let block_node = AstNode {
        r#type: AstType::BlockStatement,
        span: block.span,
    };
    let scope_id = push_scope(state, ScopeType::Block, block_node);
    hoist_declarations(state, &block.body, scope_id, raw);
    scope_id
}

#[cfg(test)]
#[path = "enter_block_test.rs"]
mod enter_block_test;
