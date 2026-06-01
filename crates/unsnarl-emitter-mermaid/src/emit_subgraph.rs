//! Emits a subgraph block, wrapping it with its owner node when one
//! is known. Only a named `Function` carries an owner (the
//! FunctionName node); it expresses the owner ↔ subgraph relationship
//! by containment in a shared `wrap_<id>` box. A result-bound
//! `CallProxy` shows its call ↔ binding relationship by an edge
//! instead, so it carries no owner and falls through to a plain
//! subgraph here.

use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

use crate::emit_node::emit_node;
use crate::emit_plain_subgraph::emit_plain_subgraph;
use crate::record_nest_slot::record_nest_slot;
use crate::render_state::RenderState;

pub fn emit_subgraph(state: &mut RenderState<'_>, sg: &VisualSubgraph, indent: &str, depth: u32) {
    // `owner_node_id` is `Some` only for a named Function.
    if let Some(owner_id) = sg.owner_node_id() {
        if let Some(&owner_node) = state.node_map.get(owner_id) {
            // Wrap the owner node and the body subgraph as SIBLINGS
            // inside a single wrapper subgraph. The owner node belongs
            // to the parent scope (it names the function from the
            // outside), so it must NOT live inside the body subgraph.
            // The wrapper exists purely to keep these two siblings
            // adjacent in the rendered diagram.
            let wrap_id = format!("wrap_{}", sg.id());
            // The wrapper sits one palette slot ABOVE the body so the
            // two nested rectangles read as different brightness
            // levels. Without that contrast the wrapper visually merges
            // with its body and the owner-vs-body boundary disappears.
            // body depth = wrap depth + 1 keeps the gradient monotonic.
            record_nest_slot(state, &wrap_id, depth);
            let mut header = String::with_capacity(indent.len() + 11 + wrap_id.len() + 5);
            header.push_str(indent);
            header.push_str("subgraph ");
            header.push_str(&wrap_id);
            header.push_str("[\" \"]");
            state.lines.push(header);
            let wrap_indent = format!("{indent}  ");
            let mut direction_line =
                String::with_capacity(wrap_indent.len() + "direction TB".len());
            direction_line.push_str(&wrap_indent);
            direction_line.push_str("direction TB");
            state.lines.push(direction_line);
            emit_node(state, owner_node, &wrap_indent);
            emit_plain_subgraph(state, sg, &wrap_indent, depth + 1);
            let mut end_line = String::with_capacity(indent.len() + 3);
            end_line.push_str(indent);
            end_line.push_str("end");
            state.lines.push(end_line);
            return;
        }
    }
    emit_plain_subgraph(state, sg, indent, depth);
}

#[cfg(test)]
#[path = "emit_subgraph_test.rs"]
mod emit_subgraph_test;
