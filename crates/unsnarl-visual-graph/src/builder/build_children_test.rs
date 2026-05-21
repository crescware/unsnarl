//! Sibling tests for [`build_children`]. Cases mirror
//! `ts/src/visual-graph/builder/build-children.test.ts`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::serialized_scope::SerializedBlock;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::build_children;
use crate::builder::arena::{BuildArena, Container, ElementHandle, NodeIdx, SubgraphIdx};
use crate::builder::state::BuildState;
use crate::builder::testing::{
    base_builder_context, base_serialized_scope, empty_serialized_ir, other_block_context,
    scope_id, span_offset_line,
};
use crate::visual_node::{SyntheticNodeKind, VisualNode};
use crate::visual_subgraph::{ControlSubgraphKind, OwnedExtras, OwnedSubgraphKind, VisualSubgraph};

fn block(span_offset: u32, span_line: u32, end_offset: u32, end_line: u32) -> SerializedBlock {
    SerializedBlock {
        r#type: AstType::BlockStatement,
        span: span_offset_line(span_offset, span_line),
        end_span: span_offset_line(end_offset, end_line),
    }
}

fn scope_with_upper(id: &str, upper: &str) -> SerializedScope {
    let mut s = base_serialized_scope(id);
    s.upper = Some(scope_id(upper));
    s
}

fn for_scope(id: &str, upper: &str) -> SerializedScope {
    let mut s = scope_with_upper(id, upper);
    s.r#type = ScopeType::For;
    s
}

fn if_branch(id: &str, upper: &str, key: &str, parent_offset: u32) -> SerializedScope {
    let mut s = scope_with_upper(id, upper);
    s.block_context = Some(other_block_context(
        AstType::IfStatement,
        key,
        parent_offset,
        None,
    ));
    s
}

fn descriptor_of(arena: &BuildArena, idx: SubgraphIdx) -> VisualSubgraph {
    arena.subgraph(idx).descriptor.clone()
}

fn root_subgraphs(arena: &BuildArena) -> Vec<SubgraphIdx> {
    arena
        .root_children
        .iter()
        .filter_map(|h| match h {
            ElementHandle::Subgraph(idx) => Some(*idx),
            _ => None,
        })
        .collect()
}

#[test]
fn non_branch_children_are_built_directly_into_parent_container() {
    let mut ir = empty_serialized_ir();
    let inner = for_scope("for1", "outer");
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("for1")];
    ir.scopes.push(outer);
    ir.scopes.push(inner);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 1);
    let VisualSubgraph::Control(c) = descriptor_of(&arena, sgs[0]) else {
        panic!("expected control");
    };
    assert!(matches!(c.kind, ControlSubgraphKind::For));
}

fn first_node_in_subgraph(arena: &BuildArena, sg: SubgraphIdx) -> Option<NodeIdx> {
    arena.subgraph(sg).children.iter().find_map(|h| match h {
        ElementHandle::Node(idx) => Some(*idx),
        _ => None,
    })
}

#[test]
fn single_if_branch_is_not_wrapped_in_if_else_container() {
    let mut ir = empty_serialized_ir();
    let cons = if_branch("c", "outer", "consequent", 5);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("c")];
    ir.scopes.push(outer);
    ir.scopes.push(cons);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 1);
    let if_sg = sgs[0];
    let VisualSubgraph::Control(c) = descriptor_of(&arena, if_sg) else {
        panic!("expected control");
    };
    assert!(matches!(c.kind, ControlSubgraphKind::If));
    // No IfElseContainer in root.
    assert!(!sgs.iter().any(|idx| matches!(
        descriptor_of(&arena, *idx),
        VisualSubgraph::Owned(o) if matches!(o.kind, OwnedSubgraphKind::IfElseContainer)
    )));

    // Anchor inside the consequent subgraph.
    let anchor_idx = first_node_in_subgraph(&arena, if_sg).expect("anchor");
    let VisualNode::Synthetic(s) = arena.node(anchor_idx).clone() else {
        panic!("expected synthetic anchor");
    };
    assert!(matches!(
        s.kind,
        SyntheticNodeKind::SyntheticIfStatementTest
    ));
}

#[test]
fn consecutive_if_siblings_wrap_in_if_else_container_with_has_else_true() {
    let mut ir = empty_serialized_ir();
    ir.raw = "\n".repeat(20);
    let mut cons = if_branch("c", "outer", "consequent", 5);
    cons.block = block(5, 1, 10, 2);
    let mut alt = if_branch("a", "outer", "alternate", 5);
    alt.block = block(11, 3, 20, 5);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("c"), scope_id("a")];
    ir.scopes.push(outer);
    ir.scopes.push(cons);
    ir.scopes.push(alt);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 1);
    let container_idx = sgs[0];
    let VisualSubgraph::Owned(o) = descriptor_of(&arena, container_idx) else {
        panic!("expected owned");
    };
    assert!(matches!(o.kind, OwnedSubgraphKind::IfElseContainer));
    let OwnedExtras::IfElseContainer { has_else } = o.extras else {
        panic!("expected IfElseContainer extras");
    };
    assert!(has_else);

    let children: Vec<_> = arena
        .subgraph(container_idx)
        .children
        .iter()
        .filter_map(|h| match h {
            ElementHandle::Subgraph(idx) => Some(*idx),
            _ => None,
        })
        .collect();
    assert_eq!(children.len(), 2);
    let kinds: Vec<_> = children
        .iter()
        .map(|i| descriptor_of(&arena, *i))
        .filter_map(|d| match d {
            VisualSubgraph::Control(c) => Some(c.kind),
            _ => None,
        })
        .collect();
    assert!(matches!(kinds[0], ControlSubgraphKind::If));
    assert!(matches!(kinds[1], ControlSubgraphKind::Else));

    // Anchor lives inside the `if` consequent only.
    let if_anchor = first_node_in_subgraph(&arena, children[0]).expect("if anchor");
    let VisualNode::Synthetic(s) = arena.node(if_anchor).clone() else {
        panic!("expected synthetic");
    };
    assert!(matches!(
        s.kind,
        SyntheticNodeKind::SyntheticIfStatementTest
    ));

    // Else subgraph has no SyntheticIfStatementTest.
    for child in &arena.subgraph(children[1]).children {
        if let ElementHandle::Node(idx) = child {
            let VisualNode::Synthetic(s) = arena.node(*idx).clone() else {
                continue;
            };
            assert!(!matches!(
                s.kind,
                SyntheticNodeKind::SyntheticIfStatementTest
            ));
        }
    }
}

#[test]
fn if_else_container_end_line_is_the_max_end_line_among_grouped_branches() {
    let mut ir = empty_serialized_ir();
    ir.raw = "\n".repeat(20);
    let mut cons = if_branch("c", "outer", "consequent", 5);
    cons.block = block(5, 1, 10, 2);
    let mut alt = if_branch("a", "outer", "alternate", 5);
    alt.block = block(11, 3, 20, 7);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("c"), scope_id("a")];
    ir.scopes.push(outer);
    ir.scopes.push(cons);
    ir.scopes.push(alt);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let container_idx = root_subgraphs(&arena)[0];
    let VisualSubgraph::Owned(o) = descriptor_of(&arena, container_idx) else {
        panic!("expected owned");
    };
    assert_eq!(o.end_line, Some(7));
}

#[test]
fn two_adjacent_ifs_with_different_offsets_are_not_merged() {
    let mut ir = empty_serialized_ir();
    let if_a = if_branch("ifA", "outer", "consequent", 5);
    let if_b = if_branch("ifB", "outer", "consequent", 30);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("ifA"), scope_id("ifB")];
    ir.scopes.push(outer);
    ir.scopes.push(if_a);
    ir.scopes.push(if_b);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 2);
    for sg in &sgs {
        let VisualSubgraph::Control(c) = descriptor_of(&arena, *sg) else {
            panic!("expected control");
        };
        assert!(matches!(c.kind, ControlSubgraphKind::If));
        let anchor_idx = first_node_in_subgraph(&arena, *sg).expect("anchor");
        let VisualNode::Synthetic(s) = arena.node(anchor_idx).clone() else {
            panic!("expected synthetic");
        };
        assert!(matches!(
            s.kind,
            SyntheticNodeKind::SyntheticIfStatementTest
        ));
    }
}

#[test]
fn missing_child_id_is_skipped_silently() {
    let mut ir = empty_serialized_ir();
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("missing")];
    ir.scopes.push(outer);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    assert!(arena.root_children.is_empty());
}
