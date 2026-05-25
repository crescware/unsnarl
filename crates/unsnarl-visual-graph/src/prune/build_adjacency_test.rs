use super::*;

fn edge(from: &str, to: &str, label: &str) -> VisualEdge {
    VisualEdge::new(from, to, label)
}

#[test]
fn empty_edges_produce_empty_maps() {
    let adj = build_adjacency(&[]);
    assert!(adj.out_edges.is_empty());
    assert!(adj.in_edges.is_empty());
}

#[test]
fn each_edge_contributes_to_both_out_and_in() {
    let adj = build_adjacency(&[edge("a", "b", "read")]);
    assert_eq!(adj.out_edges.get("a"), Some(&vec!["b".to_string()]));
    assert_eq!(adj.in_edges.get("b"), Some(&vec!["a".to_string()]));
}

#[test]
fn multiple_edges_from_the_same_source_append_in_source_order() {
    let adj = build_adjacency(&[edge("a", "b", "read"), edge("a", "c", "read")]);
    assert_eq!(
        adj.out_edges.get("a"),
        Some(&vec!["b".to_string(), "c".to_string()])
    );
}

#[test]
fn self_loops_record_both_directions_on_the_same_node() {
    let adj = build_adjacency(&[edge("a", "a", "read")]);
    assert_eq!(adj.out_edges.get("a"), Some(&vec!["a".to_string()]));
    assert_eq!(adj.in_edges.get("a"), Some(&vec!["a".to_string()]));
}
