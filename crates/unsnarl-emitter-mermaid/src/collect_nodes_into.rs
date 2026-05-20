//! Walks an element tree and indexes every [`VisualNode`] by id
//! into the supplied map.
//!
//! Mirrors `ts/src/emitter/mermaid/collect-nodes-into.ts`.

use std::collections::HashMap;

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::VisualNode;

pub fn collect_nodes_into<'a>(
    elements: &'a [VisualElement],
    out: &mut HashMap<String, &'a VisualNode>,
) {
    for e in elements {
        match e {
            VisualElement::Node(n) => {
                out.insert(n.id().to_string(), n);
            }
            VisualElement::Subgraph(s) => {
                collect_nodes_into(s.elements(), out);
            }
        }
    }
}
