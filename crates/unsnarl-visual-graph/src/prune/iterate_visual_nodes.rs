//! Walk a `VisualElement` tree and visit every node whose
//! [`NodeKind`](crate::node_kind::NodeKind) is a root candidate (per
//! [`is_root_candidate_kind`]).
//!
//! Mirrors `ts/src/visual-graph/prune/iterate-visual-nodes.ts`. The
//! TS file is a generator; the Rust port accepts a callback that the
//! walker invokes for each candidate node so callers can either
//! collect them or short-circuit without buying into an explicit
//! iterator-state machine.

use crate::prune::root_candidate_kinds::is_root_candidate_kind;
use crate::visual_element::VisualElement;
use crate::visual_node::VisualNode;

pub fn iterate_visual_nodes<'a>(
    elements: &'a [VisualElement],
    visit: &mut impl FnMut(&'a VisualNode),
) {
    for e in elements {
        match e {
            VisualElement::Node(n) => {
                if is_root_candidate_kind(n.kind()) {
                    visit(n);
                }
            }
            VisualElement::Subgraph(sg) => iterate_visual_nodes(sg.elements(), visit),
        }
    }
}

pub fn collect_root_candidates(elements: &[VisualElement]) -> Vec<&VisualNode> {
    let mut out: Vec<&VisualNode> = Vec::new();
    iterate_visual_nodes(elements, &mut |n| out.push(n));
    out
}

#[cfg(test)]
#[path = "iterate_visual_nodes_test.rs"]
mod iterate_visual_nodes_test;
