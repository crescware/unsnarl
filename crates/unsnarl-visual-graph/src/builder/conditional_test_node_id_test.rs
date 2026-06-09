//! Sibling tests for [`conditional_test_node_id`].

use super::conditional_test_node_id;

#[test]
fn alphanumeric_scope_id_pass_through() {
    assert_eq!(
        conditional_test_node_id("scope1", 42),
        "ternary_test_scope1_42"
    );
}

#[test]
fn specials_in_scope_id_are_replaced() {
    assert_eq!(conditional_test_node_id("a.b", 100), "ternary_test_a_b_100");
}

#[test]
fn zero_offset_renders_as_zero() {
    assert_eq!(conditional_test_node_id("s", 0), "ternary_test_s_0");
}

#[test]
fn empty_scope_id_keeps_separator() {
    assert_eq!(conditional_test_node_id("", 7), "ternary_test__7");
}
