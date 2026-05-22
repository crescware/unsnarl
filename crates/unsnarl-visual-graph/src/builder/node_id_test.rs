//! Sibling tests for [`node_id`].

use super::node_id;

#[test]
fn alphanumerics_pass_through_with_prefix() {
    assert_eq!(node_id("foo123"), "n_foo123");
}

#[test]
fn specials_are_replaced_with_underscore() {
    assert_eq!(node_id("a.b-c"), "n_a_b_c");
}

#[test]
fn empty_input_yields_prefix_only() {
    assert_eq!(node_id(""), "n_");
}
