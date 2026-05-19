//! Sibling tests for `hoisting/handle_import_declaration.rs`,
//! mirroring TS
//! `ts/src/boundary/eslint-scope/hoisting/handle-import-declaration.test.ts`.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of, variable_names_in_scope};

#[test]
fn named_import_specifier_local_binds_in_module_scope() {
    let r = analyze_source("import { bar as baz } from 'mod';\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "baz"));
    assert!(variable_has_def_of(
        &r.arena,
        "baz",
        DefinitionType::ImportBinding
    ));
}

#[test]
fn default_import_specifier_local_binds_in_module_scope() {
    let r = analyze_source("import foo from 'mod';\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "foo"));
    assert!(variable_has_def_of(
        &r.arena,
        "foo",
        DefinitionType::ImportBinding
    ));
}

#[test]
fn namespace_import_specifier_local_binds_in_module_scope() {
    let r = analyze_source("import * as ns from 'mod';\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "ns"));
    assert!(variable_has_def_of(
        &r.arena,
        "ns",
        DefinitionType::ImportBinding
    ));
}
