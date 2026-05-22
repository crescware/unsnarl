//! Sibling tests for `declare_function_params.rs`.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of, variable_names_in_scope};

#[test]
fn function_params_declared_as_parameter() {
    let r = analyze_source("function f(a, b, c) {}\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, function_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
    assert!(names.iter().any(|n| n == "c"));
    assert!(variable_has_def_of(
        &r.arena,
        "a",
        DefinitionType::Parameter
    ));
}

#[test]
fn rest_parameter_declared_as_parameter() {
    let r = analyze_source("function f(...rest) {}\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, function_scope);
    assert!(names.iter().any(|n| n == "rest"));
    assert!(variable_has_def_of(
        &r.arena,
        "rest",
        DefinitionType::Parameter
    ));
}

#[test]
fn destructuring_parameter_declares_each_identifier() {
    let r = analyze_source("function f({ a, b }) {}\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, function_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
}

#[test]
fn arrow_function_params_declared_as_parameter() {
    let r = analyze_source("const f = (x) => x;\n", Language::Ts);
    let arrow_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, arrow_scope);
    assert!(names.iter().any(|n| n == "x"));
    assert!(variable_has_def_of(
        &r.arena,
        "x",
        DefinitionType::Parameter
    ));
}
