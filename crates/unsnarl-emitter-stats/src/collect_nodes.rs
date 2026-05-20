//! Walks an element tree and collects every [`VisualNode`] in
//! preorder.
//!
//! Mirrors `ts/src/emitter/stats/collect-nodes.ts`. The walk order
//! matches the TS recursion so the downstream line-only sort can
//! lean on a stable sort to preserve same-line source order.

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::VisualNode;

pub fn collect_nodes(elements: &[VisualElement]) -> Vec<&VisualNode> {
    let mut out: Vec<&VisualNode> = Vec::new();
    walk(elements, &mut out);
    out
}

fn walk<'a>(elements: &'a [VisualElement], out: &mut Vec<&'a VisualNode>) {
    for e in elements {
        match e {
            VisualElement::Node(n) => out.push(n),
            VisualElement::Subgraph(s) => walk(s.elements(), out),
        }
    }
}
