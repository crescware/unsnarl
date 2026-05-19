//! Sibling tests for `is_direct_binding.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/classify/is-direct-binding.test.ts`.
//!
//! In oxc, most "direct binding" slots are inhabited by
//! `BindingIdentifier` (a distinct AST type from
//! `IdentifierReference`), so the classify check is intrinsically
//! satisfied by the type system. The tests below assert the
//! resulting definition shape.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of};

#[test]
fn function_name_binding_marked_as_function_name() {
    let r = analyze_source("function f() {}\n", Language::Ts);
    assert!(variable_has_def_of(
        &r.arena,
        "f",
        DefinitionType::FunctionName
    ));
}

#[test]
fn class_name_binding_marked_as_class_name() {
    let r = analyze_source("class C {}\n", Language::Ts);
    assert!(variable_has_def_of(
        &r.arena,
        "C",
        DefinitionType::ClassName
    ));
}

#[test]
fn function_param_binding_marked_as_parameter() {
    let r = analyze_source("function f(a) {}\n", Language::Ts);
    assert!(variable_has_def_of(
        &r.arena,
        "a",
        DefinitionType::Parameter
    ));
}

#[test]
fn catch_param_binding_marked_as_catch_clause() {
    let r = analyze_source("try {} catch (e) {}\n", Language::Ts);
    assert!(variable_has_def_of(
        &r.arena,
        "e",
        DefinitionType::CatchClause
    ));
}

#[test]
fn import_specifier_local_binding_marked_as_import_binding() {
    let r = analyze_source("import foo from 'mod';\n", Language::Ts);
    assert!(variable_has_def_of(
        &r.arena,
        "foo",
        DefinitionType::ImportBinding
    ));
}
