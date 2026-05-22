//! Sibling tests for [`while_test_node_id`], [`do_while_test_node_id`],
//! and [`for_test_node_id`].

use super::{do_while_test_node_id, for_test_node_id, while_test_node_id};

#[test]
fn while_test_composes_prefix_scope_offset() {
    assert_eq!(while_test_node_id("scope_0", 33), "while_test_scope_0_33");
}

#[test]
fn while_test_sanitises_non_alphanumerics_in_scope_id() {
    assert_eq!(
        while_test_node_id("scope.0:nested", 7),
        "while_test_scope_0_nested_7"
    );
}

#[test]
fn while_test_accepts_empty_parent_scope_id() {
    assert_eq!(while_test_node_id("", 12), "while_test__12");
}

#[test]
fn do_while_test_composes_prefix_scope_offset() {
    assert_eq!(
        do_while_test_node_id("scope_0", 41),
        "do_while_test_scope_0_41"
    );
}

#[test]
fn do_while_test_sanitises_non_alphanumerics_in_scope_id() {
    assert_eq!(do_while_test_node_id("a-b", 5), "do_while_test_a_b_5");
}

#[test]
fn for_test_composes_prefix_scope_offset() {
    assert_eq!(for_test_node_id("scope_1", 99), "for_test_scope_1_99");
}

#[test]
fn for_test_sanitises_non_alphanumerics_in_scope_id() {
    assert_eq!(for_test_node_id("scope#2!", 0), "for_test_scope_2__0");
}
