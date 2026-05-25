//! Sibling tests for [`attach_loop_test_anchor`]. The signature
//! takes `(arena, state, scope, sg)` because the anchor body is
//! interned into the arena; assertions therefore read the node back
//! through `arena.node()`.

use unsnarl_ir::primitive::{SourceColumn, SourceLine, Span, Utf16CodeUnitOffset};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::serialized_scope::{SerializedBlock, SerializedScope};
use unsnarl_oxc_parity::AstType;

use crate::builder::arena::{BuildArena, SubgraphIdx};
use crate::builder::loop_test_anchor::attach_loop_test_anchor;
use crate::builder::state::{BuildState, LoopTestAnchorPosition};
use crate::builder::testing::{base_serialized_scope, other_block_context, scope_id};
use crate::direction::Direction;
use crate::visual_node::{SyntheticNodeKind, SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::{ControlSubgraphKind, ControlVisualSubgraph};

fn body_subgraph_idx(arena: &mut BuildArena, kind: ControlSubgraphKind) -> SubgraphIdx {
    let sg = match kind {
        ControlSubgraphKind::For => {
            ControlVisualSubgraph::for_subgraph("s_body", 1, Vec::new(), Direction::RL)
        }
        ControlSubgraphKind::While => {
            ControlVisualSubgraph::while_subgraph("s_body", 1, Vec::new(), Direction::RL)
        }
        ControlSubgraphKind::DoWhile => {
            ControlVisualSubgraph::do_while("s_body", 1, Vec::new(), Direction::RL)
        }
        _ => ControlVisualSubgraph::block("s_body", 1, Vec::new(), Direction::RL),
    };
    arena.push_subgraph(sg.into())
}

fn pending_synthetic_node<'a>(
    arena: &'a BuildArena,
    state: &BuildState,
    index: usize,
) -> &'a SyntheticVisualNode {
    let pending = state
        .pending_loop_test_anchors
        .get(index)
        .expect("expected a pending loop-test anchor");
    match arena.node(pending.node) {
        VisualNode::Synthetic(s) => s,
        VisualNode::Binding(_) => panic!("loop-test anchor must be a synthetic node"),
    }
}

fn for_scope_at(parent: Option<&str>, offset: u32, line: u32) -> SerializedScope {
    let mut scope = base_serialized_scope("for_body");
    scope.r#type = ScopeType::For;
    scope.upper = parent.map(scope_id);
    scope.block = SerializedBlock {
        r#type: AstType::ForStatement,
        span: Span {
            line: SourceLine(line),
            column: SourceColumn(0),
            offset: Utf16CodeUnitOffset(offset),
        },
        end_span: Span {
            line: SourceLine(line + 2),
            column: SourceColumn(1),
            offset: Utf16CodeUnitOffset(offset + 46),
        },
    };
    scope
}

#[test]
fn for_scope_pushes_for_test_anchor_with_name_position_first_and_registers_offset() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::For);
    let scope = for_scope_at(Some("scope_0"), 34, 3);

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
    let pending = state.pending_loop_test_anchors[0];
    assert_eq!(pending.subgraph, sg);
    assert!(pending.position == LoopTestAnchorPosition::First);

    let node = pending_synthetic_node(&arena, &state, 0);
    assert_eq!(node.id, "for_test_scope_0_34");
    assert!(node.kind == SyntheticNodeKind::SyntheticForStatementHeader);
    assert_eq!(node.name, "for-test");
    assert_eq!(node.line, 3);
    assert_eq!(node.end_line, None);
    assert!(!node.is_jsx_element);
    assert!(!node.unused);

    assert_eq!(
        state.for_test_anchor_by_offset.get(&34).map(String::as_str),
        Some("for_test_scope_0_34")
    );
}

#[test]
fn for_scope_re_entering_same_offset_does_not_push_a_duplicate() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::For);
    let scope = for_scope_at(Some("scope_0"), 34, 3);

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);
    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
}

#[test]
fn for_scope_with_no_upper_falls_back_to_empty_parent_in_the_id() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::For);
    let scope = for_scope_at(None, 10, 1);

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    let node = pending_synthetic_node(&arena, &state, 0);
    assert_eq!(node.id, "for_test__10");
}

fn while_body_scope(parent_span_offset: u32, block_line: u32) -> SerializedScope {
    let mut scope = base_serialized_scope("while_body");
    scope.r#type = ScopeType::Block;
    scope.upper = Some(scope_id("scope_0"));
    scope.block = SerializedBlock {
        r#type: AstType::BlockStatement,
        span: Span {
            line: SourceLine(block_line),
            column: SourceColumn(0),
            offset: Utf16CodeUnitOffset(33),
        },
        end_span: Span {
            line: SourceLine(block_line + 2),
            column: SourceColumn(1),
            offset: Utf16CodeUnitOffset(80),
        },
    };
    scope.block_context = Some(other_block_context(
        AstType::WhileStatement,
        "body",
        parent_span_offset,
        None,
    ));
    scope
}

#[test]
fn while_body_pushes_anchor_at_first_keyed_by_parent_span_offset() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::While);
    let scope = while_body_scope(27, 3);

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
    let pending = state.pending_loop_test_anchors[0];
    assert_eq!(pending.subgraph, sg);
    assert!(pending.position == LoopTestAnchorPosition::First);

    let node = pending_synthetic_node(&arena, &state, 0);
    assert_eq!(node.id, "while_test_scope_0_27");
    assert!(node.kind == SyntheticNodeKind::SyntheticWhileStatementTest);
    assert_eq!(node.name, "while-test");
    assert_eq!(node.line, 3);

    assert_eq!(
        state
            .while_test_anchor_by_offset
            .get(&27)
            .map(String::as_str),
        Some("while_test_scope_0_27")
    );
}

#[test]
fn while_body_re_entering_same_parent_span_offset_is_not_duplicated() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::While);
    let scope = while_body_scope(27, 3);

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);
    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
}

fn do_while_body_scope(parent_span_offset: u32) -> SerializedScope {
    let mut scope = base_serialized_scope("dowhile_body");
    scope.r#type = ScopeType::Block;
    scope.upper = Some(scope_id("scope_0"));
    scope.block = SerializedBlock {
        r#type: AstType::BlockStatement,
        span: Span {
            line: SourceLine(3),
            column: SourceColumn(0),
            offset: Utf16CodeUnitOffset(36),
        },
        end_span: Span {
            line: SourceLine(6),
            column: SourceColumn(1),
            offset: Utf16CodeUnitOffset(80),
        },
    };
    scope.block_context = Some(other_block_context(
        AstType::DoWhileStatement,
        "body",
        parent_span_offset,
        None,
    ));
    scope
}

#[test]
fn do_while_body_pushes_anchor_at_last_with_line_from_block_end_span() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::DoWhile);
    let scope = do_while_body_scope(33);

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
    let pending = state.pending_loop_test_anchors[0];
    assert!(pending.position == LoopTestAnchorPosition::Last);

    let node = pending_synthetic_node(&arena, &state, 0);
    assert_eq!(node.id, "do_while_test_scope_0_33");
    assert!(node.kind == SyntheticNodeKind::SyntheticDoWhileStatementTest);
    assert_eq!(node.name, "do-while-test");
    assert_eq!(node.line, 6);

    assert_eq!(
        state
            .do_while_test_anchor_by_offset
            .get(&33)
            .map(String::as_str),
        Some("do_while_test_scope_0_33")
    );
}

#[test]
fn do_while_body_re_entering_same_parent_span_offset_is_not_duplicated() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::DoWhile);
    let scope = do_while_body_scope(33);

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);
    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert_eq!(state.pending_loop_test_anchors.len(), 1);
}

#[test]
fn non_loop_block_if_statement_consequent_is_a_no_op() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::For);
    let mut scope = base_serialized_scope("block");
    scope.r#type = ScopeType::Block;
    scope.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        10,
        None,
    ));

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert!(state.pending_loop_test_anchors.is_empty());
    assert!(state.while_test_anchor_by_offset.is_empty());
    assert!(state.do_while_test_anchor_by_offset.is_empty());
    assert!(state.for_test_anchor_by_offset.is_empty());
}

#[test]
fn block_with_while_parent_but_key_not_body_is_a_no_op() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::While);
    let mut scope = base_serialized_scope("block");
    scope.r#type = ScopeType::Block;
    scope.block_context = Some(other_block_context(
        AstType::WhileStatement,
        "consequent",
        27,
        None,
    ));

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert!(state.pending_loop_test_anchors.is_empty());
    assert!(state.while_test_anchor_by_offset.is_empty());
}

#[test]
fn block_with_no_block_context_is_a_no_op() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::For);
    let mut scope = base_serialized_scope("block");
    scope.r#type = ScopeType::Block;
    scope.block_context = None;

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert!(state.pending_loop_test_anchors.is_empty());
}

#[test]
fn function_scope_is_a_no_op() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let sg = body_subgraph_idx(&mut arena, ControlSubgraphKind::For);
    let mut scope = base_serialized_scope("fn");
    scope.r#type = ScopeType::Function;

    attach_loop_test_anchor(&mut arena, &mut state, &scope, sg);

    assert!(state.pending_loop_test_anchors.is_empty());
}
