//! Sibling tests for `hoisting/handle_class_declaration.rs`, mirroring
//! TS `ts/src/boundary/eslint-scope/hoisting/handle-class-declaration.test.ts`.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of, variable_names_in_scope};

#[test]
fn class_declaration_hoists_outer_class_name_into_module_scope() {
    let r = analyze_source("class C {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "C"));
    assert!(variable_has_def_of(
        &r.arena,
        "C",
        DefinitionType::ClassName
    ));
}

#[test]
fn anonymous_class_declaration_is_skipped() {
    // `export default class {}` has no id; nothing should be hoisted.
    let r = analyze_source("export default class {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    // Nothing was hoisted from the (anonymous) class declaration.
    assert!(!names.iter().any(|n| n.is_empty()));
}
