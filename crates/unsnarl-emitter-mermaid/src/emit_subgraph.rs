//! Emits a subgraph block, wrapping it with its owner Function
//! node when the kind is Function and an owner is known.
//!
//! Mirrors `ts/src/emitter/mermaid/emit-subgraph.ts`.

use unsnarl_visual_graph::subgraph_kind::SubgraphKind;
use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

use crate::emit_node::emit_node;
use crate::emit_plain_subgraph::emit_plain_subgraph;
use crate::record_nest_slot::record_nest_slot;
use crate::render_state::RenderState;

pub fn emit_subgraph(state: &mut RenderState<'_>, sg: &VisualSubgraph, indent: &str, depth: u32) {
    if sg.kind() == SubgraphKind::Function {
        if let Some(owner_id) = sg.owner_node_id() {
            if let Some(&owner_node) = state.node_map.get(owner_id) {
                // Wrap the FunctionName node and the function body
                // subgraph as SIBLINGS inside a single wrapper
                // subgraph. The FunctionName node belongs to the
                // parent scope (it names the function from the
                // outside), so it must NOT live inside the body
                // subgraph -- that would imply "f references
                // itself from within its own body". The wrapper
                // exists purely to keep these two siblings
                // adjacent in the rendered diagram.
                let wrap_id = format!("wrap_{}", sg.id());
                // The wrapper sits one palette slot ABOVE the body
                // so the two nested rectangles read as different
                // brightness levels. Without that contrast the
                // wrapper visually merges with its body and the
                // function-vs-body boundary disappears. body depth
                // = wrap depth + 1 keeps the gradient monotonic.
                record_nest_slot(state, &wrap_id, depth);
                state
                    .lines
                    .push(format!("{indent}subgraph {wrap_id}[\" \"]"));
                let wrap_indent = format!("{indent}  ");
                state.lines.push(format!("{wrap_indent}direction TB"));
                emit_node(state, owner_node, &wrap_indent);
                emit_plain_subgraph(state, sg, &wrap_indent, depth + 1);
                state.lines.push(format!("{indent}end"));
                return;
            }
        }
    }
    emit_plain_subgraph(state, sg, indent, depth);
}

#[cfg(test)]
#[path = "emit_subgraph_test.rs"]
mod emit_subgraph_test;
