//! Place a ternary's test diamond at the head of its consequent
//! (`? then`) subgraph — the expression-level analogue of
//! [`super::attach_test_anchor_to_consequent`]. The `alternate`
//! (`: else`) arm is the fallback path and carries no test.

use unsnarl_ir::primitive::{SourceIndex, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::SerializedScope;

use crate::visual_node::{SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, Container, ElementHandle};
use super::conditional_test_node_id::conditional_test_node_id;
use super::line_for_offset::line_for_offset;
use super::state::BuildState;

fn make_conditional_test_anchor(
    id: String,
    offset: Utf16CodeUnitOffset,
    source_index: &SourceIndex<'_>,
) -> VisualNode {
    SyntheticVisualNode::conditional_test(id, line_for_offset(source_index, offset)).into()
}

/// Place the ternary's test diamond at the head of the consequent arm
/// it gates. `build_conditional_group` calls this once per ternary,
/// immediately after building the consequent's subgraph; a consequent
/// arm is a control subgraph that never depth-collapses, so its subgraph
/// is always registered by the time we look it up.
pub fn attach_conditional_test_anchor(
    arena: &mut BuildArena,
    state: &mut BuildState,
    consequent: &SerializedScope,
    offset: Utf16CodeUnitOffset,
    source_index: &SourceIndex<'_>,
) {
    let body_sg = state
        .subgraph_by_scope
        .get(consequent.id.value())
        .copied()
        .expect("consequent arm subgraph must exist: a never-collapsed control subgraph just built by build_conditional_group");
    let parent_id = consequent.upper.as_ref().map(|s| s.value()).unwrap_or("");
    let id = conditional_test_node_id(parent_id, offset.0);
    let node = make_conditional_test_anchor(id.clone(), offset, source_index);
    let idx = arena.push_node(node);
    arena.prepend_child(Container::Subgraph(body_sg), ElementHandle::Node(idx));
    state.conditional_test_anchor_by_offset.insert(offset.0, id);
}
