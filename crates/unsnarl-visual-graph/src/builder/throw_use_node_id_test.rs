//! Sibling tests for [`throw_use_node_id`]. Cases mirror
//! `ts/src/visual-graph/builder/throw-use-node-id.test.ts`.

use super::throw_use_node_id;

#[test]
fn alphanumerics_pass_through_with_prefix() {
    assert_eq!(throw_use_node_id("r42"), "throw_use_r42");
}

#[test]
fn specials_are_replaced_with_underscore() {
    assert_eq!(throw_use_node_id("r-1.2"), "throw_use_r_1_2");
}

#[test]
fn empty_input_yields_prefix_only() {
    assert_eq!(throw_use_node_id(""), "throw_use_");
}
