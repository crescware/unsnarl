//! Sibling tests for [`write_op_node_id`].

use super::write_op_node_id;

#[test]
fn alphanumerics_pass_through_with_prefix() {
    assert_eq!(write_op_node_id("ref42"), "wr_ref42");
}

#[test]
fn specials_are_replaced_with_underscore() {
    assert_eq!(write_op_node_id("ref:42/x"), "wr_ref_42_x");
}

#[test]
fn empty_input_yields_prefix_only() {
    assert_eq!(write_op_node_id(""), "wr_");
}
