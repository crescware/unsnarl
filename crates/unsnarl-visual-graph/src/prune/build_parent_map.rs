//! Build a child -> parent-subgraph id map by walking the element
//! tree.
//!
//! Mirrors `ts/src/visual-graph/prune/build-parent-map.ts`.

use std::collections::HashMap;

use crate::visual_element::VisualElement;

pub fn build_parent_map(elements: &[VisualElement]) -> HashMap<String, String> {
    let mut parent: HashMap<String, String> = HashMap::new();
    walk(elements, None, &mut parent);
    parent
}

fn walk(items: &[VisualElement], parent_id: Option<&str>, parent: &mut HashMap<String, String>) {
    for item in items {
        if let Some(pid) = parent_id {
            parent.insert(item.id().to_string(), pid.to_string());
        }
        if let VisualElement::Subgraph(sg) = item {
            walk(sg.elements(), Some(sg.id()), parent);
        }
    }
}

#[cfg(test)]
#[path = "build_parent_map_test.rs"]
mod build_parent_map_test;
