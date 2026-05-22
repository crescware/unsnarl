use std::collections::HashSet;

use unsnarl_visual_graph::visual_edge::VisualEdge;

use super::split_edges;
use crate::testing::base_edge;

fn edge(from: &str, to: &str) -> VisualEdge {
    VisualEdge {
        from: from.to_string(),
        to: to.to_string(),
        ..base_edge()
    }
}

fn set_of(ids: &[&str]) -> HashSet<String> {
    ids.iter().map(|s| s.to_string()).collect()
}

#[test]
fn routes_edges_whose_from_is_in_import_sources_to_imports() {
    let edges = vec![edge("mod_a", "n_x"), edge("n_x", "n_y")];
    let (body, imports) = split_edges(&edges, &set_of(&["mod_a"]));
    let imports_from: Vec<&str> = imports.iter().map(|e| e.from.as_str()).collect();
    let body_from: Vec<&str> = body.iter().map(|e| e.from.as_str()).collect();
    assert_eq!(imports_from, vec!["mod_a"]);
    assert_eq!(body_from, vec!["n_x"]);
}

#[test]
fn preserves_relative_order_within_each_bucket() {
    let edges = vec![edge("n_a", "n_b"), edge("mod_a", "n_a"), edge("n_c", "n_d")];
    let (body, imports) = split_edges(&edges, &set_of(&["mod_a"]));
    let body_pairs: Vec<String> = body
        .iter()
        .map(|e| format!("{}->{}", e.from, e.to))
        .collect();
    let import_pairs: Vec<String> = imports
        .iter()
        .map(|e| format!("{}->{}", e.from, e.to))
        .collect();
    assert_eq!(
        body_pairs,
        vec!["n_a->n_b".to_string(), "n_c->n_d".to_string()]
    );
    assert_eq!(import_pairs, vec!["mod_a->n_a".to_string()]);
}

#[test]
fn edges_that_target_an_import_source_go_to_body_not_imports() {
    let edges = vec![edge("n_x", "mod_a")];
    let (body, imports) = split_edges(&edges, &set_of(&["mod_a"]));
    assert!(imports.is_empty());
    assert_eq!(body.len(), 1);
}

#[test]
fn empty_edges_returns_two_empty_buckets() {
    let edges: Vec<VisualEdge> = Vec::new();
    let (body, imports) = split_edges(&edges, &HashSet::new());
    assert!(body.is_empty());
    assert!(imports.is_empty());
}
