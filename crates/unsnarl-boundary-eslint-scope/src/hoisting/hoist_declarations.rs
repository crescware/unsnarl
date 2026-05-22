//! Run the hoist pass over a statement body.
//!
//! Takes a typed `&[Statement<'_>]`; every element is already a
//! valid statement, so no node-like filter is needed.

use oxc_ast::ast::Statement;

use unsnarl_ir::ids::ScopeId;

use crate::hoisting::visit::visit_statement;
use crate::state::ScopeBuilderState;

pub(crate) fn hoist_declarations(
    state: &mut ScopeBuilderState,
    body: &[Statement<'_>],
    scope: ScopeId,
    raw: &str,
) {
    for stmt in body {
        visit_statement(state, scope, stmt, raw);
    }
}

#[cfg(test)]
#[path = "hoist_declarations_test.rs"]
mod hoist_declarations_test;
