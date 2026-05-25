use std::collections::HashSet;

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::{
    ControlSubgraphKind, OwnedExtras, OwnedVisualSubgraph, VisualSubgraph,
};

use super::collect_wrapped_owner_ids;
use crate::mermaid_fixtures::{base_const_binding, base_function_subgraph, base_plain_subgraph};

fn function_subgraph_with(owner_node_id: &str, elements: Vec<VisualElement>) -> VisualElement {
    let sg = OwnedVisualSubgraph {
        extras: OwnedExtras::Function {
            owner_node_id: Some(owner_node_id.to_string()),
            owner_name: "owner".to_string(),
        },
        elements,
        ..base_function_subgraph()
    };
    VisualSubgraph::from(sg).into()
}

#[test]
fn captures_owner_node_id_of_every_function_subgraph() {
    let elements = vec![
        function_subgraph_with("n_a", Vec::new()),
        function_subgraph_with("n_b", Vec::new()),
    ];
    let mut out: HashSet<String> = HashSet::new();
    collect_wrapped_owner_ids(&elements, &mut out);
    let mut got: Vec<String> = out.into_iter().collect();
    got.sort();
    assert_eq!(got, vec!["n_a".to_string(), "n_b".to_string()]);
}

#[test]
fn non_function_subgraphs_carry_no_owner_node_id_and_are_skipped() {
    let if_sg: VisualElement =
        VisualSubgraph::from(base_plain_subgraph(ControlSubgraphKind::If)).into();
    let mut out: HashSet<String> = HashSet::new();
    collect_wrapped_owner_ids(&[if_sg], &mut out);
    assert!(out.is_empty());
}

#[test]
fn recurses_into_nested_subgraphs() {
    let inner = function_subgraph_with("n_inner", Vec::new());
    let outer = function_subgraph_with("n_outer", vec![inner]);
    let mut out: HashSet<String> = HashSet::new();
    collect_wrapped_owner_ids(&[outer], &mut out);
    let mut got: Vec<String> = out.into_iter().collect();
    got.sort();
    assert_eq!(got, vec!["n_inner".to_string(), "n_outer".to_string()]);
}

#[test]
fn plain_top_level_nodes_are_skipped_without_traversal_error() {
    let elements = vec![
        VisualNode::from(BindingVisualNode {
            id: "n_a".to_string(),
            ..base_const_binding()
        })
        .into(),
        function_subgraph_with("n_b", Vec::new()),
    ];
    let mut out: HashSet<String> = HashSet::new();
    collect_wrapped_owner_ids(&elements, &mut out);
    let got: Vec<String> = out.into_iter().collect();
    assert_eq!(got, vec!["n_b".to_string()]);
}
