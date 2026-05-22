//! Sibling tests for [`push_edge`].
//! The Rust
//! signature takes `(emitted, edges, from, label, to)` instead of
//! the TS `(state, ...)` shape, so each test owns the two
//! deduplication structures directly.

use std::collections::HashSet;

use super::push_edge;
use crate::visual_edge::VisualEdge;

fn run(steps: &[(&str, &str, &str)]) -> (HashSet<String>, Vec<VisualEdge>) {
    let mut emitted: HashSet<String> = HashSet::new();
    let mut edges: Vec<VisualEdge> = Vec::new();
    for (from, label, to) in steps {
        push_edge(&mut emitted, &mut edges, from, label, to);
    }
    (emitted, edges)
}

#[test]
fn appends_a_new_edge_and_records_the_dedup_key() {
    let (emitted, edges) = run(&[("a", "read", "b")]);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].from, "a");
    assert_eq!(edges[0].to, "b");
    assert_eq!(edges[0].label, "read");
    assert!(emitted.contains("a -->|read| b"));
}

#[test]
fn ignores_a_second_call_that_exactly_matches_a_prior_edge() {
    let (_, edges) = run(&[("a", "read", "b"), ("a", "read", "b")]);
    assert_eq!(edges.len(), 1);
}

#[test]
fn different_label_keeps_both_edges() {
    let (_, edges) = run(&[("a", "read", "b"), ("a", "write", "b")]);
    assert_eq!(edges.len(), 2);
}

#[test]
fn different_from_keeps_both_edges() {
    let (_, edges) = run(&[("a", "read", "b"), ("x", "read", "b")]);
    assert_eq!(edges.len(), 2);
}

#[test]
fn different_to_keeps_both_edges() {
    let (_, edges) = run(&[("a", "read", "b"), ("a", "read", "z")]);
    assert_eq!(edges.len(), 2);
}

#[test]
fn preserves_insertion_order_across_distinct_edges() {
    let (_, edges) = run(&[("a", "read", "b"), ("a", "write", "b"), ("c", "read", "d")]);
    let signature: Vec<String> = edges
        .iter()
        .map(|e| format!("{}-{}-{}", e.from, e.label, e.to))
        .collect();
    assert_eq!(signature, vec!["a-read-b", "a-write-b", "c-read-d"]);
}
