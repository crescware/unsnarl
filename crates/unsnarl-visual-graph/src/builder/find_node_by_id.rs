//! Mirrors `ts/src/visual-graph/builder/find-node-by-id.ts`.
//!
//! Linear search through a (potentially nested) `VisualElement`
//! tree, returning a `&mut` to the matching `VisualNode` so
//! callers can patch `unused` / `end_line` after the fact.

use crate::visual_element::VisualElement;
use crate::visual_node::VisualNode;

pub fn find_node_by_id<'a>(
    elements: &'a mut [VisualElement],
    id: &str,
) -> Option<&'a mut VisualNode> {
    for e in elements {
        match e {
            VisualElement::Node(n) => {
                if n.id() == id {
                    return Some(n);
                }
            }
            VisualElement::Subgraph(s) => {
                if let Some(found) = find_node_by_id(s.elements_mut(), id) {
                    return Some(found);
                }
            }
        }
    }
    None
}
