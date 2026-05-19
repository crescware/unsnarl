//! Sibling tests for `handle_identifier_reference.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/handle-identifier-reference.test.ts`.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of};

#[test]
fn identifier_reference_resolves_to_module_binding() {
    let r = analyze_source("const x = 1; const y = x;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .expect("x must be bound");
    let refs_to_x: Vec<_> = r
        .arena
        .references
        .iter()
        .filter(|r| r.resolved == Some(x_id))
        .collect();
    assert!(!refs_to_x.is_empty());
}

#[test]
fn undefined_reference_creates_implicit_global() {
    let r = analyze_source("y;\n", Language::Ts);
    assert!(r.arena.scopes[r.global_scope].set().contains_key("y"));
    assert!(variable_has_def_of(
        &r.arena,
        "y",
        DefinitionType::ImplicitGlobalVariable
    ));
}

#[test]
fn reference_walks_scope_chain_through_for_resolution() {
    // `x` is declared at the module scope; references inside the for
    // body must still resolve to that module binding (and the for
    // scope records the reference on its `through` array).
    let r = analyze_source(
        "let x = 1; for (let i = 0; i < 1; i++) { x; }\n",
        Language::Ts,
    );
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    let resolved_to_x = r
        .arena
        .references
        .iter()
        .filter(|r| r.resolved == Some(x_id))
        .count();
    assert!(resolved_to_x >= 1);
}
