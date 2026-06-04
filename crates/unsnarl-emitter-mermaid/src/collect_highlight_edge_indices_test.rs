use std::collections::HashSet;

use unsnarl_visual_graph::boundary_edge_direction::{
    BoundaryEdgeDirectionIn, BoundaryEdgeDirectionOut,
};
use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;
use unsnarl_visual_graph::visual_edge::VisualEdge;

use super::collect_highlight_edge_indices;

fn e(from: &str, to: &str) -> VisualEdge {
    VisualEdge::new(from, to, "")
}

fn set_of(ids: &[&str]) -> HashSet<String> {
    ids.iter().map(|s| s.to_string()).collect()
}

// The original point-highlight cases: member == point, so the rule
// reduces to the historical either-endpoint behavior.

#[test]
fn returns_empty_when_no_ids_are_highlighted() {
    let body = [e("a", "b"), e("b", "c")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let empty = HashSet::new();
    let r = collect_highlight_edge_indices(&body_refs, &[], &[], &empty, &empty);
    assert!(r.is_empty());
}

#[test]
fn collects_body_edges_whose_endpoint_matches_a_point_id() {
    let body = [e("a", "b"), e("b", "c"), e("a", "c")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let s = set_of(&["b"]);
    let r = collect_highlight_edge_indices(&body_refs, &[], &[], &s, &s);
    assert_eq!(r, vec![0, 1]);
}

#[test]
fn collects_across_body_and_import_edges_with_global_indexing() {
    let body = [e("a", "b")];
    let imports = [e("mod", "a"), e("b", "sink")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let import_refs: Vec<&VisualEdge> = imports.iter().collect();
    let s = set_of(&["a"]);
    let r = collect_highlight_edge_indices(&body_refs, &import_refs, &[], &s, &s);
    assert_eq!(r, vec![0, 1]);
}

#[test]
fn collects_boundary_edges_by_inside_point_id() {
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
    let s = set_of(&["b"]);
    let r = collect_highlight_edge_indices(&[], &[], &boundary, &s, &s);
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
    let s = set_of(&["a"]);
    let r = collect_highlight_edge_indices(&body_refs, &import_refs, &boundary, &s, &s);
    assert_eq!(r, vec![0, 1, 2]);
}

// New POC #90 (judgment A) cases: a reachability set (empty point set)
// paints only edges internal to the set — no boundary bleed.

#[test]
fn reachability_set_paints_only_internal_edges_no_bleed() {
    // member = {a,b,c}; the entry edge x->a and the exit edge c->y each
    // have only one endpoint in the set, so they must NOT paint.
    let body = [e("x", "a"), e("a", "b"), e("b", "c"), e("c", "y")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let member = set_of(&["a", "b", "c"]);
    let point = HashSet::new();
    let r = collect_highlight_edge_indices(&body_refs, &[], &[], &member, &point);
    assert_eq!(r, vec![1, 2]);
}

#[test]
fn a_point_id_still_bleeds_one_edge_while_reachability_ids_do_not() {
    // member = {a,b,c}; only `a` is also a point id. The entry edge
    // x->a paints (a is a point id, radius-1), but c->y does not
    // (c is reachability-only).
    let body = [e("x", "a"), e("a", "b"), e("b", "c"), e("c", "y")];
    let body_refs: Vec<&VisualEdge> = body.iter().collect();
    let member = set_of(&["a", "b", "c"]);
    let point = set_of(&["a"]);
    let r = collect_highlight_edge_indices(&body_refs, &[], &[], &member, &point);
    assert_eq!(r, vec![0, 1, 2]);
}

#[test]
fn boundary_edges_never_paint_for_a_reachability_only_inside_id() {
    // The inside node is in member but not point: a boundary marker is
    // not part of a propagation path, so it stays unpainted.
    let boundary = vec![VisualBoundaryEdge::Out {
        inside: "a".to_string(),
        direction: BoundaryEdgeDirectionOut::Out,
    }];
    let member = set_of(&["a"]);
    let point = HashSet::new();
    let r = collect_highlight_edge_indices(&[], &[], &boundary, &member, &point);
    assert!(r.is_empty());
}
