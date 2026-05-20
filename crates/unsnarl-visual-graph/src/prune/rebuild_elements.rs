//! Rebuild a `VisualElement` tree keeping only nodes whose id is in
//! the supplied keep-set; drop subgraphs whose descendants have all
//! been removed.
//!
//! Mirrors `ts/src/visual-graph/prune/rebuild-elements.ts`.

use std::collections::HashSet;

use crate::visual_element::VisualElement;
use crate::visual_subgraph::VisualSubgraph;

pub fn rebuild_elements(elements: &[VisualElement], keep: &HashSet<String>) -> Vec<VisualElement> {
    let mut result: Vec<VisualElement> = Vec::new();
    for item in elements {
        match item {
            VisualElement::Node(n) => {
                if keep.contains(n.id()) {
                    result.push(VisualElement::Node(n.clone()));
                }
            }
            VisualElement::Subgraph(sg) => {
                let children = rebuild_elements(sg.elements(), keep);
                // Subgraphs only survive when at least one descendant
                // survived. Keeping an empty subgraph -- even if it
                // appeared as an edge endpoint during BFS -- crashes
                // Mermaid's elk layout because the cluster has no
                // labels[0] for the renderer to size against. The
                // edges that pointed at this subgraph are filtered out
                // by the `survivors` check in the caller, so dropping
                // the cluster is consistent.
                if !children.is_empty() {
                    result.push(VisualElement::Subgraph(clone_with_children(sg, children)));
                }
            }
        }
    }
    result
}

fn clone_with_children(sg: &VisualSubgraph, children: Vec<VisualElement>) -> VisualSubgraph {
    match sg {
        VisualSubgraph::Owned(o) => {
            let mut clone = o.clone();
            clone.elements = children;
            VisualSubgraph::Owned(clone)
        }
        VisualSubgraph::Control(c) => {
            let mut clone = c.clone();
            clone.elements = children;
            VisualSubgraph::Control(clone)
        }
    }
}

#[cfg(test)]
#[path = "rebuild_elements_test.rs"]
mod rebuild_elements_test;
