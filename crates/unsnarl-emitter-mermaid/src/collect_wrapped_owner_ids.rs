//! Walks an element tree and collects every owner-carrying
//! subgraph's owner-node id into the supplied set.
//!
//! A subgraph carries an owner node only when it is a named `Function`
//! (owner = the FunctionName node). That owner node is absorbed into a
//! wrapper subgraph alongside the body, so it must NOT also be emitted
//! as a sibling node at its declaring scope. Subgraphs without an owner
//! contribute nothing here.

use std::collections::HashSet;

use unsnarl_visual_graph::visual_element::VisualElement;

pub fn collect_wrapped_owner_ids(elements: &[VisualElement], out: &mut HashSet<String>) {
    for e in elements {
        let VisualElement::Subgraph(s) = e else {
            continue;
        };
        if let Some(id) = s.owner_node_id() {
            out.insert(id.to_string());
        }
        collect_wrapped_owner_ids(s.elements(), out);
    }
}

#[cfg(test)]
#[path = "collect_wrapped_owner_ids_test.rs"]
mod collect_wrapped_owner_ids_test;
