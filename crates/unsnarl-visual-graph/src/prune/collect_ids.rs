//! Collect every node id and subgraph id from an element tree.

use std::collections::HashSet;

use crate::visual_element::VisualElement;

pub fn collect_ids(elements: &[VisualElement]) -> HashSet<String> {
    let mut ids: HashSet<String> = HashSet::new();
    walk(elements, &mut ids);
    ids
}

fn walk(items: &[VisualElement], ids: &mut HashSet<String>) {
    for item in items {
        ids.insert(item.id().to_string());
        if let VisualElement::Subgraph(sg) = item {
            walk(sg.elements(), ids);
        }
    }
}

#[cfg(test)]
#[path = "collect_ids_test.rs"]
mod collect_ids_test;
