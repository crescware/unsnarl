//! Stage a loop-test anchor node for a body subgraph. The actual
//! placement is deferred: the anchor node is created and parked in
//! `state.pending_loop_test_anchors`, then placed at the very end
//! of the build once every other element has been added so the
//! anchor sits at the correct edge (unshift for for/while, push
//! for do-while) of the rendered element list.
//!
//! Handles three cases that all share the same staging pattern:
//!
//! - `For` scope (block.type ∈ ForStatement / ForInStatement /
//!   ForOfStatement): anchor kind picked by AST type, placed
//!   `First`. Offset key is `scope.block.span.offset`.
//! - Block scope whose `blockContext.parentType` is
//!   `WhileStatement`: `SyntheticWhileStatementTest`, placed
//!   `First`. Offset key is `blockContext.parentSpanOffset`.
//! - Block scope whose `blockContext.parentType` is
//!   `DoWhileStatement`: `SyntheticDoWhileStatementTest`, placed
//!   `Last`. Offset key is `blockContext.parentSpanOffset`; the
//!   anchor's `line` reads from `scope.block.endSpan.line` so the
//!   anchor sits below the body in the rendered diagram.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, SubgraphIdx};
use super::loop_test_node_id::{do_while_test_node_id, for_test_node_id, while_test_node_id};
use super::state::{BuildState, LoopTestAnchorPosition, PendingLoopTestAnchor};

pub fn attach_loop_test_anchor(
    arena: &mut BuildArena,
    state: &mut BuildState,
    scope: &SerializedScope,
    sg: SubgraphIdx,
) {
    if scope.r#type == ScopeType::For {
        let offset = scope.block.span.offset.0;
        if state.for_test_anchor_by_offset.contains_key(&offset) {
            return;
        }
        let kind = match scope.block.r#type {
            AstType::ForStatement => SyntheticNodeKind::SyntheticForStatementHeader,
            AstType::ForInStatement => SyntheticNodeKind::SyntheticForInStatementHeader,
            _ => SyntheticNodeKind::SyntheticForOfStatementHeader,
        };
        let parent_id = scope.upper.as_ref().map(|s| s.value()).unwrap_or("");
        let id = for_test_node_id(parent_id, offset);
        let node = VisualNode::Synthetic(SyntheticVisualNode {
            r#type: NodeTypeTag::Node,
            id: id.clone(),
            kind,
            name: "for-test".to_string(),
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
        state.for_test_anchor_by_offset.insert(offset, id);
        return;
    }
    if scope.r#type != ScopeType::Block {
        return;
    }
    let Some(ctx) = scope.block_context.as_ref() else {
        return;
    };
    if ctx.key() != "body" {
        return;
    }
    match ctx.parent_type() {
        AstType::WhileStatement => {
            let offset = ctx.parent_span_offset().0;
            if state.while_test_anchor_by_offset.contains_key(&offset) {
                return;
            }
            let parent_id = scope.upper.as_ref().map(|s| s.value()).unwrap_or("");
            let id = while_test_node_id(parent_id, offset);
            let node = VisualNode::Synthetic(SyntheticVisualNode {
                r#type: NodeTypeTag::Node,
                id: id.clone(),
                kind: SyntheticNodeKind::SyntheticWhileStatementTest,
                name: "while-test".to_string(),
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
            state.while_test_anchor_by_offset.insert(offset, id);
        }
        AstType::DoWhileStatement => {
            let offset = ctx.parent_span_offset().0;
            if state.do_while_test_anchor_by_offset.contains_key(&offset) {
                return;
            }
            let parent_id = scope.upper.as_ref().map(|s| s.value()).unwrap_or("");
            let id = do_while_test_node_id(parent_id, offset);
            let node = VisualNode::Synthetic(SyntheticVisualNode {
                r#type: NodeTypeTag::Node,
                id: id.clone(),
                kind: SyntheticNodeKind::SyntheticDoWhileStatementTest,
                name: "do-while-test".to_string(),
                line: scope.block.end_span.line.0,
                end_line: None,
                is_jsx_element: false,
                unused: false,
                extras: SyntheticExtras::None {},
            });
            let node_idx = arena.push_node(node);
            state.pending_loop_test_anchors.push(PendingLoopTestAnchor {
                subgraph: sg,
                node: node_idx,
                position: LoopTestAnchorPosition::Last,
            });
            state.do_while_test_anchor_by_offset.insert(offset, id);
        }
        _ => {}
    }
}

#[cfg(test)]
#[path = "loop_test_anchor_test.rs"]
mod loop_test_anchor_test;
