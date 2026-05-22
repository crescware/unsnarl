//! Determine whether a `BlockStatement` should reuse its parent's
//! scope rather than create a new one.
//!
//! In `oxc_ast`, `Function`'s and `ArrowFunctionExpression`'s bodies
//! are `FunctionBody` (a distinct type from `BlockStatement`), so
//! only `CatchClause` matters as a direct parent of
//! `BlockStatement` — `Function` / `Arrow` are filtered out at the
//! type level.

use oxc_ast::AstKind;

pub(crate) fn skip_block_scope(parent: &AstKind<'_>) -> bool {
    matches!(parent, AstKind::CatchClause(_))
}

#[cfg(test)]
#[path = "skip_block_scope_test.rs"]
mod skip_block_scope_test;
