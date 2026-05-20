//! Mirrors `ts/src/visual-graph/builder/switch-discriminant-anchor.ts`.
//!
//! Stage the discriminant anchor for a switch scope. Like the
//! loop-test anchor the actual placement is deferred to the end of
//! the build via `state.pending_loop_test_anchors`; the
//! discriminant always lands at the head of the switch subgraph's
//! element list (`First`).

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;

use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, SubgraphIdx};
use super::sanitize::sanitize;
use super::state::{BuildState, LoopTestAnchorPosition, PendingLoopTestAnchor};

fn switch_discriminant_node_id(parent_scope_id: &str, offset: u32) -> String {
    format!("switch_discriminant_{}_{offset}", sanitize(parent_scope_id))
}

pub fn attach_switch_discriminant_anchor(
    arena: &mut BuildArena,
    state: &mut BuildState,
    scope: &SerializedScope,
    sg: SubgraphIdx,
) {
    if scope.r#type != ScopeType::Switch {
        return;
    }
    let offset = scope.block.span.offset.0;
    if state
        .switch_discriminant_anchor_by_offset
        .contains_key(&offset)
    {
        return;
    }
    let parent_id = scope.upper.as_ref().map(|s| s.value()).unwrap_or("");
    let id = switch_discriminant_node_id(parent_id, offset);
    let node = VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.clone(),
        kind: SyntheticNodeKind::SyntheticSwitchStatementDiscriminant,
        name: "switch-discriminant".to_string(),
        line: scope.block.span.line.0,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    });
    let node_idx = arena.push_node(node);
    state.pending_loop_test_anchors.push(PendingLoopTestAnchor {
        subgraph: sg,
        node: node_idx,
        position: LoopTestAnchorPosition::First,
    });
    state
        .switch_discriminant_anchor_by_offset
        .insert(offset, id);
}
