//! Sibling tests for `enter_switch.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/enter-switch.test.ts`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, find_first_descendant_scope};

#[test]
fn switch_statement_creates_switch_scope() {
    let r = analyze_source("switch (1) { case 1: { break; } }\n", Language::Ts);
    let switch_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::Switch)
        .expect("a Switch scope must exist");
    assert!(matches!(
        r.arena.scopes[switch_scope].r#type,
        ScopeType::Switch
    ));
}

#[test]
fn switch_scope_does_not_declare_anything_directly() {
    let r = analyze_source("switch (1) { }\n", Language::Ts);
    let switch_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::Switch)
        .expect("a Switch scope must exist");
    // Switch scope itself carries no variables; declarations live in
    // its case-clause Block sub-scopes.
    assert!(r.arena.scopes[switch_scope].variables.is_empty());
}
