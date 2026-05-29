//! Twin of [`super::ensure_return_use_node::ensure_return_use_node`]
//! for `throw` completions. See that module's doc for the full
//! rationale -- including the `enclosing_fn_var_id` /
//! `enclosing_fn_scope_id` host-key fallback for owner-var-less
//! callbacks. The only differences here are the discriminator
//! (`SerializedCompletion::Throw`), the wrapping subgraph kind
//! (`Throw`), and the node kind (`ThrowArgumentReference`).

use unsnarl_ir::serialized::{SerializedCompletion, SerializedReference};

use crate::direction::Direction;
use crate::visual_node::SyntheticVisualNode;
use crate::visual_subgraph::OwnedVisualSubgraph;

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
    enclosing_fn_var_id: Option<&str>,
    enclosing_fn_scope_id: Option<&str>,
    r: &SerializedReference,
) -> Option<String> {
    let SerializedCompletion::Throw {
        start_span,
        end_span,
    } = &r.completion
    else {
        return None;
    };
    let host = find_host_subgraph(r, enclosing_fn_var_id, &ctx.scope_map, state)?;
    let host_key = enclosing_fn_var_id.or(enclosing_fn_scope_id)?;
    let container_key = format!("{}-{}", start_span.offset.0, end_span.offset.0);
    let existing = state
        .throw_subgraphs_by_fn
        .get(host_key)
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
        let mut sg = OwnedVisualSubgraph::throw_subgraph(
            throw_subgraph_id(host_key, &container_key),
            start_line,
            Vec::new(),
            Direction::RL,
        );
        sg.end_line = end_line;
        let descriptor = sg.into();
        let idx = arena.push_subgraph(descriptor);
        arena.append_child(Container::Subgraph(host), ElementHandle::Subgraph(idx));
        state
            .throw_subgraphs_by_fn
            .entry(host_key.to_string())
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
        let mut n = SyntheticVisualNode::throw_argument_reference(id.clone(), name, start_line);
        n.end_line = end_line;
        n.is_jsx_element = r.jsx_element.is_some();
        let node = n.into();
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
