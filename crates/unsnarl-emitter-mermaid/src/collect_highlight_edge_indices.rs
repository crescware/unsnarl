//! Walks body / import / boundary edges in the order the renderer
//! emits them and returns the indices of edges to paint.
//!
//! Two membership sets drive the decision (POC #90, judgment A):
//! - `member`: every highlighted id. A body/import edge whose BOTH
//!   endpoints are in `member` is an internal edge of a path/direction
//!   set and paints.
//! - `point`: the point (`Single`) subset. A body/import edge with
//!   EITHER endpoint in `point` paints, preserving the radius-1
//!   adjacency the original point highlight has always drawn.
//!
//! `point` is a subset of `member`, so a pure point highlight (where
//! `member == point`) reduces to the historical either-endpoint rule.
//! Boundary edges are pruning markers and never part of a reachability
//! path, so they paint only for a point hit on the inside node.

use std::collections::HashSet;

use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;
use unsnarl_visual_graph::visual_edge::VisualEdge;

pub fn collect_highlight_edge_indices(
    body_edges: &[&VisualEdge],
    import_edges: &[&VisualEdge],
    boundary_edges: &[VisualBoundaryEdge],
    member: &HashSet<String>,
    point: &HashSet<String>,
) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    if member.is_empty() {
        return out;
    }
    let paints = |e: &VisualEdge| -> bool {
        (member.contains(&e.from) && member.contains(&e.to))
            || point.contains(&e.from)
            || point.contains(&e.to)
    };
    let mut i = 0usize;
    for e in body_edges {
        if paints(e) {
            out.push(i);
        }
        i += 1;
    }
    for e in import_edges {
        if paints(e) {
            out.push(i);
        }
        i += 1;
    }
    for be in boundary_edges {
        let inside = match be {
            VisualBoundaryEdge::Out { inside, .. } | VisualBoundaryEdge::In { inside, .. } => {
                inside
            }
        };
        if point.contains(inside) {
            out.push(i);
        }
        i += 1;
    }
    out
}

#[cfg(test)]
#[path = "collect_highlight_edge_indices_test.rs"]
mod collect_highlight_edge_indices_test;
