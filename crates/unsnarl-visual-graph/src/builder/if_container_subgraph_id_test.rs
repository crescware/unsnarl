//! Sibling tests for [`if_container_subgraph_id`].

use super::if_container_subgraph_id;

#[test]
fn alphanumeric_scope_id_pass_through() {
    assert_eq!(if_container_subgraph_id("scope1", 42), "cont_if_scope1_42");
}

#[test]
fn specials_in_scope_id_are_replaced() {
    assert_eq!(if_container_subgraph_id("a.b", 100), "cont_if_a_b_100");
}

#[test]
fn zero_offset_renders_as_zero() {
    assert_eq!(if_container_subgraph_id("s", 0), "cont_if_s_0");
}

#[test]
fn empty_scope_id_keeps_separator() {
    assert_eq!(if_container_subgraph_id("", 7), "cont_if__7");
}
