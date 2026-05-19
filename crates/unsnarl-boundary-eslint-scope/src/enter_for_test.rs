//! Sibling tests for `enter_for.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/enter-for.test.ts`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, find_first_descendant_scope, variable_names_in_scope};

#[test]
fn for_let_binding_lives_in_for_scope() {
    let r = analyze_source("for (let i = 0; i < 10; i++) { i; }\n", Language::Ts);
    let for_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::For)
        .expect("a For scope must exist");
    let names = variable_names_in_scope(&r.arena, for_scope);
    assert!(names.iter().any(|n| n == "i"));
}

#[test]
fn for_var_hoists_out_to_module() {
    let r = analyze_source("for (var i = 0; i < 10; i++) { i; }\n", Language::Ts);
    let module_names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(module_names.iter().any(|n| n == "i"));
}

#[test]
fn for_in_let_binding_lives_in_for_scope() {
    let r = analyze_source("for (let k in obj) { k; }\n", Language::Ts);
    let for_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::For)
        .expect("a For scope must exist");
    let names = variable_names_in_scope(&r.arena, for_scope);
    assert!(names.iter().any(|n| n == "k"));
}

#[test]
fn for_of_let_binding_lives_in_for_scope() {
    let r = analyze_source("for (let v of arr) { v; }\n", Language::Ts);
    let for_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::For)
        .expect("a For scope must exist");
    let names = variable_names_in_scope(&r.arena, for_scope);
    assert!(names.iter().any(|n| n == "v"));
}
