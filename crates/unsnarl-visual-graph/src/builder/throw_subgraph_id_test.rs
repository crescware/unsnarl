//! Sibling tests for [`throw_subgraph_id`]. Cases mirror
//! `ts/src/visual-graph/builder/throw-subgraph-id.test.ts`.

use super::throw_subgraph_id;

#[test]
fn alphanumeric_var_and_key_pass_through() {
    assert_eq!(throw_subgraph_id("v1", "10-20"), "s_throw_v1_10_20");
}

#[test]
fn specials_in_var_id_are_replaced() {
    assert_eq!(throw_subgraph_id("v.1", "implicit"), "s_throw_v_1_implicit");
}

#[test]
fn empty_inputs_keep_separators() {
    assert_eq!(throw_subgraph_id("", ""), "s_throw__");
}

#[test]
fn specials_in_both_arguments_are_replaced() {
    assert_eq!(throw_subgraph_id("owner-x", "5-9"), "s_throw_owner_x_5_9");
}
