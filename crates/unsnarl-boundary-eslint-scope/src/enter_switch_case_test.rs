//! Sibling tests for `enter_switch_case.rs`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, find_first_descendant_scope, variable_names_in_scope};

#[test]
fn switch_case_block_scope_holds_lexical_declarations_in_consequent() {
    // A `let` placed directly inside the SwitchCase consequent (no
    // additional BlockStatement wrapper) lives on the case's own
    // Block scope.
    let r = analyze_source("switch (1) { case 1: let x = 2; break; }\n", Language::Ts);
    let switch_scope =
        find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::Switch).unwrap();
    let case_block = r.arena.scopes[switch_scope]
        .child_scopes
        .iter()
        .copied()
        .find(|&c| matches!(r.arena.scopes[c].r#type, ScopeType::Block))
        .expect("the switch must have a case-clause Block scope");
    let names = variable_names_in_scope(&r.arena, case_block);
    assert!(names.iter().any(|n| n == "x"));
}

#[test]
fn switch_case_with_block_consequent_creates_block_scope_chain() {
    // When the consequent is itself a `{ ... }`, the walker
    // additionally creates a nested Block scope inside the
    // case-Block scope.
    let r = analyze_source("switch (1) { case 1: { break; } }\n", Language::Ts);
    let switch_scope =
        find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::Switch).unwrap();
    let case_block = r.arena.scopes[switch_scope]
        .child_scopes
        .iter()
        .copied()
        .find(|&c| matches!(r.arena.scopes[c].r#type, ScopeType::Block))
        .expect("the switch must have a case-clause Block scope");
    let inner_block_exists = r.arena.scopes[case_block]
        .child_scopes
        .iter()
        .any(|&c| matches!(r.arena.scopes[c].r#type, ScopeType::Block));
    assert!(inner_block_exists);
}
