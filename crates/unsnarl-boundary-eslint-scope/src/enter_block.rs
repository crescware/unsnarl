//! Push a `Block` scope for a `BlockStatement` and hoist its
//! declarations.
//!
//! Mirrors `enterBlock` in
//! `ts/src/boundary/eslint-scope/enter-block.ts`. The TS port pulls
//! `node["body"]` and checks `Array.isArray`; the Rust port takes
//! the typed `BlockStatement<'_>` whose `body: Vec<Statement<'_>>` is
//! structurally guaranteed.
//!
//! The TS `visitor.onScope?.(...)` callback is dispatched on the
//! Rust side from `ScopeBuildVisitor::fire_on_scope` (in
//! `scope_build_visitor.rs`) once the helper has pushed the scope,
//! so this module is responsible only for pushing and hoisting.

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
