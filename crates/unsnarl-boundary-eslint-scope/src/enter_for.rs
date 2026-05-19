//! Push a `For` scope and declare the head bindings.
//!
//! Mirrors `enterFor` in `ts/src/boundary/eslint-scope/enter-for.ts`.
//! The TS port handles all three for-statement variants
//! (`for` / `for-in` / `for-of`) through one `NodeLike` entry; the
//! Rust port exposes three variant-specific helpers that share the
//! `push_scope` + `declare_for_left` shape, so each caller passes the
//! statically-typed head node it already has.

use oxc_ast::ast::{
    ForInStatement, ForOfStatement, ForStatement, ForStatementInit, ForStatementLeft,
};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::declare_for_left::declare_for_left;
use crate::state::{push_scope, ScopeBuilderState};

pub(crate) fn enter_for_statement(
    state: &mut ScopeBuilderState,
    stmt: &ForStatement<'_>,
    raw: &str,
) -> ScopeId {
    let node = AstNode {
        r#type: AstType::ForStatement,
        span: stmt.span,
    };
    let scope_id = push_scope(state, ScopeType::For, node);
    if let Some(ForStatementInit::VariableDeclaration(var_decl)) = stmt.init.as_ref() {
        declare_for_left(state, scope_id, var_decl, raw);
    }
    scope_id
}

pub(crate) fn enter_for_in_statement(
    state: &mut ScopeBuilderState,
    stmt: &ForInStatement<'_>,
    raw: &str,
) -> ScopeId {
    let node = AstNode {
        r#type: AstType::ForInStatement,
        span: stmt.span,
    };
    let scope_id = push_scope(state, ScopeType::For, node);
    if let ForStatementLeft::VariableDeclaration(var_decl) = &stmt.left {
        declare_for_left(state, scope_id, var_decl, raw);
    }
    scope_id
}

pub(crate) fn enter_for_of_statement(
    state: &mut ScopeBuilderState,
    stmt: &ForOfStatement<'_>,
    raw: &str,
) -> ScopeId {
    let node = AstNode {
        r#type: AstType::ForOfStatement,
        span: stmt.span,
    };
    let scope_id = push_scope(state, ScopeType::For, node);
    if let ForStatementLeft::VariableDeclaration(var_decl) = &stmt.left {
        declare_for_left(state, scope_id, var_decl, raw);
    }
    scope_id
}
