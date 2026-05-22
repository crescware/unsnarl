//! Sibling tests for `hoisting/handle_function_declaration.rs`.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of, variable_names_in_scope};

#[test]
fn function_declaration_hoists_function_name_into_module_scope() {
    let r = analyze_source("function f() {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "f"));
    assert!(variable_has_def_of(
        &r.arena,
        "f",
        DefinitionType::FunctionName
    ));
}

#[test]
fn function_declaration_hoist_predates_inner_walk() {
    // The binding for `f` must be reachable from inside the function
    // body — the hoist runs before the walker descends into the
    // function, so a `let g = f;` inside the body resolves to the
    // outer `f`.
    let r = analyze_source("function f() { const g = f; }\n", Language::Ts);
    let f_id = r.arena.scopes[r.global_scope]
        .set()
        .get("f")
        .copied()
        .unwrap();
    let resolved_to_f = r
        .arena
        .references
        .iter()
        .any(|r| r.resolved == Some(f_id) && r.identifier.name() == "f");
    assert!(resolved_to_f);
}
