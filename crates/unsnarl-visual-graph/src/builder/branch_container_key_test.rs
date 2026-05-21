//! Sibling tests for [`branch_container_key`]. Cases mirror
//! `ts/src/visual-graph/builder/branch-container-key.test.ts`. The
//! Rust impl splits `BlockContext` into `Other` (if / try) and
//! `CaseClause` (switch cases), so the switch-cases scenarios build
//! the `CaseClause` variant directly while every other scenario
//! uses the `other_block_context` helper.

use unsnarl_ir::primitive::SourceOffset;
use unsnarl_ir::scope::block_context::{BlockContext, CaseClauseBlockContext};
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::branch_container_key;
use crate::builder::testing::{base_serialized_scope, other_block_context, scope_id};

fn scope_with(upper: Option<&str>, ctx: Option<BlockContext>) -> SerializedScope {
    let mut s = base_serialized_scope("inner");
    s.upper = upper.map(scope_id);
    s.block_context = ctx;
    s
}

fn case_clause(parent: AstType, key: &str, offset: u32) -> BlockContext {
    BlockContext::CaseClause(CaseClauseBlockContext::new(
        parent,
        key.to_string(),
        SourceOffset(offset),
        None,
    ))
}

#[test]
fn returns_none_when_block_context_is_missing() {
    let s = base_serialized_scope("inner");
    assert_eq!(branch_container_key(&s), None);
}

#[test]
fn switch_cases_returns_switch_key() {
    let s = scope_with(
        Some("outer"),
        Some(case_clause(AstType::SwitchStatement, "cases", 12)),
    );
    assert_eq!(
        branch_container_key(&s),
        Some("switch:outer:12".to_string())
    );
}

#[test]
fn if_consequent_returns_if_key() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(
            AstType::IfStatement,
            "consequent",
            3,
            None,
        )),
    );
    assert_eq!(branch_container_key(&s), Some("if:outer:3".to_string()));
}

#[test]
fn if_alternate_returns_if_key() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(
            AstType::IfStatement,
            "alternate",
            3,
            None,
        )),
    );
    assert_eq!(branch_container_key(&s), Some("if:outer:3".to_string()));
}

#[test]
fn switch_with_non_cases_key_returns_none() {
    let s = scope_with(
        Some("outer"),
        Some(case_clause(AstType::SwitchStatement, "discriminant", 7)),
    );
    assert_eq!(branch_container_key(&s), None);
}

#[test]
fn if_with_key_other_than_consequent_or_alternate_returns_none() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(AstType::IfStatement, "test", 3, None)),
    );
    assert_eq!(branch_container_key(&s), None);
}

#[test]
fn try_block_returns_try_key() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(AstType::TryStatement, "block", 9, None)),
    );
    assert_eq!(branch_container_key(&s), Some("try:outer:9".to_string()));
}

#[test]
fn try_handler_returns_try_key() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(
            AstType::TryStatement,
            "handler",
            9,
            None,
        )),
    );
    assert_eq!(branch_container_key(&s), Some("try:outer:9".to_string()));
}

#[test]
fn try_finalizer_returns_none() {
    // `finally` is post-merge, not a sibling branch.
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(
            AstType::TryStatement,
            "finalizer",
            9,
            None,
        )),
    );
    assert_eq!(branch_container_key(&s), None);
}

#[test]
fn if_branch_with_chain_root_offset_uses_chain_root_for_the_key() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(
            AstType::IfStatement,
            "consequent",
            40,
            Some(5),
        )),
    );
    assert_eq!(branch_container_key(&s), Some("if:outer:5".to_string()));
}

#[test]
fn if_alternate_with_chain_root_shares_the_same_chain_key() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(
            AstType::IfStatement,
            "alternate",
            40,
            Some(5),
        )),
    );
    assert_eq!(branch_container_key(&s), Some("if:outer:5".to_string()));
}

#[test]
fn unrelated_parent_type_returns_none() {
    let s = scope_with(
        Some("outer"),
        Some(other_block_context(AstType::ForStatement, "body", 5, None)),
    );
    assert_eq!(branch_container_key(&s), None);
}

#[test]
fn null_upper_renders_as_empty_string_in_the_key() {
    let s = scope_with(
        None,
        Some(other_block_context(
            AstType::IfStatement,
            "consequent",
            1,
            None,
        )),
    );
    assert_eq!(branch_container_key(&s), Some("if::1".to_string()));
}
