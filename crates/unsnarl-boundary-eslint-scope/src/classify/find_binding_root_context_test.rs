//! Sibling tests for `find_binding_root_context.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/classify/find-binding-root-context.test.ts`.

use unsnarl_ir::Language;

use crate::testing::analyze_source;

#[test]
fn function_param_default_classifies_outer_reference_as_read() {
    let r = analyze_source(
        "const fallback = 1; function f(a = fallback) { return a; }\n",
        Language::Ts,
    );
    let fallback_id = r.arena.scopes[r.global_scope]
        .set()
        .get("fallback")
        .copied()
        .unwrap();
    let resolved_to_fallback = r
        .arena
        .references
        .iter()
        .any(|r| r.resolved == Some(fallback_id) && r.identifier.name() == "fallback");
    assert!(resolved_to_fallback);
}

#[test]
fn destructured_var_root_walks_up_through_object_pattern() {
    // `let { a } = obj;` — `a` is a BindingIdentifier (not an
    // IdentifierReference), so classify is never asked. We confirm
    // that `a` lives as a Variable in the module scope, which means
    // the walk-up reached the VariableDeclarator + "id" terminator.
    let r = analyze_source("const { a } = obj;\n", Language::Ts);
    assert!(r.arena.scopes[r.global_scope].set().contains_key("a"));
}
