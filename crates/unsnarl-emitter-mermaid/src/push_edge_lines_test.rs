use unsnarl_visual_graph::visual_edge::VisualEdge;

use super::push_edge_lines;

fn edge(from: &str, to: &str, label: &str) -> VisualEdge {
    VisualEdge::new(from, to, label)
}

#[test]
fn formats_each_edge_as_arrow_with_label() {
    let edges = vec![edge("a", "b", "read"), edge("c", "d", "write")];
    let mut lines: Vec<String> = Vec::new();
    push_edge_lines(&edges, &mut lines, None);
    assert_eq!(
        lines,
        vec![
            "  a -->|read| b".to_string(),
            "  c -->|write| d".to_string(),
        ]
    );
}

#[test]
fn preserves_input_order_in_the_appended_lines() {
    let edges = vec![edge("x", "y", "set")];
    let mut lines: Vec<String> = vec!["existing".to_string()];
    push_edge_lines(&edges, &mut lines, None);
    assert_eq!(
        lines,
        vec!["existing".to_string(), "  x -->|set| y".to_string()]
    );
}

#[test]
fn empty_input_pushes_nothing() {
    let edges: Vec<VisualEdge> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    push_edge_lines(&edges, &mut lines, None);
    assert!(lines.is_empty());
}
