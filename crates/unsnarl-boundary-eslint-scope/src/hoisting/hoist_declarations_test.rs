//! Sibling tests for `hoisting/hoist_declarations.rs`.

use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_names_in_scope};

#[test]
fn body_statements_hoist_each_declaration() {
    let r = analyze_source(
        "var a = 1; function f() {} class C {} const b = 2;\n",
        Language::Ts,
    );
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "f"));
    assert!(names.iter().any(|n| n == "C"));
    assert!(names.iter().any(|n| n == "b"));
}

#[test]
fn non_declaration_statements_do_not_introduce_hoisted_bindings() {
    // `1 + 2;` produces no binding at all.
    // `foo()` runs the walker which records an implicit-global
    // reference to `foo` — the resulting Variable lives on the
    // module scope but with `ImplicitGlobalVariable`, NOT
    // `Variable` / `FunctionName` / etc., so it is not a hoisted
    // declaration.
    let r = analyze_source("1 + 2;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.is_empty());
}
