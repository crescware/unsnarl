//! Walks an element tree and collects the id of every
//! [`VisualSubgraph`] into the supplied set.
//!
//! Used by [`crate::collect_edge_target_subgraph_ids`] to decide
//! which subgraphs deserve the visible-border treatment when an
//! ordinary edge terminates on the subgraph itself rather than on
//! one of its child nodes.

use std::collections::HashSet;

use unsnarl_visual_graph::visual_element::VisualElement;

pub fn collect_subgraph_ids(elements: &[VisualElement], out: &mut HashSet<String>) {
    for e in elements {
        if let VisualElement::Subgraph(s) = e {
            out.insert(s.id().to_string());
            collect_subgraph_ids(s.elements(), out);
        }
    }
}

#[cfg(test)]
#[path = "collect_subgraph_ids_test.rs"]
mod collect_subgraph_ids_test;
