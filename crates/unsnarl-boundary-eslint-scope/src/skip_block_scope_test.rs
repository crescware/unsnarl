//! Sibling tests for `skip_block_scope.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/skip-block-scope.test.ts`.
//!
//! In oxc the only parent type that triggers the skip is
//! `CatchClause` (TS's `Function` / `ArrowFunctionExpression` are
//! ruled out structurally because their bodies are `FunctionBody`,
//! not `BlockStatement`).

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, find_first_descendant_scope};

#[test]
fn catch_body_block_statement_does_not_create_extra_block_scope() {
    // The catch clause itself is the only scope wrapping the
    // BlockStatement; there is no nested Block scope inside it.
    let r = analyze_source("try {} catch (e) { let x = 1; }\n", Language::Ts);
    let catch_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::Catch)
        .expect("a Catch scope must exist");
    let inner_blocks: Vec<_> = r.arena.scopes[catch_scope]
        .child_scopes
        .iter()
        .copied()
        .filter(|&id| matches!(r.arena.scopes[id].r#type, ScopeType::Block))
        .collect();
    assert!(
        inner_blocks.is_empty(),
        "catch body must reuse the catch scope, not create a nested Block scope"
    );
}

#[test]
fn if_body_block_statement_does_create_block_scope() {
    // The skip rule does NOT apply to IfStatement parents; the body
    // is a fresh Block scope.
    let r = analyze_source("if (true) { let x = 1; }\n", Language::Ts);
    let block_scope = r.arena.scopes[r.global_scope]
        .child_scopes
        .iter()
        .copied()
        .find(|&id| matches!(r.arena.scopes[id].r#type, ScopeType::Block));
    assert!(block_scope.is_some());
}
