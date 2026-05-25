//! Sibling tests for [`previous_fallthrough_case`].
//! `branch_container_key` builds the `switch:<upper>:<offset>` key
//! from a `CaseClause` block context, so each case fixture uses the
//! `CaseClause` variant directly.

use std::collections::HashMap;

use unsnarl_ir::primitive::Utf16CodeUnitOffset;
use unsnarl_ir::scope::block_context::{BlockContext, CaseClauseBlockContext};
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::previous_fallthrough_case;
use crate::builder::builder_fixtures::{base_serialized_scope, scope_id};

fn case_scope(id: &str, parent_span_offset: u32, falls_through: bool) -> SerializedScope {
    let mut s = base_serialized_scope(id);
    s.upper = Some(scope_id("switch"));
    s.falls_through = falls_through;
    s.block_context = Some(BlockContext::CaseClause(CaseClauseBlockContext::new(
        AstType::SwitchStatement,
        "cases".to_string(),
        Utf16CodeUnitOffset(parent_span_offset),
        None,
    )));
    s
}

fn sorted(cases: &[SerializedScope]) -> HashMap<String, Vec<&SerializedScope>> {
    let mut map = HashMap::new();
    map.insert(
        "switch:switch:100".to_string(),
        cases.iter().collect::<Vec<_>>(),
    );
    map
}

#[test]
fn first_case_has_no_previous() {
    let c0 = case_scope("c0", 100, true);
    let c1 = case_scope("c1", 100, false);
    let c2 = case_scope("c2", 100, true);
    let c3 = case_scope("c3", 100, false);
    let cases = [c0, c1, c2, c3];
    let sorted = sorted(&cases);
    let prev = previous_fallthrough_case(&cases[0], &sorted);
    assert!(prev.is_none());
}

#[test]
fn previous_falls_through_returns_previous() {
    let c0 = case_scope("c0", 100, true);
    let c1 = case_scope("c1", 100, false);
    let c2 = case_scope("c2", 100, true);
    let c3 = case_scope("c3", 100, false);
    let cases = [c0, c1, c2, c3];
    let sorted = sorted(&cases);
    let prev = previous_fallthrough_case(&cases[1], &sorted);
    assert_eq!(
        prev.map(|s| s.id.value().to_string()),
        Some("c0".to_string())
    );
}

#[test]
fn previous_does_not_fall_through_returns_none() {
    let c0 = case_scope("c0", 100, true);
    let c1 = case_scope("c1", 100, false);
    let c2 = case_scope("c2", 100, true);
    let c3 = case_scope("c3", 100, false);
    let cases = [c0, c1, c2, c3];
    let sorted = sorted(&cases);
    let prev = previous_fallthrough_case(&cases[2], &sorted);
    assert!(prev.is_none());
}

#[test]
fn falls_through_chain_works_at_later_positions() {
    let c0 = case_scope("c0", 100, true);
    let c1 = case_scope("c1", 100, false);
    let c2 = case_scope("c2", 100, true);
    let c3 = case_scope("c3", 100, false);
    let cases = [c0, c1, c2, c3];
    let sorted = sorted(&cases);
    let prev = previous_fallthrough_case(&cases[3], &sorted);
    assert_eq!(
        prev.map(|s| s.id.value().to_string()),
        Some("c2".to_string())
    );
}

#[test]
fn scope_without_branch_container_key_returns_none() {
    let cases = [
        case_scope("c0", 100, true),
        case_scope("c1", 100, false),
        case_scope("c2", 100, true),
        case_scope("c3", 100, false),
    ];
    let sorted = sorted(&cases);
    let plain = base_serialized_scope("x");
    let prev = previous_fallthrough_case(&plain, &sorted);
    assert!(prev.is_none());
}

#[test]
fn container_key_not_in_map_returns_none() {
    let cases = [
        case_scope("c0", 100, true),
        case_scope("c1", 100, false),
        case_scope("c2", 100, true),
        case_scope("c3", 100, false),
    ];
    let sorted = sorted(&cases);
    let orphan = case_scope("orphan", 999, true);
    let prev = previous_fallthrough_case(&orphan, &sorted);
    assert!(prev.is_none());
}
