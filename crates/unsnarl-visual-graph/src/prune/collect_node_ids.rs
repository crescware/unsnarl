//! Collect every node id (subgraph ids excluded) in document order.
//!
//! Mirrors `ts/src/visual-graph/prune/collect-node-ids.ts`.

use crate::visual_element::VisualElement;

pub fn collect_node_ids(elements: &[VisualElement]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    walk(elements, &mut out);
    out
}

fn walk(items: &[VisualElement], out: &mut Vec<String>) {
    for item in items {
        match item {
            VisualElement::Node(n) => out.push(n.id().to_string()),
            VisualElement::Subgraph(sg) => walk(sg.elements(), out),
        }
    }
}

#[cfg(test)]
#[path = "collect_node_ids_test.rs"]
mod collect_node_ids_test;
