//! Walk a `VisualElement` tree and visit every [`VisualSubgraph`] in
//! pre-order.

use crate::visual_element::VisualElement;
use crate::visual_subgraph::VisualSubgraph;

pub fn iterate_visual_subgraphs<'a>(
    elements: &'a [VisualElement],
    visit: &mut impl FnMut(&'a VisualSubgraph),
) {
    for e in elements {
        if let VisualElement::Subgraph(sg) = e {
            visit(sg);
            iterate_visual_subgraphs(sg.elements(), visit);
        }
    }
}

pub fn collect_subgraphs(elements: &[VisualElement]) -> Vec<&VisualSubgraph> {
    let mut out: Vec<&VisualSubgraph> = Vec::new();
    iterate_visual_subgraphs(elements, &mut |sg| out.push(sg));
    out
}

#[cfg(test)]
#[path = "iterate_visual_subgraphs_test.rs"]
mod iterate_visual_subgraphs_test;
