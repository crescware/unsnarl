//! Sibling tests for `hoisting/visit.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/hoisting/visit.test.ts`.

use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_names_in_scope};

#[test]
fn export_named_declaration_recurses_into_inner_declaration() {
    let r = analyze_source("export const x = 1;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "x"));
}

#[test]
fn export_default_function_declaration_recurses_into_function() {
    let r = analyze_source("export default function f() {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "f"));
}

#[test]
fn export_default_class_declaration_recurses_into_class() {
    let r = analyze_source("export default class C {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "C"));
}

#[test]
fn unrelated_statement_types_are_ignored_by_hoist_visit() {
    let r = analyze_source("if (true) {}\n", Language::Ts);
    // IfStatement is not a declaration shape, so it produces no
    // hoisted bindings (the inner BlockStatement is handled by the
    // walker, not the hoist pass).
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.is_empty());
}
