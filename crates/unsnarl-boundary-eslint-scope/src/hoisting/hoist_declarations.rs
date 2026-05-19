//! Run the hoist pass over a statement body.
//!
//! Mirrors `hoistDeclarations` in
//! `ts/src/boundary/eslint-scope/hoisting/hoist-declarations.ts`. TS
//! iterates `body` as `readonly unknown[]` and filters with
//! `isNodeLike`; the Rust port takes a typed `&[Statement<'_>]`, so
//! the filter is unnecessary — every element is already a valid
//! statement.

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
