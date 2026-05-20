//! Walks an element tree and collects every Function subgraph's
//! owner-node id into the supplied set.
//!
//! Mirrors `ts/src/emitter/mermaid/collect-wrapped-owner-ids.ts`.
//! Owner ids of Function subgraphs are absorbed into a wrapper
//! subgraph alongside the body, so they must NOT also be emitted
//! as a sibling node at their declaring scope.

use std::collections::HashSet;

use unsnarl_visual_graph::subgraph_kind::SubgraphKind;
use unsnarl_visual_graph::visual_element::VisualElement;

pub fn collect_wrapped_owner_ids(elements: &[VisualElement], out: &mut HashSet<String>) {
    for e in elements {
        let VisualElement::Subgraph(s) = e else {
            continue;
        };
        if s.kind() == SubgraphKind::Function {
            if let Some(id) = s.owner_node_id() {
                out.insert(id.to_string());
            }
        }
        collect_wrapped_owner_ids(s.elements(), out);
    }
}
