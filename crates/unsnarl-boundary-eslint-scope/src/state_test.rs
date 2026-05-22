//! Sibling tests for `state.rs`.
//!
//! The TS port colocates `declareVariable` in `declare/`; the Rust
//! port pulls it into `state.rs` because it directly mutates
//! `ScopeBuilderState`. The sibling tests therefore live next to the
//! Rust impl.

use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::testing::{analyze_source, variable_has_def_of};

#[test]
fn declare_variable_creates_variable_and_definition_pair() {
    let r = analyze_source("let x = 1;\n", Language::Ts);
    assert!(r.arena.scopes[r.global_scope].set().contains_key("x"));
    assert!(variable_has_def_of(&r.arena, "x", DefinitionType::Variable));
}

#[test]
fn declare_variable_reuses_existing_binding_for_same_name() {
    // Two `var` bindings of the same name should produce one Variable
    // row with two definitions, not two Variable rows.
    let r = analyze_source("var x = 1; var x = 2;\n", Language::Ts);
    let count: usize = r.arena.variables.iter().filter(|v| v.name() == "x").count();
    assert_eq!(count, 1);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    assert_eq!(r.arena.variables[x_id].defs.len(), 2);
}

#[test]
fn declare_variable_collects_identifiers_for_each_definition() {
    let r = analyze_source("var x = 1; var x = 2;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    assert_eq!(r.arena.variables[x_id].identifiers.len(), 2);
}
