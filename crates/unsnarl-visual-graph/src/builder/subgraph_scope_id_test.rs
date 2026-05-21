//! Sibling tests for [`subgraph_scope_id`]. Cases mirror
//! `ts/src/visual-graph/builder/subgraph-scope-id.test.ts`.

use super::subgraph_scope_id;
use crate::builder::testing::base_serialized_scope;

#[test]
fn alphanumeric_scope_id_pass_through() {
    let scope = base_serialized_scope("scope1");
    assert_eq!(subgraph_scope_id(&scope), "s_scope1");
}

#[test]
fn specials_in_scope_id_are_replaced() {
    let scope = base_serialized_scope("scope.1-x");
    assert_eq!(subgraph_scope_id(&scope), "s_scope_1_x");
}
