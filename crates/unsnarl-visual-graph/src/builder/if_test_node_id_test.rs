//! Sibling tests for [`if_test_node_id`].

use super::if_test_node_id;

#[test]
fn alphanumeric_scope_id_pass_through() {
    assert_eq!(if_test_node_id("scope1", 42), "if_test_scope1_42");
}

#[test]
fn specials_in_scope_id_are_replaced() {
    assert_eq!(if_test_node_id("a.b", 100), "if_test_a_b_100");
}

#[test]
fn zero_offset_renders_as_zero() {
    assert_eq!(if_test_node_id("s", 0), "if_test_s_0");
}

#[test]
fn empty_scope_id_keeps_separator() {
    assert_eq!(if_test_node_id("", 7), "if_test__7");
}
