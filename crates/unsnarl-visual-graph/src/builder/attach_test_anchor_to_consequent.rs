//! Place an IfStatement's test anchor at the head of the consequent
//! subgraph it gates.

use unsnarl_ir::primitive::{SourceIndex, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::SerializedScope;

use crate::visual_node::{SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, Container, ElementHandle};
use super::if_test_node_id::if_test_node_id;
use super::line_for_offset::line_for_offset;
use super::state::BuildState;

fn make_if_test_anchor(
    id: String,
    offset: Utf16CodeUnitOffset,
    source_index: &SourceIndex<'_>,
) -> VisualNode {
    SyntheticVisualNode::if_statement_test(id, line_for_offset(source_index, offset)).into()
}

/// Place the IfStatement's test anchor at the head of the consequent
/// subgraph it gates. `else` is the fallback path and carries no
/// test. A consequent that collapsed past the depth threshold builds
/// no subgraph (`should_subgraph` is always true for a non-collapsed
/// if-branch Block scope, so `subgraph_by_scope` misses only when the
/// scope collapsed); with nowhere to host the anchor it is dropped
/// rather than leaking into the surrounding container.
pub fn attach_test_anchor_to_consequent(
    arena: &mut BuildArena,
    state: &mut BuildState,
    consequent: &SerializedScope,
    offset: Utf16CodeUnitOffset,
    source_index: &SourceIndex<'_>,
) {
    if state.if_test_anchor_by_offset.contains_key(&offset.0) {
        return;
    }
    let Some(body_sg) = state.subgraph_by_scope.get(consequent.id.value()).copied() else {
        return;
    };
    let parent_id = consequent.upper.as_ref().map(|s| s.value()).unwrap_or("");
    let id = if_test_node_id(parent_id, offset.0);
    let node = make_if_test_anchor(id.clone(), offset, source_index);
    let idx = arena.push_node(node);
    arena.prepend_child(Container::Subgraph(body_sg), ElementHandle::Node(idx));
    state.if_test_anchor_by_offset.insert(offset.0, id);
}
