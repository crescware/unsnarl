//! Sibling tests for `enter_function.rs`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_names_in_scope};

#[test]
fn function_creates_function_scope_with_implicit_arguments() {
    let r = analyze_source("function f(a, b) { return a + b; }\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let scope = &r.arena.scopes[function_scope];
    assert!(matches!(scope.r#type, ScopeType::Function));
    let names = variable_names_in_scope(&r.arena, function_scope);
    assert!(names.iter().any(|n| n == "arguments"));
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
}

#[test]
fn arrow_function_skips_implicit_arguments() {
    let r = analyze_source("const f = (a, b) => a + b;\n", Language::Ts);
    let arrow_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, arrow_scope);
    assert!(!names.iter().any(|n| n == "arguments"));
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
}

#[test]
fn function_body_hoists_var_into_function_scope() {
    let r = analyze_source("function f() { var local = 1; }\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, function_scope);
    assert!(names.iter().any(|n| n == "local"));
}
