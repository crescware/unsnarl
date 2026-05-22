//! Sibling tests for `is_pattern_step.rs`.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of, variable_names_in_scope};

#[test]
fn nested_object_pattern_binding_resolves_through_pattern_chain() {
    // `let { a: { b } } = obj;` exercises the BindingProperty / Object
    // pattern chain. Both `a`-as-rename-target (no — `a` is the key,
    // not a binding) and `b` (the actual binding) need to be
    // classified: `b` should be a Variable definition.
    let r = analyze_source("const { a: { b } } = obj;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "b"));
    assert!(variable_has_def_of(&r.arena, "b", DefinitionType::Variable));
}

#[test]
fn array_pattern_inside_object_pattern_collects_each_identifier() {
    let r = analyze_source("const { a: [b, c] } = obj;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "b"));
    assert!(names.iter().any(|n| n == "c"));
}

#[test]
fn rest_element_inside_array_pattern_collects_identifier() {
    let r = analyze_source("const [head, ...tail] = arr;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "head"));
    assert!(names.iter().any(|n| n == "tail"));
}

#[test]
fn assignment_pattern_left_collects_identifier() {
    // `const [{ x = 1 }] = arr;` — `x` is a binding (default 1).
    let r = analyze_source("const [{ x = 1 }] = arr;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "x"));
}
