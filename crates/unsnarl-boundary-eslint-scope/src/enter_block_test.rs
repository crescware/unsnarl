//! Sibling tests for `enter_block.rs`.
//!
//! Integration-style: feed a source string through `analyze_source`
//! and observe the resulting scope chain. The TS unit tests that
//! drove `enterBlock` with a hand-built `NodeLike` are subsumed
//! here.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_names_in_scope};

#[test]
fn block_statement_creates_block_scope_for_let() {
    let r = analyze_source("{ let inner = 1; }\n", Language::Ts);
    let block_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let scope = &r.arena.scopes[block_scope];
    assert!(matches!(scope.r#type, ScopeType::Block));
    let names = variable_names_in_scope(&r.arena, block_scope);
    assert!(names.iter().any(|n| n == "inner"));
}

#[test]
fn block_statement_hoists_inner_var_into_block_let_const_pair() {
    let r = analyze_source("{ const a = 1; let b = 2; }\n", Language::Ts);
    let block_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, block_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
}
