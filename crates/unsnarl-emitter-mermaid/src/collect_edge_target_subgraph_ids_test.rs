use std::collections::HashSet;

use unsnarl_visual_graph::visual_edge::VisualEdge;

use super::collect_edge_target_subgraph_ids;

fn edge(from: &str, to: &str) -> VisualEdge {
    VisualEdge::new(from, to, "")
}

#[test]
fn empty_inputs_return_empty_set() {
    let out = collect_edge_target_subgraph_ids(&[], &HashSet::new());
    assert!(out.is_empty());
}

#[test]
fn ignores_edges_whose_target_is_not_a_subgraph_id() {
    let edges = vec![edge("n_a", "n_b"), edge("n_c", "n_d")];
    let subgraph_ids: HashSet<String> = HashSet::from(["s_x".to_string()]);
    let out = collect_edge_target_subgraph_ids(&edges, &subgraph_ids);
    assert!(out.is_empty());
}

#[test]
fn keeps_only_the_targets_that_are_subgraph_ids() {
    let edges = vec![edge("n_a", "s_x"), edge("n_b", "n_c"), edge("n_d", "s_y")];
    let subgraph_ids: HashSet<String> = HashSet::from(["s_x".to_string(), "s_y".to_string()]);
    let out = collect_edge_target_subgraph_ids(&edges, &subgraph_ids);
    assert_eq!(out, HashSet::from(["s_x".to_string(), "s_y".to_string()]));
}

#[test]
fn deduplicates_when_the_same_subgraph_is_targeted_multiple_times() {
    let edges = vec![edge("n_a", "s_x"), edge("n_b", "s_x"), edge("n_c", "s_x")];
    let subgraph_ids: HashSet<String> = HashSet::from(["s_x".to_string()]);
    let out = collect_edge_target_subgraph_ids(&edges, &subgraph_ids);
    assert_eq!(out, HashSet::from(["s_x".to_string()]));
}

#[test]
fn does_not_match_edges_whose_from_field_equals_a_subgraph_id() {
    // Only `to` counts; an edge *originating* from a subgraph (e.g.
    // a subgraph-rooted read) must not flip that subgraph into the
    // edge-target set.
    let edges = vec![edge("s_x", "n_a")];
    let subgraph_ids: HashSet<String> = HashSet::from(["s_x".to_string()]);
    let out = collect_edge_target_subgraph_ids(&edges, &subgraph_ids);
    assert!(out.is_empty());
}
