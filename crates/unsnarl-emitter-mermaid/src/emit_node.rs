//! Emits one `<id><shape>["<label>"]` line for a node.

use unsnarl_visual_graph::visual_node::VisualNode;

use crate::node_syntax::node_syntax;
use crate::render_state::RenderState;

pub fn emit_node(state: &mut RenderState<'_>, node: &VisualNode, indent: &str) {
    let line = format!("{indent}{}{}", node.id(), node_syntax(node, state.debug));
    state.lines.push(line);
}

#[cfg(test)]
#[path = "emit_node_test.rs"]
mod emit_node_test;
