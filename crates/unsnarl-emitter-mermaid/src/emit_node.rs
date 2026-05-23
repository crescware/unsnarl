//! Emits one `<id><shape>["<label>"]` line for a node.

use unsnarl_visual_graph::visual_node::VisualNode;

use crate::node_syntax::node_syntax_into;
use crate::render_state::RenderState;

pub fn emit_node(state: &mut RenderState<'_>, node: &VisualNode, indent: &str) {
    // Build the line directly into one owned `String` so the helper
    // chain (`node_head` → `node_label` → `node_syntax` → outer
    // `format!`) does not allocate one intermediate `String` per
    // layer. The capacity hint stays a rough lower bound; the buffer
    // grows on demand for longer labels.
    let mut line = String::with_capacity(indent.len() + node.id().len() + 32);
    line.push_str(indent);
    line.push_str(node.id());
    node_syntax_into(&mut line, node, state.debug);
    state.lines.push(line);
}

#[cfg(test)]
#[path = "emit_node_test.rs"]
mod emit_node_test;
