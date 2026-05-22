//! Sibling tests for [`attach_switch_discriminant_anchor`].

use unsnarl_ir::primitive::{SourceColumn, SourceLine, SourceOffset, Span};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::serialized_scope::{SerializedBlock, SerializedScope};
use unsnarl_oxc_parity::AstType;

use crate::builder::arena::{BuildArena, SubgraphIdx};
use crate::builder::state::{BuildState, LoopTestAnchorPosition};
use crate::builder::switch_discriminant_anchor::attach_switch_discriminant_anchor;
use crate::builder::testing::{base_serialized_scope, scope_id};
use crate::direction::Direction;
use crate::visual_element_type::SubgraphTypeTag;
use crate::visual_node::{SyntheticNodeKind, SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::{
    ControlExtras, ControlSubgraphKind, ControlVisualSubgraph, VisualSubgraph,
};

fn switch_subgraph_idx(arena: &mut BuildArena) -> SubgraphIdx {
    arena.push_subgraph(VisualSubgraph::Control(ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_switch".to_string(),
        line: 1,
        end_line: None,
        direction: Direction::RL,
        elements: Vec::new(),
        kind: ControlSubgraphKind::Switch,
        extras: ControlExtras::None {},
    }))
}

fn pending_synthetic_node<'a>(
    arena: &'a BuildArena,
    state: &BuildState,
    index: usize,
) -> &'a SyntheticVisualNode {
    let pending = state
        .pending_loop_test_anchors
        .get(index)
        .expect("expected a pending discriminant anchor");
    match arena.node(pending.node) {
        VisualNode::Synthetic(s) => s,
        VisualNode::Binding(_) => panic!("switch-discriminant anchor must be a synthetic node"),
    }
}

fn switch_scope(parent: Option<&str>, offset: u32, line: u32) -> SerializedScope {
    let mut scope = base_serialized_scope("switch_body");
    scope.r#type = ScopeType::Switch;
    scope.upper = parent.map(scope_id);
    scope.block = SerializedBlock {
        r#type: AstType::SwitchStatement,
        span: Span {
            line: SourceLine(line),
            column: SourceColumn(0),
            offset: SourceOffset(offset),
        },
        end_span: Span {
            line: SourceLine(line + 5),
            column: SourceColumn(1),
            offset: SourceOffset(offset + 86),
        },
    };
    scope
}

#[test]
fn switch_scope_pushes_discriminant_anchor_at_first_and_registers_offset() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = switch_subgraph_idx(&mut arena);
    let scope = switch_scope(Some("scope_0"), 34, 3);

    attach_switch_discriminant_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
    let pending = state.pending_loop_test_anchors[0];
    assert_eq!(pending.subgraph, sg);
    assert!(pending.position == LoopTestAnchorPosition::First);

    let node = pending_synthetic_node(&arena, &state, 0);
    assert_eq!(node.id, "switch_discriminant_scope_0_34");
    assert!(node.kind == SyntheticNodeKind::SyntheticSwitchStatementDiscriminant);
    assert_eq!(node.name, "switch-discriminant");
    assert_eq!(node.line, 3);
    assert_eq!(node.end_line, None);
    assert!(!node.is_jsx_element);
    assert!(!node.unused);

    assert_eq!(
        state
            .switch_discriminant_anchor_by_offset
            .get(&34)
            .map(String::as_str),
        Some("switch_discriminant_scope_0_34")
    );
}

#[test]
fn switch_scope_re_entering_same_offset_does_not_push_a_duplicate() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = switch_subgraph_idx(&mut arena);
    let scope = switch_scope(Some("scope_0"), 34, 3);

    attach_switch_discriminant_anchor(&mut arena, &mut state, &scope, sg);
    attach_switch_discriminant_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
}

#[test]
fn switch_scope_with_no_upper_falls_back_to_empty_parent_in_the_id() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = switch_subgraph_idx(&mut arena);
    let scope = switch_scope(None, 10, 1);

    attach_switch_discriminant_anchor(&mut arena, &mut state, &scope, sg);

    let node = pending_synthetic_node(&arena, &state, 0);
    assert_eq!(node.id, "switch_discriminant__10");
}

#[test]
fn non_switch_scope_block_is_a_no_op() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = switch_subgraph_idx(&mut arena);
    let mut scope = base_serialized_scope("block");
    scope.r#type = ScopeType::Block;

    attach_switch_discriminant_anchor(&mut arena, &mut state, &scope, sg);

    assert!(state.pending_loop_test_anchors.is_empty());
    assert!(state.switch_discriminant_anchor_by_offset.is_empty());
}

#[test]
fn non_switch_scope_for_is_a_no_op() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = switch_subgraph_idx(&mut arena);
    let mut scope = base_serialized_scope("for");
    scope.r#type = ScopeType::For;

    attach_switch_discriminant_anchor(&mut arena, &mut state, &scope, sg);

    assert!(state.pending_loop_test_anchors.is_empty());
}
