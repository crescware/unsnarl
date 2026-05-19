//! Push a `Block` scope for a `SwitchCase` and hoist its
//! consequent.
//!
//! Mirrors `enterSwitchCase` in
//! `ts/src/boundary/eslint-scope/enter-switch-case.ts`. Note that the
//! TS port uses scope type `"block"` (not `"switch"`); the Rust port
//! does the same. The case's consequent is `Vec<Statement<'_>>` in
//! oxc, structurally guaranteed, so the TS `Array.isArray` check
//! collapses.

use oxc_ast::ast::SwitchCase;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::hoisting::hoist_declarations::hoist_declarations;
use crate::state::{push_scope, ScopeBuilderState};

pub(crate) fn enter_switch_case(
    state: &mut ScopeBuilderState,
    case: &SwitchCase<'_>,
    raw: &str,
) -> ScopeId {
    let node = AstNode {
        r#type: AstType::SwitchCase,
        span: case.span,
    };
    let scope_id = push_scope(state, ScopeType::Block, node);
    hoist_declarations(state, &case.consequent, scope_id, raw);
    scope_id
}

#[cfg(test)]
#[path = "enter_switch_case_test.rs"]
mod enter_switch_case_test;
