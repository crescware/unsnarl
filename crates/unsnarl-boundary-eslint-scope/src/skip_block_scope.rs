//! Determine whether a `BlockStatement` should reuse its parent's
//! scope rather than create a new one.
//!
//! Mirrors `skipBlockScope` in
//! `ts/src/boundary/eslint-scope/skip-block-scope.ts`. The TS list
//! covers `Function` / `FunctionExpression` / `ArrowFunctionExpression`
//! / `CatchClause` because in the TS AST those nodes have a
//! `BlockStatement` body directly. In `oxc_ast`, `Function`'s and
//! `ArrowFunctionExpression`'s bodies are `FunctionBody` (a distinct
//! type, not `BlockStatement`), so only `CatchClause` matters as a
//! direct parent of `BlockStatement` — `Function` / `Arrow` are
//! filtered out at the type level.

use oxc_ast::AstKind;

pub(crate) fn skip_block_scope(parent: &AstKind<'_>) -> bool {
    matches!(parent, AstKind::CatchClause(_))
}
