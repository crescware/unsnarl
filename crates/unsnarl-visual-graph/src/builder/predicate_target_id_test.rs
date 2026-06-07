//! Sibling tests for [`predicate_target_id`]. The implementation
//! takes the per-kind anchor maps via [`PredicateAnchorMaps`]
//! rather than the full `BuildState` shape, so each test builds
//! the wrapper from local [`HashMap`]s.

use std::collections::HashMap;

use unsnarl_oxc_parity::PredicateContainerType;

use super::{predicate_target_id, PredicateAnchorMaps};
use crate::builder::builder_fixtures::{base_serialized_reference, predicate_container};

fn empty_maps() -> [HashMap<u32, String>; 6] {
    Default::default()
}

fn anchors(maps: &[HashMap<u32, String>; 6]) -> PredicateAnchorMaps<'_> {
    PredicateAnchorMaps {
        if_test: &maps[0],
        switch_discriminant: &maps[1],
        while_test: &maps[2],
        do_while_test: &maps[3],
        for_test: &maps[4],
        conditional_test: &maps[5],
    }
}

#[test]
fn no_predicate_container_returns_none() {
    let r = base_serialized_reference();
    let maps = empty_maps();
    assert_eq!(predicate_target_id(&r, &anchors(&maps)), None);
}

#[test]
fn switch_statement_resolves_to_registered_anchor_by_offset() {
    let mut r = base_serialized_reference();
    r.predicate_container = Some(predicate_container(
        PredicateContainerType::SwitchStatement,
        100,
    ));
    let mut maps = empty_maps();
    maps[1].insert(100, "switch_discriminant_x".to_string());
    assert_eq!(
        predicate_target_id(&r, &anchors(&maps)),
        Some("switch_discriminant_x".to_string())
    );
}

#[test]
fn switch_statement_with_no_anchor_returns_none() {
    let mut r = base_serialized_reference();
    r.predicate_container = Some(predicate_container(
        PredicateContainerType::SwitchStatement,
        100,
    ));
    let maps = empty_maps();
    assert_eq!(predicate_target_id(&r, &anchors(&maps)), None);
}

#[test]
fn if_statement_with_no_anchor_returns_none() {
    let mut r = base_serialized_reference();
    r.predicate_container = Some(predicate_container(PredicateContainerType::IfStatement, 50));
    let maps = empty_maps();
    assert_eq!(predicate_target_id(&r, &anchors(&maps)), None);
}

#[test]
fn while_statement_resolves_to_registered_anchor_by_offset() {
    let mut r = base_serialized_reference();
    r.predicate_container = Some(predicate_container(
        PredicateContainerType::WhileStatement,
        33,
    ));
    let mut maps = empty_maps();
    maps[2].insert(33, "while_test_x".to_string());
    assert_eq!(
        predicate_target_id(&r, &anchors(&maps)),
        Some("while_test_x".to_string())
    );
}

#[test]
fn do_while_statement_resolves_to_registered_anchor_by_offset() {
    let mut r = base_serialized_reference();
    r.predicate_container = Some(predicate_container(
        PredicateContainerType::DoWhileStatement,
        33,
    ));
    let mut maps = empty_maps();
    maps[3].insert(33, "do_while_test_x".to_string());
    assert_eq!(
        predicate_target_id(&r, &anchors(&maps)),
        Some("do_while_test_x".to_string())
    );
}

#[test]
fn conditional_expression_resolves_to_registered_anchor_by_offset() {
    let mut r = base_serialized_reference();
    r.predicate_container = Some(predicate_container(
        PredicateContainerType::ConditionalExpression,
        72,
    ));
    let mut maps = empty_maps();
    maps[5].insert(72, "ternary_test_x".to_string());
    assert_eq!(
        predicate_target_id(&r, &anchors(&maps)),
        Some("ternary_test_x".to_string())
    );
}

#[test]
fn for_for_of_for_in_all_resolve_through_for_test_map() {
    let mut maps = empty_maps();
    maps[4].insert(40, "for_test_x".to_string());
    for t in [
        PredicateContainerType::ForStatement,
        PredicateContainerType::ForOfStatement,
        PredicateContainerType::ForInStatement,
    ] {
        let mut r = base_serialized_reference();
        r.predicate_container = Some(predicate_container(t, 40));
        assert_eq!(
            predicate_target_id(&r, &anchors(&maps)),
            Some("for_test_x".to_string())
        );
    }
}
