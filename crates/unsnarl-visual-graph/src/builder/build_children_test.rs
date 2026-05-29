//! Sibling tests for [`build_children`].

use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::serialized_scope::SerializedBlock;
use unsnarl_ir::serialized::{
    SerializedCallbackArgument, SerializedExpressionStatementContainer, SerializedHeadExpression,
    SerializedReference, SerializedScope,
};
use unsnarl_oxc_parity::AstType;

use super::build_children;
use crate::builder::arena::{BuildArena, Container, ElementHandle, NodeIdx, SubgraphIdx};
use crate::builder::builder_fixtures::{
    base_builder_context, base_serialized_reference, base_serialized_scope, empty_serialized_ir,
    other_block_context, scope_id, span_offset_line,
};
use crate::builder::expression_statement_index::ExpressionStatementIndex;
use crate::builder::state::BuildState;
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
fn collapsed_consequent_drops_the_if_test_anchor() {
    // A consequent gated past the depth ceiling collapses, so
    // `build_scope` records it as collapsed and builds no subgraph.
    // With no subgraph to host the if-test anchor, the anchor must be
    // dropped rather than leaking into the surrounding container.
    let mut ir = empty_serialized_ir();
    let mut cons = if_branch("c", "outer", "consequent", 5);
    cons.nesting_depths = NestingDepths::uniform(NestingDepth(2));
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("c")];
    ir.scopes.push(outer);
    ir.scopes.push(cons);

    let mut ctx = base_builder_context(&ir);
    ctx.depths = Some(NestingDepths::uniform(NestingDepth(1)));
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    // The collapsed consequent built no subgraph and no if-test
    // anchor was registered anywhere.
    assert!(root_subgraphs(&arena).is_empty());
    assert!(state.if_test_anchor_by_offset.is_empty());
    // No SyntheticIfStatementTest node leaked into the root container.
    for h in &arena.root_children {
        if let ElementHandle::Node(idx) = h {
            if let VisualNode::Synthetic(s) = arena.node(*idx).clone() {
                assert!(!matches!(
                    s.kind,
                    SyntheticNodeKind::SyntheticIfStatementTest
                ));
            }
        }
    }
}

fn callback_fn_scope(id: &str, upper: &str, stmt_offset: u32, arg_index: u32) -> SerializedScope {
    let mut s = scope_with_upper(id, upper);
    s.r#type = ScopeType::Function;
    s.block = block(stmt_offset, 1, stmt_offset + 10, 3);
    // The CallProxy wrapper is driven by span containment against the
    // registered ExpressionStatement containers, not by a field on the
    // annotation -- the scope's block span (set above to start at
    // `stmt_offset`) is what correlates it to its statement.
    s.callback_argument = Some(SerializedCallbackArgument {
        callee: SerializedHeadExpression::identifier("cb".to_string()),
        arg_index,
    });
    s
}

fn expr_stmt_container(
    start: u32,
    end: u32,
    callee_name: &str,
) -> SerializedExpressionStatementContainer {
    SerializedExpressionStatementContainer {
        start_span: span_offset_line(start, 1),
        end_span: span_offset_line(end, 3),
        head: SerializedHeadExpression::Call {
            callee: Box::new(SerializedHeadExpression::identifier(
                callee_name.to_string(),
            )),
            start_span: span_offset_line(start, 1),
            end_span: span_offset_line(end, 3),
        },
    }
}

/// Wrap each container in a reference so it can drive an
/// [`ExpressionStatementIndex`] -- the same shape `build` consumes
/// from `ir.references`. The returned Vec must outlive the
/// `BuilderContext` it is indexed into.
fn refs_for(containers: Vec<SerializedExpressionStatementContainer>) -> Vec<SerializedReference> {
    containers
        .into_iter()
        .map(|c| {
            let mut r = base_serialized_reference();
            r.expression_statement_container = Some(c);
            r
        })
        .collect()
}

#[test]
fn sibling_callbacks_for_the_same_statement_share_one_call_proxy_wrapper() {
    let mut ir = empty_serialized_ir();
    let stmt_offset: u32 = 100;
    let cb_a = callback_fn_scope("cbA", "outer", stmt_offset, 0);
    let cb_b = callback_fn_scope("cbB", "outer", stmt_offset, 1);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("cbA"), scope_id("cbB")];
    ir.scopes.push(outer);
    ir.scopes.push(cb_a);
    ir.scopes.push(cb_b);

    let refs = refs_for(vec![expr_stmt_container(
        stmt_offset,
        stmt_offset + 20,
        "run",
    )]);
    let mut ctx = base_builder_context(&ir);
    ctx.expression_statement_index = ExpressionStatementIndex::build(&refs);

    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    // Exactly one root subgraph -- the CallProxy wrapping both callbacks.
    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 1, "expected a single root subgraph");
    let VisualSubgraph::Owned(o) = descriptor_of(&arena, sgs[0]) else {
        panic!("expected owned subgraph");
    };
    assert!(matches!(o.kind, OwnedSubgraphKind::CallProxy));
    let OwnedExtras::CallProxy { call_name } = &o.extras else {
        panic!("expected CallProxy extras");
    };
    assert_eq!(call_name, "run()");

    // Both function scopes were routed into the wrapper.
    let wrapper_children: Vec<SubgraphIdx> = arena
        .subgraph(sgs[0])
        .children
        .iter()
        .filter_map(|h| match h {
            ElementHandle::Subgraph(idx) => Some(*idx),
            _ => None,
        })
        .collect();
    assert_eq!(wrapper_children.len(), 2);
    for child in wrapper_children {
        let VisualSubgraph::Owned(c) = descriptor_of(&arena, child) else {
            panic!("expected owned child");
        };
        assert!(matches!(c.kind, OwnedSubgraphKind::Function));
    }

    // The expression-statement offset cache has been pre-populated so
    // downstream `ensure_expression_statement_node` reuses the
    // wrapper's id instead of allocating a separate leaf node.
    let cached = state
        .expression_statement_by_offset
        .get(&stmt_offset)
        .expect("offset must be cached");
    assert_eq!(cached, &format!("expr_stmt_{stmt_offset}"));
}

#[test]
fn falls_through_to_default_handling_when_no_matching_container_is_registered() {
    let mut ir = empty_serialized_ir();
    let stmt_offset: u32 = 200;
    let cb = callback_fn_scope("cb", "outer", stmt_offset, 0);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("cb")];
    ir.scopes.push(outer);
    ir.scopes.push(cb);

    // Deliberately leave the `ExpressionStatementIndex` empty so no
    // statement encloses the callback's block span.
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    // No CallProxy wrapper; the function scope lands directly in
    // the parent container and the offset cache is left untouched
    // for `ensure_expression_statement_node` to handle later.
    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 1);
    let VisualSubgraph::Owned(o) = descriptor_of(&arena, sgs[0]) else {
        panic!("expected owned");
    };
    assert!(matches!(o.kind, OwnedSubgraphKind::Function));
    assert!(!state
        .expression_statement_by_offset
        .contains_key(&stmt_offset));
}

#[test]
fn distinct_statement_offsets_get_distinct_call_proxy_wrappers() {
    let mut ir = empty_serialized_ir();
    let cb_a = callback_fn_scope("cbA", "outer", 100, 0);
    let cb_b = callback_fn_scope("cbB", "outer", 200, 0);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("cbA"), scope_id("cbB")];
    ir.scopes.push(outer);
    ir.scopes.push(cb_a);
    ir.scopes.push(cb_b);

    let refs = refs_for(vec![
        expr_stmt_container(100, 120, "first"),
        expr_stmt_container(200, 220, "second"),
    ]);
    let mut ctx = base_builder_context(&ir);
    ctx.expression_statement_index = ExpressionStatementIndex::build(&refs);

    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 2);
    let call_names: Vec<String> = sgs
        .iter()
        .filter_map(|idx| match descriptor_of(&arena, *idx) {
            VisualSubgraph::Owned(o) => match o.extras {
                OwnedExtras::CallProxy { call_name } => Some(call_name),
                _ => None,
            },
            _ => None,
        })
        .collect();
    assert_eq!(
        call_names,
        vec!["first()".to_string(), "second()".to_string()]
    );
}

#[test]
fn call_proxy_wrapper_lands_at_first_callback_position_preserving_sibling_order() {
    // Regression: a previous implementation pre-walked `children`
    // and appended every CallProxy wrapper before the main child
    // loop ever started, so a `[before, callback, after]` child
    // sequence collapsed into `[wrapper, before, after]` because
    // the wrapper was appended first regardless of the callback's
    // source-order position. The wrapper must now appear at the
    // position of its first callback child so the surrounding
    // non-callback siblings keep their relative order.
    let mut ir = empty_serialized_ir();
    let stmt_offset: u32 = 100;
    let before = for_scope("before", "outer");
    let cb = callback_fn_scope("cb", "outer", stmt_offset, 0);
    let after = for_scope("after", "outer");
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("before"), scope_id("cb"), scope_id("after")];
    ir.scopes.push(outer);
    ir.scopes.push(before);
    ir.scopes.push(cb);
    ir.scopes.push(after);

    let refs = refs_for(vec![expr_stmt_container(
        stmt_offset,
        stmt_offset + 20,
        "run",
    )]);
    let mut ctx = base_builder_context(&ir);
    ctx.expression_statement_index = ExpressionStatementIndex::build(&refs);

    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sgs = root_subgraphs(&arena);
    assert_eq!(
        sgs.len(),
        3,
        "expected [before, wrapper, after] -- got {} root subgraphs",
        sgs.len()
    );
    let kinds: Vec<&'static str> = sgs
        .iter()
        .map(|idx| match descriptor_of(&arena, *idx) {
            VisualSubgraph::Owned(o) => match o.kind {
                OwnedSubgraphKind::CallProxy => "call-proxy",
                OwnedSubgraphKind::Function => "function",
                _ => "other-owned",
            },
            VisualSubgraph::Control(c) => match c.kind {
                ControlSubgraphKind::For => "for",
                _ => "other-control",
            },
        })
        .collect();
    assert_eq!(
        kinds,
        vec!["for", "call-proxy", "for"],
        "wrapper must appear between the two `for` siblings, not before them",
    );
}

#[test]
fn callback_at_the_end_appends_wrapper_after_earlier_siblings() {
    // Single-callback variant of the order-preservation regression:
    // `[before, callback]` must render as `[before, wrapper]`, not
    // `[wrapper, before]`. Catches a reordering that would survive
    // the multi-sibling test if the implementation happened to
    // append wrappers first only when *multiple* callbacks share a
    // statement.
    let mut ir = empty_serialized_ir();
    let stmt_offset: u32 = 200;
    let before = for_scope("before", "outer");
    let cb = callback_fn_scope("cb", "outer", stmt_offset, 0);
    let mut outer = base_serialized_scope("outer");
    outer.child_scopes = vec![scope_id("before"), scope_id("cb")];
    ir.scopes.push(outer);
    ir.scopes.push(before);
    ir.scopes.push(cb);

    let refs = refs_for(vec![expr_stmt_container(
        stmt_offset,
        stmt_offset + 20,
        "tail",
    )]);
    let mut ctx = base_builder_context(&ir);
    ctx.expression_statement_index = ExpressionStatementIndex::build(&refs);

    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_children(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sgs = root_subgraphs(&arena);
    assert_eq!(sgs.len(), 2);
    assert!(
        matches!(
            descriptor_of(&arena, sgs[0]),
            VisualSubgraph::Control(c) if matches!(c.kind, ControlSubgraphKind::For)
        ),
        "first sibling must be the `before` For scope",
    );
    assert!(
        matches!(
            descriptor_of(&arena, sgs[1]),
            VisualSubgraph::Owned(o) if matches!(o.kind, OwnedSubgraphKind::CallProxy)
        ),
        "second sibling must be the CallProxy wrapper",
    );
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
