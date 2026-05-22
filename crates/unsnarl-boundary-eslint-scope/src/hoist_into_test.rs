//! Sibling tests for `hoist_into.rs`.
//!
//! The `hoist_into` pass is exercised via `analyze_source`'s
//! implicit program-level hoisting.

use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_names_in_scope};

#[test]
fn program_body_var_let_const_hoist_into_module_scope() {
    let r = analyze_source("var a = 1; let b = 2; const c = 3;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
    assert!(names.iter().any(|n| n == "c"));
}

#[test]
fn program_body_function_declaration_hoists() {
    let r = analyze_source("function f() {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "f"));
}

#[test]
fn program_body_class_declaration_hoists() {
    let r = analyze_source("class C {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "C"));
}
