//! Sibling tests for [`return_subgraph_id`].

use super::return_subgraph_id;

#[test]
fn alphanumeric_var_and_key_pass_through() {
    assert_eq!(return_subgraph_id("v1", "10-20"), "s_return_v1_10_20");
}

#[test]
fn specials_in_var_id_are_replaced() {
    assert_eq!(
        return_subgraph_id("v.1", "implicit"),
        "s_return_v_1_implicit"
    );
}

#[test]
fn empty_inputs_keep_separators() {
    assert_eq!(return_subgraph_id("", ""), "s_return__");
}

#[test]
fn specials_in_both_arguments_are_replaced() {
    assert_eq!(return_subgraph_id("owner-x", "5-9"), "s_return_owner_x_5_9");
}
