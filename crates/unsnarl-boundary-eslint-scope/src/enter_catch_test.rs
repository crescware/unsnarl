//! Sibling tests for `enter_catch.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/enter-catch.test.ts`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{
    analyze_source, find_first_descendant_scope, variable_has_def_of, variable_names_in_scope,
};

#[test]
fn catch_scope_binds_param() {
    let r = analyze_source("try { } catch (err) { }\n", Language::Ts);
    let catch_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::Catch)
        .expect("a Catch scope must exist");
    let names = variable_names_in_scope(&r.arena, catch_scope);
    assert!(names.iter().any(|n| n == "err"));
    assert!(variable_has_def_of(
        &r.arena,
        "err",
        DefinitionType::CatchClause
    ));
}

#[test]
fn catch_scope_with_destructuring_param_collects_each_identifier() {
    let r = analyze_source("try { } catch ({ a, b }) { }\n", Language::Ts);
    let catch_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::Catch)
        .expect("a Catch scope must exist");
    let names = variable_names_in_scope(&r.arena, catch_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
}
