//! Mirrors `ts/src/visual-graph/builder/ensure-throw-use-node.ts`.
//!
//! Twin of [`super::ensure_return_use_node::ensure_return_use_node`]
//! for `throw` completions. See that module's doc for the full
//! rationale; the only differences are the discriminator
//! (`SerializedCompletion::Throw`), the wrapping subgraph kind
//! (`Throw`), and the node kind (`ThrowArgumentReference`).

use unsnarl_ir::serialized::{SerializedCompletion, SerializedReference};

use crate::direction::Direction;
use crate::visual_element_type::{NodeTypeTag, SubgraphTypeTag};
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::{OwnedExtras, OwnedSubgraphKind, OwnedVisualSubgraph, VisualSubgraph};

use super::arena::{BuildArena, Container, ElementHandle};
use super::context::BuilderContext;
use super::find_host_subgraph::find_host_subgraph;
use super::state::BuildState;
use super::throw_subgraph_id::throw_subgraph_id;
use super::throw_use_node_id::throw_use_node_id;

pub fn ensure_throw_use_node(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    enclosing_fn_var_id: &str,
    r: &SerializedReference,
) -> Option<String> {
    let SerializedCompletion::Throw {
        start_span,
        end_span,
    } = &r.completion
    else {
        return None;
    };
    let host = find_host_subgraph(r, Some(enclosing_fn_var_id), &ctx.scope_map, state)?;
    let container_key = format!("{}-{}", start_span.offset.0, end_span.offset.0);
    let existing = state
        .throw_subgraphs_by_fn
        .get(enclosing_fn_var_id)
        .and_then(|m| m.get(&container_key).copied());
    let sg_idx = if let Some(idx) = existing {
        idx
    } else {
        let start_line = start_span.line.0;
        let raw_end_line = end_span.line.0;
        let end_line = if raw_end_line != start_line {
            Some(raw_end_line)
        } else {
            None
        };
        let descriptor = VisualSubgraph::Owned(OwnedVisualSubgraph {
            r#type: SubgraphTypeTag::Subgraph,
            id: throw_subgraph_id(enclosing_fn_var_id, &container_key),
            kind: OwnedSubgraphKind::Throw,
            line: start_line,
            end_line,
            direction: Direction::RL,
            extras: OwnedExtras::None {},
            elements: Vec::new(),
        });
        let idx = arena.push_subgraph(descriptor);
        arena.append_child(Container::Subgraph(host), ElementHandle::Subgraph(idx));
        state
            .throw_subgraphs_by_fn
            .entry(enclosing_fn_var_id.to_string())
            .or_default()
            .insert(container_key, idx);
        idx
    };
    let id = throw_use_node_id(r.id.value());
    if !state.throw_use_added.contains(r.id.value()) {
        state.throw_use_added.insert(r.id.value().to_string());
        let v = r
            .resolved
            .as_ref()
            .and_then(|id| ctx.variable_map.get(id.value()).copied());
        let name = v
            .map(|v| v.name().to_string())
            .unwrap_or_else(|| r.identifier.name().to_string());
        let start_line = r.identifier.span().line.0;
        let jsx_end = r.jsx_element.as_ref().map(|j| j.end_span.line.0);
        let end_line = match jsx_end {
            Some(line) if line != start_line => Some(line),
            _ => None,
        };
        let node = VisualNode::Synthetic(SyntheticVisualNode {
            r#type: NodeTypeTag::Node,
            id: id.clone(),
            kind: SyntheticNodeKind::ThrowArgumentReference,
            name,
            line: start_line,
            end_line,
            is_jsx_element: r.jsx_element.is_some(),
            unused: false,
            extras: SyntheticExtras::None {},
        });
        let node_idx = arena.push_node(node);
        arena.append_child(Container::Subgraph(sg_idx), ElementHandle::Node(node_idx));
        state
            .node_id_origin_scope
            .insert(id.clone(), r.from.value().to_string());
    }
    Some(id)
}

#[cfg(test)]
#[path = "ensure_throw_use_node_test.rs"]
mod ensure_throw_use_node_test;
