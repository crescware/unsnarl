use std::collections::HashSet;

use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{SyntheticVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::{OwnedVisualSubgraph, VisualSubgraph};

use super::collect_subgraph_ids;

fn subgraph(id: &str, children: Vec<VisualElement>) -> VisualElement {
    let sg: VisualSubgraph =
        OwnedVisualSubgraph::class(id, 1, None, children, Direction::RL).into();
    sg.into()
}

fn node(id: &str) -> VisualElement {
    let n: VisualNode =
        SyntheticVisualNode::expression_statement(id.to_string(), "anything", 1).into();
    n.into()
}

#[test]
fn empty_input_leaves_set_unchanged() {
    let mut out: HashSet<String> = HashSet::new();
    out.insert("preexisting".to_string());
    collect_subgraph_ids(&[], &mut out);
    assert_eq!(out, HashSet::from(["preexisting".to_string()]));
}

#[test]
fn ignores_node_elements() {
    let mut out: HashSet<String> = HashSet::new();
    collect_subgraph_ids(&[node("expr_stmt_1")], &mut out);
    assert!(out.is_empty());
}

#[test]
fn collects_top_level_subgraph_ids() {
    let mut out: HashSet<String> = HashSet::new();
    collect_subgraph_ids(
        &[
            subgraph("s_a", Vec::new()),
            subgraph("s_b", Vec::new()),
            node("n_x"),
        ],
        &mut out,
    );
    assert_eq!(out, HashSet::from(["s_a".to_string(), "s_b".to_string()]));
}

#[test]
fn descends_into_nested_subgraphs() {
    let inner = subgraph("s_inner", Vec::new());
    let mid = subgraph("s_mid", vec![node("n_z"), inner]);
    let outer = subgraph("s_outer", vec![mid]);
    let mut out: HashSet<String> = HashSet::new();
    collect_subgraph_ids(&[outer], &mut out);
    assert_eq!(
        out,
        HashSet::from([
            "s_outer".to_string(),
            "s_mid".to_string(),
            "s_inner".to_string(),
        ])
    );
}

#[test]
fn merges_into_a_non_empty_output_set() {
    let mut out: HashSet<String> = HashSet::new();
    out.insert("seeded".to_string());
    collect_subgraph_ids(&[subgraph("s_a", Vec::new())], &mut out);
    assert_eq!(
        out,
        HashSet::from(["seeded".to_string(), "s_a".to_string()])
    );
}
