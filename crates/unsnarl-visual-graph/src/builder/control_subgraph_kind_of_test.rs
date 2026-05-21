//! Sibling tests for [`control_subgraph_kind_of`]. Cases mirror
//! `ts/src/visual-graph/builder/control-subgraph-kind-of.test.ts`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use super::control_subgraph_kind_of;
use crate::builder::testing::{base_serialized_scope, other_block_context};
use crate::visual_subgraph::ControlSubgraphKind;

fn check_scope_type(t: ScopeType) -> Option<ControlSubgraphKind> {
    let mut s = base_serialized_scope("s");
    s.r#type = t;
    control_subgraph_kind_of(&s)
}

#[test]
fn catch_scope_maps_to_catch() {
    assert!(matches!(
        check_scope_type(ScopeType::Catch),
        Some(ControlSubgraphKind::Catch)
    ));
}

#[test]
fn for_scope_maps_to_for() {
    assert!(matches!(
        check_scope_type(ScopeType::For),
        Some(ControlSubgraphKind::For)
    ));
}

#[test]
fn switch_scope_maps_to_switch() {
    assert!(matches!(
        check_scope_type(ScopeType::Switch),
        Some(ControlSubgraphKind::Switch)
    ));
}

#[test]
fn function_scope_maps_to_none() {
    assert!(check_scope_type(ScopeType::Function).is_none());
}

#[test]
fn module_scope_maps_to_none() {
    assert!(check_scope_type(ScopeType::Module).is_none());
}

#[test]
fn global_scope_maps_to_none() {
    assert!(check_scope_type(ScopeType::Global).is_none());
}

#[test]
fn class_scope_maps_to_none() {
    assert!(check_scope_type(ScopeType::Class).is_none());
}

#[test]
fn bare_block_scope_maps_to_block() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Block;
    assert!(matches!(
        control_subgraph_kind_of(&s),
        Some(ControlSubgraphKind::Block)
    ));
}

fn check_block_context(parent: AstType, key: &str) -> Option<ControlSubgraphKind> {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Block;
    s.block_context = Some(other_block_context(parent, key, 0, None));
    control_subgraph_kind_of(&s)
}

#[test]
fn try_block_maps_to_try() {
    assert!(matches!(
        check_block_context(AstType::TryStatement, "block"),
        Some(ControlSubgraphKind::Try)
    ));
}

#[test]
fn try_finalizer_maps_to_finally() {
    assert!(matches!(
        check_block_context(AstType::TryStatement, "finalizer"),
        Some(ControlSubgraphKind::Finally)
    ));
}

#[test]
fn try_handler_maps_to_block() {
    assert!(matches!(
        check_block_context(AstType::TryStatement, "handler"),
        Some(ControlSubgraphKind::Block)
    ));
}

#[test]
fn if_consequent_maps_to_if() {
    assert!(matches!(
        check_block_context(AstType::IfStatement, "consequent"),
        Some(ControlSubgraphKind::If)
    ));
}

#[test]
fn if_alternate_maps_to_else() {
    assert!(matches!(
        check_block_context(AstType::IfStatement, "alternate"),
        Some(ControlSubgraphKind::Else)
    ));
}

#[test]
fn if_test_maps_to_block() {
    assert!(matches!(
        check_block_context(AstType::IfStatement, "test"),
        Some(ControlSubgraphKind::Block)
    ));
}

#[test]
fn switch_cases_maps_to_case() {
    assert!(matches!(
        check_block_context(AstType::SwitchStatement, "cases"),
        Some(ControlSubgraphKind::Case)
    ));
}

#[test]
fn switch_discriminant_maps_to_block() {
    assert!(matches!(
        check_block_context(AstType::SwitchStatement, "discriminant"),
        Some(ControlSubgraphKind::Block)
    ));
}

#[test]
fn while_body_maps_to_while() {
    assert!(matches!(
        check_block_context(AstType::WhileStatement, "body"),
        Some(ControlSubgraphKind::While)
    ));
}

#[test]
fn do_while_body_maps_to_do_while() {
    assert!(matches!(
        check_block_context(AstType::DoWhileStatement, "body"),
        Some(ControlSubgraphKind::DoWhile)
    ));
}
