//! Appends one `from --|label|--> to` line per edge.
//!
//! Mirrors `ts/src/emitter/mermaid/push-edge-lines.ts`. The TS form
//! takes the node map as optional and falls back to `false` when
//! omitted; the Rust port keeps the same shape with
//! `Option<&HashMap<...>>` so callers that lack the map still go
//! through the solid-arrow path.

use std::collections::HashMap;

use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_edge::VisualEdge;
use unsnarl_visual_graph::visual_node::VisualNode;

pub fn push_edge_lines<'a, I>(
    edges: I,
    lines: &mut Vec<String>,
    node_map: Option<&HashMap<String, &VisualNode>>,
) where
    I: IntoIterator<Item = &'a VisualEdge>,
{
    for e in edges {
        let arrow = if touches_beyond_depth(e, node_map) {
            "-.->"
        } else {
            "-->"
        };
        lines.push(format!("  {} {arrow}|{}| {}", e.from, e.label, e.to));
    }
}

/// Edges that point at (or away from) a `SyntheticBeyondDepth`
/// `((...))` stub render with a dashed arrow so the boundary into
/// the hidden subtree is visually consistent with the pruning
/// boundary edges.
fn touches_beyond_depth(e: &VisualEdge, node_map: Option<&HashMap<String, &VisualNode>>) -> bool {
    let Some(map) = node_map else {
        return false;
    };
    let from_is_stub = map
        .get(&e.from)
        .map(|n| n.kind() == NodeKind::SyntheticBeyondDepth)
        .unwrap_or(false);
    let to_is_stub = map
        .get(&e.to)
        .map(|n| n.kind() == NodeKind::SyntheticBeyondDepth)
        .unwrap_or(false);
    from_is_stub || to_is_stub
}

#[cfg(test)]
#[path = "push_edge_lines_test.rs"]
mod push_edge_lines_test;
