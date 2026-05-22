//! Sibling tests for `enter_class.rs`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of, variable_names_in_scope};

#[test]
fn class_declaration_creates_inner_class_scope_with_class_name_binding() {
    let r = analyze_source("class C {}\n", Language::Ts);
    let class_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let inner = &r.arena.scopes[class_scope];
    assert!(matches!(inner.r#type, ScopeType::Class));
    let inner_names = variable_names_in_scope(&r.arena, class_scope);
    assert!(inner_names.iter().any(|n| n == "C"));
    assert!(variable_has_def_of(
        &r.arena,
        "C",
        DefinitionType::ClassName
    ));
}

#[test]
fn class_expression_creates_class_scope_without_outer_binding() {
    let r = analyze_source("const x = class Inner {};\n", Language::Ts);
    let outer_names = variable_names_in_scope(&r.arena, r.global_scope);
    // The outer `Inner` binding is NOT registered (only ClassDeclaration hoists);
    // only `x` lives in the module scope.
    assert!(outer_names.iter().any(|n| n == "x"));
    assert!(!outer_names.iter().any(|n| n == "Inner"));
}
