use std::collections::HashSet;

use unsnarl_visual_graph::boundary_edge_direction::{
    BoundaryEdgeDirectionIn, BoundaryEdgeDirectionOut,
};
use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;
use unsnarl_visual_graph::visual_edge::VisualEdge;

use super::collect_highlight_edge_indices;

fn e(from: &str, to: &str) -> VisualEdge {
    VisualEdge {
        from: from.to_string(),
        to: to.to_string(),
        label: String::new(),
    }
}

fn set_of(ids: &[&str]) -> HashSet<String> {
    ids.iter().map(|s| s.to_string()).collect()
}

#[test]
fn returns_empty_when_no_ids_are_highlighted() {
    let body = [e("a", "b"), e("b", "c")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let r = collect_highlight_edge_indices(&body_refs, &[], &[], &HashSet::new());
    assert!(r.is_empty());
}

#[test]
fn collects_body_edges_whose_endpoint_matches() {
    let body = [e("a", "b"), e("b", "c"), e("a", "c")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let r = collect_highlight_edge_indices(&body_refs, &[], &[], &set_of(&["b"]));
    assert_eq!(r, vec![0, 1]);
}

#[test]
fn collects_across_body_and_import_edges_with_global_indexing() {
    let body = [e("a", "b")];
    let imports = [e("mod", "a"), e("b", "sink")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let import_refs: Vec<&VisualEdge> = imports.iter().collect();
    let r = collect_highlight_edge_indices(&body_refs, &import_refs, &[], &set_of(&["a"]));
    assert_eq!(r, vec![0, 1]);
}

#[test]
fn collects_boundary_edges_by_inside_id() {
    let boundary = vec![
        VisualBoundaryEdge::Out {
            inside: "a".to_string(),
            direction: BoundaryEdgeDirectionOut::Out,
        },
        VisualBoundaryEdge::In {
            inside: "b".to_string(),
            direction: BoundaryEdgeDirectionIn::In,
            label: "read".to_string(),
        },
    ];
    let r = collect_highlight_edge_indices(&[], &[], &boundary, &set_of(&["b"]));
    assert_eq!(r, vec![1]);
}

#[test]
fn indexing_across_all_three_buckets_is_contiguous() {
    let body = [e("a", "b")];
    let imports = [e("a", "c")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let import_refs: Vec<&VisualEdge> = imports.iter().collect();
    let boundary = vec![VisualBoundaryEdge::Out {
        inside: "a".to_string(),
        direction: BoundaryEdgeDirectionOut::Out,
    }];
    let r = collect_highlight_edge_indices(&body_refs, &import_refs, &boundary, &set_of(&["a"]));
    assert_eq!(r, vec![0, 1, 2]);
}
