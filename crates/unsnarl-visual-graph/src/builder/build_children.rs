//! Walks `parent_scope.child_scopes` and dispatches each child to
//! [`build_scope`]. Branches of a single `if`/`else if`/`else`
//! chain — siblings whose `branch_container_key` starts with
//! `"if:"` and matches — are grouped under a single
//! `IfElseContainer` wrapping subgraph so the rendered diagram
//! shows one merged container per chain.
//!
//! For each `if` / `else if` arm the matching test anchor is
//! placed at the head of the consequent it gates (or, when the
//! consequent did not materialise as a subgraph and the body did
//! not collapse, into the surrounding container). The `else`
//! (alternate) branch carries no test of its own. An `if`-only
//! statement (no `else`) is treated as a lone branch and rendered
//! without the `IfElseContainer` wrapping.

use std::collections::HashMap;

use unsnarl_ir::primitive::{SourceIndex, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::SerializedScope;

use crate::direction::Direction;
use crate::visual_node::{SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::branch_container_key::branch_container_key;
use super::build_scope::build_scope;
use super::context::BuilderContext;
use super::expression_statement_node_id::expression_statement_node_id;
use super::if_container_subgraph_id::if_container_subgraph_id;
use super::if_test_node_id::if_test_node_id;
use super::line_for_offset::line_for_offset;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;

fn make_if_test_anchor(
    id: String,
    offset: Utf16CodeUnitOffset,
    source_index: &SourceIndex<'_>,
) -> VisualNode {
    SyntheticVisualNode::if_statement_test(id, line_for_offset(source_index, offset)).into()
}

/// Look up (or create on first sight) the `CallProxy` wrapper
/// subgraph for the ExpressionStatement at `stmt_offset`. Returns
/// `None` when the statement has no `ExpressionStatementContainer`
/// registered -- callers should fall through to default handling
/// in that case.
///
/// First-sight allocation appends the wrapper to `container` at
/// the call site, so the wrapper lands in the same source-order
/// position as the first callback child belonging to that
/// statement. Subsequent callbacks for the same statement reuse
/// the cached subgraph index and only emit themselves as wrapper
/// children.
fn ensure_call_proxy_wrapper(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_stmt_offset: &mut HashMap<u32, SubgraphIdx>,
    stmt_offset: u32,
) -> Option<SubgraphIdx> {
    if let Some(&idx) = call_proxy_by_stmt_offset.get(&stmt_offset) {
        return Some(idx);
    }
    let container_ref = ctx
        .expression_statement_containers_by_offset
        .get(&stmt_offset)
        .copied()?;
    let id = expression_statement_node_id(stmt_offset);
    let name = render_head_expression(&container_ref.head, &ctx.source_index);
    let start_line = container_ref.start_span.line.0;
    let end_line = if container_ref.end_span.line.0 != start_line {
        Some(container_ref.end_span.line.0)
    } else {
        None
    };
    let mut sg =
        OwnedVisualSubgraph::call_proxy(id.clone(), start_line, name, Vec::new(), Direction::RL);
    sg.end_line = end_line;
    let idx = arena.push_subgraph(sg.into());
    arena.append_child(container, ElementHandle::Subgraph(idx));
    // Wire the offset cache so the downstream
    // `ensure_expression_statement_node` call (during reference
    // traversal) returns this wrapper subgraph's id instead of
    // emitting a separate leaf `expr_stmt_<offset>` node.
    state.expression_statement_by_offset.insert(stmt_offset, id);
    call_proxy_by_stmt_offset.insert(stmt_offset, idx);
    Some(idx)
}

fn push_if_test_anchor(
    arena: &mut BuildArena,
    state: &mut BuildState,
    parent_scope_id: &str,
    offset: Utf16CodeUnitOffset,
    container: Container,
    source_index: &SourceIndex<'_>,
) {
    if state.if_test_anchor_by_offset.contains_key(&offset.0) {
        return;
    }
    let id = if_test_node_id(parent_scope_id, offset.0);
    let node = make_if_test_anchor(id.clone(), offset, source_index);
    let idx = arena.push_node(node);
    arena.append_child(container, ElementHandle::Node(idx));
    state.if_test_anchor_by_offset.insert(offset.0, id);
}

/// Place the IfStatement's test anchor inside the consequent it
/// gates. `else` is the fallback path and carries no test. When the
/// consequent did not materialise as a subgraph (e.g. it was a bare
/// statement and we collapsed it into the surrounding scope), fall
/// back to the supplied container unless the consequent collapsed
/// past the depth threshold — in that case the anchor must not leak
/// into the surviving outer container.
fn attach_test_anchor_to_consequent(
    arena: &mut BuildArena,
    state: &mut BuildState,
    consequent: &SerializedScope,
    offset: Utf16CodeUnitOffset,
    fallback_container: Container,
    source_index: &SourceIndex<'_>,
) {
    if state.if_test_anchor_by_offset.contains_key(&offset.0) {
        return;
    }
    let body_sg = state.subgraph_by_scope.get(consequent.id.value()).copied();
    if let Some(body_sg) = body_sg {
        let parent_id = consequent.upper.as_ref().map(|s| s.value()).unwrap_or("");
        let id = if_test_node_id(parent_id, offset.0);
        let node = make_if_test_anchor(id.clone(), offset, source_index);
        let idx = arena.push_node(node);
        arena.prepend_child(Container::Subgraph(body_sg), ElementHandle::Node(idx));
        state.if_test_anchor_by_offset.insert(offset.0, id);
        return;
    }
    if state
        .collapsed_root_by_scope
        .contains_key(consequent.id.value())
    {
        return;
    }
    let parent_id = consequent.upper.as_ref().map(|s| s.value()).unwrap_or("");
    push_if_test_anchor(
        arena,
        state,
        parent_id,
        offset,
        fallback_container,
        source_index,
    );
}

pub fn build_children(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    parent_scope: &SerializedScope,
    container: Container,
) {
    // Collect children in source order. We hold owned references
    // (`&SerializedScope`) borrowed from the IR via `scope_map`.
    let children: Vec<&SerializedScope> = parent_scope
        .child_scopes
        .iter()
        .filter_map(|id| ctx.scope_map.get(id.value()).copied())
        .collect();

    // `ExpressionStatement start offset → CallProxy wrapper`.
    // Populated lazily inside the main loop: the wrapper is
    // appended to `container` at the position of its *first*
    // callback-arg child, and every subsequent callback child
    // belonging to the same ExpressionStatement is routed into the
    // existing wrapper. Allocating at first-sight (rather than in a
    // separate pre-pass over `children`) preserves the source order
    // of sibling scopes that interleave with the callback statement
    // -- `[block, callback, block]` now renders as
    // `[block, wrapper, block]` instead of being reordered to
    // `[wrapper, block, block]` by a pre-pass that always appends
    // wrappers first.
    let mut call_proxy_by_stmt_offset: HashMap<u32, SubgraphIdx> = HashMap::new();

    let mut i = 0;
    while i < children.len() {
        let child = children[i];
        // Callback-arg children route into a `CallProxy` wrapper.
        // The wrapper is created on first sight (so it lands at the
        // first callback's source position) and reused for any
        // later siblings that share the same statement offset.
        if let Some(cb) = child.callback_argument.as_ref() {
            // The CallProxy wrapper is an ExpressionStatement-specific
            // mechanism (it reuses the `expr_stmt_<offset>` leaf), so
            // it fires only when `statement_offset` is present.
            // Variable-bound / returned / nested callbacks carry
            // `None` and skip the wrapper -- their
            // `<callee>(args[N])` label is attached by
            // `describe_subgraph` instead.
            if let Some(stmt_offset) = cb.statement_offset {
                if let Some(wrapper_idx) = ensure_call_proxy_wrapper(
                    arena,
                    state,
                    ctx,
                    container,
                    &mut call_proxy_by_stmt_offset,
                    stmt_offset.0,
                ) {
                    build_scope(arena, state, ctx, child, Container::Subgraph(wrapper_idx));
                    i += 1;
                    continue;
                }
                // No matching ExpressionStatementContainer was
                // registered (e.g. the analyzer fired the annotation
                // but no reference inside that statement reached the
                // visual-graph layer to populate the container map).
                // Fall through to default handling so the function
                // scope still lands somewhere instead of being
                // silently dropped.
            }
        }
        let ckey = branch_container_key(child);
        let Some(key) = ckey.as_deref() else {
            build_scope(arena, state, ctx, child, container);
            i += 1;
            continue;
        };
        if !key.starts_with("if:") {
            build_scope(arena, state, ctx, child, container);
            i += 1;
            continue;
        }
        let mut group: Vec<&SerializedScope> = vec![child];
        let mut j = i + 1;
        while j < children.len() {
            let next = children[j];
            if branch_container_key(next).as_deref() != Some(key) {
                break;
            }
            group.push(next);
            j += 1;
        }
        if group.len() < 2 {
            let lone = group[0];
            let lone_offset = lone.block_context.as_ref().map(|c| c.parent_span_offset());
            build_scope(arena, state, ctx, lone, container);
            if let Some(offset) = lone_offset {
                attach_test_anchor_to_consequent(
                    arena,
                    state,
                    lone,
                    offset,
                    container,
                    &ctx.source_index,
                );
            }
            i = j;
            continue;
        }

        let offset = child
            .block_context
            .as_ref()
            .map(|c| c.parent_span_offset())
            .unwrap_or(Utf16CodeUnitOffset(0));
        let parent_id = child.upper.as_ref().map(|s| s.value()).unwrap_or("");
        let container_id = if_container_subgraph_id(parent_id, offset.0);
        let has_else = group
            .iter()
            .any(|v| v.block_context.as_ref().map(|c| c.key()) == Some("alternate"));
        let descriptor = OwnedVisualSubgraph::if_else_container(
            container_id,
            line_for_offset(&ctx.source_index, offset),
            has_else,
            Vec::new(),
            Direction::RL,
        )
        .into();
        let container_idx: SubgraphIdx = arena.push_subgraph(descriptor);
        arena.append_child(container, ElementHandle::Subgraph(container_idx));

        // Build each branch first so the consequent subgraphs exist
        // before we attach test anchors to them.
        for g in &group {
            build_scope(arena, state, ctx, g, Container::Subgraph(container_idx));
        }

        // Attach each IfStatement's test anchor to the consequent it
        // gates. Distinct `parentSpanOffset` values within the group
        // correspond to distinct IfStatement nodes.
        let mut seen_offsets: std::collections::HashSet<Utf16CodeUnitOffset> =
            std::collections::HashSet::new();
        for g in &group {
            let off = g.block_context.as_ref().map(|c| c.parent_span_offset());
            let key = g.block_context.as_ref().map(|c| c.key());
            let Some(off) = off else { continue };
            if seen_offsets.contains(&off) {
                continue;
            }
            if key != Some("consequent") {
                continue;
            }
            seen_offsets.insert(off);
            attach_test_anchor_to_consequent(
                arena,
                state,
                g,
                off,
                Container::Subgraph(container_idx),
                &ctx.source_index,
            );
        }

        let container_line = arena.subgraph(container_idx).descriptor.line();
        let child_handles: Vec<ElementHandle> = arena.subgraph(container_idx).children.clone();
        let mut container_end_line = container_line;
        for handle in child_handles {
            if let ElementHandle::Subgraph(child_sg) = handle {
                if let Some(end) = arena.subgraph(child_sg).descriptor.end_line() {
                    container_end_line = container_end_line.max(end);
                }
            }
        }
        if container_end_line != container_line {
            arena
                .subgraph_mut(container_idx)
                .descriptor
                .set_end_line(Some(container_end_line));
        }
        i = j;
    }
}

#[cfg(test)]
#[path = "build_children_test.rs"]
mod build_children_test;
