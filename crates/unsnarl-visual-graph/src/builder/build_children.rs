//! Mirrors `ts/src/visual-graph/builder/build-children.ts`.
//!
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

use unsnarl_ir::serialized::SerializedScope;

use crate::direction::Direction;
use crate::visual_element_type::{NodeTypeTag, SubgraphTypeTag};
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::{OwnedExtras, OwnedSubgraphKind, OwnedVisualSubgraph, VisualSubgraph};

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::branch_container_key::branch_container_key;
use super::build_scope::build_scope;
use super::context::BuilderContext;
use super::if_container_subgraph_id::if_container_subgraph_id;
use super::if_test_node_id::if_test_node_id;
use super::line_for_offset::line_for_offset;
use super::state::BuildState;

fn make_if_test_anchor(id: String, offset: u32, raw: &str) -> VisualNode {
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id,
        kind: SyntheticNodeKind::SyntheticIfStatementTest,
        name: "if-test".to_string(),
        line: line_for_offset(raw, offset),
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    })
}

fn push_if_test_anchor(
    arena: &mut BuildArena,
    state: &mut BuildState,
    parent_scope_id: &str,
    offset: u32,
    container: Container,
    raw: &str,
) {
    if state.if_test_anchor_by_offset.contains_key(&offset) {
        return;
    }
    let id = if_test_node_id(parent_scope_id, offset);
    let node = make_if_test_anchor(id.clone(), offset, raw);
    let idx = arena.push_node(node);
    arena.append_child(container, ElementHandle::Node(idx));
    state.if_test_anchor_by_offset.insert(offset, id);
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
    offset: u32,
    fallback_container: Container,
    raw: &str,
) {
    if state.if_test_anchor_by_offset.contains_key(&offset) {
        return;
    }
    let body_sg = state.subgraph_by_scope.get(consequent.id.value()).copied();
    if let Some(body_sg) = body_sg {
        let parent_id = consequent.upper.as_ref().map(|s| s.value()).unwrap_or("");
        let id = if_test_node_id(parent_id, offset);
        let node = make_if_test_anchor(id.clone(), offset, raw);
        let idx = arena.push_node(node);
        arena.prepend_child(Container::Subgraph(body_sg), ElementHandle::Node(idx));
        state.if_test_anchor_by_offset.insert(offset, id);
        return;
    }
    if state
        .collapsed_root_by_scope
        .contains_key(consequent.id.value())
    {
        return;
    }
    let parent_id = consequent.upper.as_ref().map(|s| s.value()).unwrap_or("");
    push_if_test_anchor(arena, state, parent_id, offset, fallback_container, raw);
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

    let mut i = 0;
    while i < children.len() {
        let child = children[i];
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
            let lone_offset = lone
                .block_context
                .as_ref()
                .map(|c| c.parent_span_offset().0);
            build_scope(arena, state, ctx, lone, container);
            if let Some(offset) = lone_offset {
                attach_test_anchor_to_consequent(
                    arena,
                    state,
                    lone,
                    offset,
                    container,
                    &ctx.ir.raw,
                );
            }
            i = j;
            continue;
        }

        let offset = child
            .block_context
            .as_ref()
            .map(|c| c.parent_span_offset().0)
            .unwrap_or(0);
        let parent_id = child.upper.as_ref().map(|s| s.value()).unwrap_or("");
        let container_id = if_container_subgraph_id(parent_id, offset);
        let has_else = group
            .iter()
            .any(|v| v.block_context.as_ref().map(|c| c.key()) == Some("alternate"));
        let descriptor = VisualSubgraph::Owned(OwnedVisualSubgraph {
            r#type: SubgraphTypeTag::Subgraph,
            id: container_id,
            kind: OwnedSubgraphKind::IfElseContainer,
            line: line_for_offset(&ctx.ir.raw, offset),
            end_line: None,
            direction: Direction::RL,
            extras: OwnedExtras::IfElseContainer { has_else },
            elements: Vec::new(),
        });
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
        let mut seen_offsets: std::collections::HashSet<u32> = std::collections::HashSet::new();
        for g in &group {
            let off = g.block_context.as_ref().map(|c| c.parent_span_offset().0);
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
                &ctx.ir.raw,
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
