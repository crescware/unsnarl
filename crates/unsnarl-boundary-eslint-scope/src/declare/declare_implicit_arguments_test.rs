//! Sibling tests for `declare_implicit_arguments.rs`.

use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_names_in_scope};

#[test]
fn function_scope_declares_arguments() {
    let r = analyze_source("function f() {}\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, function_scope);
    assert!(names.iter().any(|n| n == "arguments"));
}

#[test]
fn arrow_function_scope_does_not_declare_arguments() {
    let r = analyze_source("const f = () => 1;\n", Language::Ts);
    let arrow_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, arrow_scope);
    assert!(!names.iter().any(|n| n == "arguments"));
}

#[test]
fn function_arguments_has_no_identifiers_or_defs() {
    let r = analyze_source("function f() {}\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let arguments_id = r.arena.scopes[function_scope]
        .set()
        .get("arguments")
        .copied()
        .expect("`arguments` must be bound in the function scope");
    let v = &r.arena.variables[arguments_id];
    assert!(v.identifiers.is_empty());
    assert!(v.defs.is_empty());
}
