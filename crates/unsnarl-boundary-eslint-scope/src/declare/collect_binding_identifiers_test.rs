//! Sibling tests for `collect_binding_identifiers.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/declare/collect-binding-identifiers.test.ts`.

use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_names_in_scope};

#[test]
fn object_pattern_collects_each_identifier_including_rename() {
    let r = analyze_source("const { a, b: c, ...rest } = obj;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
    // `b: c` renames to `c`; the binding is `c`.
    assert!(names.iter().any(|n| n == "c"));
    assert!(names.iter().any(|n| n == "rest"));
}

#[test]
fn array_pattern_collects_each_identifier_skipping_holes() {
    let r = analyze_source("const [a, , b, ...rest] = arr;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
    assert!(names.iter().any(|n| n == "rest"));
}

#[test]
fn assignment_pattern_collects_left_identifier() {
    let r = analyze_source("const { a = 1 } = obj;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
}

#[test]
fn deeply_nested_pattern_collects_each_identifier() {
    let r = analyze_source("const { x: [y, { z }] } = obj;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "y"));
    assert!(names.iter().any(|n| n == "z"));
}
