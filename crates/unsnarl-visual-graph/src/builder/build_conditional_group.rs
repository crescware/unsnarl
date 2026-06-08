//! Assemble the `ConditionalContainer` for one ternary `cond ? a : b`.
//!
//! Mirrors the `IfElseContainer` path in [`super::build_children`]: the
//! two arms (consequent, alternate) are grouped under a single owned
//! container subgraph, and the test diamond is placed at the head of the
//! consequent (`? then`) arm. A ternary always has exactly these two
//! arms — both are synthesised unconditionally (see
//! `synthesise_conditional_arms`) and the synthetic arm scopes never
//! depth-collapse — so, unlike `if` (which may lack an `else`), there is
//! no lone-arm path.

use unsnarl_ir::primitive::Utf16CodeUnitOffset;
use unsnarl_ir::serialized::SerializedScope;

use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::attach_conditional_test_anchor::attach_conditional_test_anchor;
use super::build_scope::build_scope;
use super::conditional_container_subgraph_id::conditional_container_subgraph_id;
use super::context::BuilderContext;
use super::line_for_offset::line_for_offset;
use super::state::BuildState;

pub fn build_conditional_group(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    group: &[&SerializedScope],
) {
    let child = group[0];
    let offset = child
        .block_context
        .as_ref()
        .map(|c| c.parent_span_offset())
        .unwrap_or(Utf16CodeUnitOffset(0));
    let parent_id = child.upper.as_ref().map(|s| s.value()).unwrap_or("");
    let container_id = conditional_container_subgraph_id(parent_id, offset.0);
    let descriptor = OwnedVisualSubgraph::conditional_container(
        container_id,
        line_for_offset(&ctx.source_index, offset),
        Vec::new(),
        Direction::RL,
    )
    .into();
    let container_idx: SubgraphIdx = arena.push_subgraph(descriptor);
    arena.append_child(container, ElementHandle::Subgraph(container_idx));

    // Build both arms first so the consequent subgraph exists before the
    // test diamond is prepended to it.
    for &g in group {
        build_scope(arena, state, ctx, g, Container::Subgraph(container_idx));
    }

    // The diamond lives at the head of the consequent; the alternate is
    // the fallback path and carries no test.
    for &g in group {
        let key = g.block_context.as_ref().map(|c| c.key());
        let off = g.block_context.as_ref().map(|c| c.parent_span_offset());
        if key == Some("consequent") {
            if let Some(off) = off {
                attach_conditional_test_anchor(arena, state, g, off, &ctx.source_index);
            }
        }
    }

    // Stretch the container's end line to cover both arms.
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
}
