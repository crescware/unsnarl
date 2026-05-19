//! Push a `Switch` scope (no declarations of its own).
//!
//! Mirrors `enterSwitch` in
//! `ts/src/boundary/eslint-scope/enter-switch.ts`. The Rust port
//! shrinks to a single `push_scope` call; the TS `onScope` callback
//! is dispatched on the Rust side from
//! `ScopeBuildVisitor::fire_on_scope` (in `scope_build_visitor.rs`)
//! once this helper has pushed the scope.

use oxc_ast::ast::SwitchStatement;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::state::{push_scope, ScopeBuilderState};

pub(crate) fn enter_switch(state: &mut ScopeBuilderState, switch: &SwitchStatement<'_>) -> ScopeId {
    let node = AstNode {
        r#type: AstType::SwitchStatement,
        span: switch.span,
    };
    push_scope(state, ScopeType::Switch, node)
}

#[cfg(test)]
#[path = "enter_switch_test.rs"]
mod enter_switch_test;
