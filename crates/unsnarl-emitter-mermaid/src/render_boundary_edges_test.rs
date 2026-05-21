//! Mirrors `ts/src/emitter/mermaid/render-boundary-edges.test.ts`.

use unsnarl_visual_graph::boundary_edge_direction::{
    BoundaryEdgeDirectionIn, BoundaryEdgeDirectionOut,
};
use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;

use super::render_boundary_edges;
use crate::testing::base_graph;

#[test]
fn does_nothing_when_boundary_edges_is_empty() {
    let mut lines: Vec<String> = Vec::new();
    let mut stub_ids: Vec<String> = Vec::new();
    render_boundary_edges(&base_graph(), &mut lines, &mut stub_ids);
    assert!(lines.is_empty());
    assert!(stub_ids.is_empty());

    let g = base_graph();
    render_boundary_edges(&g, &mut lines, &mut stub_ids);
    assert!(lines.is_empty());
    assert!(stub_ids.is_empty());
}

#[test]
fn emits_an_unlabeled_dashed_arrow_for_direction_out() {
    let mut g = base_graph();
    g.boundary_edges = vec![VisualBoundaryEdge::Out {
        inside: "n_x".to_string(),
        direction: BoundaryEdgeDirectionOut::Out,
    }];
    let mut lines: Vec<String> = Vec::new();
    let mut stub_ids: Vec<String> = Vec::new();
    render_boundary_edges(&g, &mut lines, &mut stub_ids);
    assert_eq!(stub_ids, vec!["boundary_stub_1".to_string()]);
    assert!(lines.contains(&"  boundary_stub_1((...))".to_string()));
    assert!(lines.contains(&"  n_x -.-> boundary_stub_1".to_string()));
}

#[test]
fn emits_a_labeled_dashed_arrow_for_direction_in() {
    let mut g = base_graph();
    g.boundary_edges = vec![VisualBoundaryEdge::In {
        inside: "n_x".to_string(),
        direction: BoundaryEdgeDirectionIn::In,
        label: "read".to_string(),
    }];
    let mut lines: Vec<String> = Vec::new();
    let mut stub_ids: Vec<String> = Vec::new();
    render_boundary_edges(&g, &mut lines, &mut stub_ids);
    assert!(lines.contains(&"  boundary_stub_1((...))".to_string()));
    assert!(lines.contains(&"  boundary_stub_1 -.->|read| n_x".to_string()));
}

#[test]
fn assigns_sequential_stub_ids_and_appends_them_to_stub_ids() {
    let mut g = base_graph();
    g.boundary_edges = vec![
        VisualBoundaryEdge::Out {
            inside: "a".to_string(),
            direction: BoundaryEdgeDirectionOut::Out,
        },
        VisualBoundaryEdge::In {
            inside: "b".to_string(),
            direction: BoundaryEdgeDirectionIn::In,
            label: "write".to_string(),
        },
        VisualBoundaryEdge::Out {
            inside: "c".to_string(),
            direction: BoundaryEdgeDirectionOut::Out,
        },
    ];
    let mut lines: Vec<String> = Vec::new();
    let mut stub_ids: Vec<String> = Vec::new();
    render_boundary_edges(&g, &mut lines, &mut stub_ids);
    assert_eq!(
        stub_ids,
        vec![
            "boundary_stub_1".to_string(),
            "boundary_stub_2".to_string(),
            "boundary_stub_3".to_string(),
        ]
    );
}
