//! Return (creating on first call) the single BeyondDepth stub
//! associated with a visible ancestor subgraph. Multiple anonymous
//! collapsed children of the same surviving outer container funnel
//! into the same `((...))` placeholder so the rendered graph shows
//! one boundary marker per visible parent instead of one per hidden
//! child.

use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::sanitize::sanitize;
use super::state::BuildState;

pub fn ensure_beyond_depth_stub(
    arena: &mut BuildArena,
    state: &mut BuildState,
    parent: SubgraphIdx,
) -> String {
    let parent_id = arena.subgraph(parent).descriptor.id().to_string();
    if let Some(existing) = state.beyond_depth_stub_by_parent.get(&parent_id) {
        return existing.clone();
    }
    let stub_id = format!("beyond_depth_{}", sanitize(&parent_id));
    let line = arena.subgraph(parent).descriptor.line();
    let end_line = arena.subgraph(parent).descriptor.end_line();
    let node = VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: stub_id.clone(),
        kind: SyntheticNodeKind::SyntheticBeyondDepth,
        name: "...".to_string(),
        line,
        end_line,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    });
    let node_idx = arena.push_node(node);
    arena.append_child(Container::Subgraph(parent), ElementHandle::Node(node_idx));
    state
        .beyond_depth_stub_by_parent
        .insert(parent_id, stub_id.clone());
    stub_id
}
