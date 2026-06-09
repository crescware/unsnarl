//! Wraps a node's label in the right mermaid shape syntax
//! (`["..."]`, `((...))`, `{"..."}`, ...).

use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

use crate::node_label::node_label_into;

pub fn node_syntax(n: &VisualNode, debug: bool) -> String {
    let mut out = String::new();
    node_syntax_into(&mut out, n, debug);
    out
}

/// Destination-arg variant of [`node_syntax`]: writes the wrapped
/// label directly into `out` so `emit_node` can build the entire
/// `<indent><id><syntax>` line into a single buffer rather than
/// stacking one fresh `String` per nesting level (`node_head` →
/// `node_label` → `node_syntax` → outer line).
pub fn node_syntax_into(out: &mut String, n: &VisualNode, debug: bool) {
    let (open, close) = match n.kind() {
        NodeKind::WriteReference => ("([\"", "\"])"),
        // Circle shape mirrors the pruning boundary stub; both
        // stand in for "more graph keeps going past this rendered
        // boundary". Neither variant quotes the inner label.
        NodeKind::SyntheticModuleSink | NodeKind::SyntheticBeyondDepth => ("((", "))"),
        NodeKind::SyntheticIfStatementTest
        | NodeKind::SyntheticSwitchStatementDiscriminant
        | NodeKind::SyntheticConditionalTest => ("{\"", "\"}"),
        _ => ("[\"", "\"]"),
    };
    out.push_str(open);
    node_label_into(out, n, debug);
    out.push_str(close);
}

#[cfg(test)]
#[path = "node_syntax_test.rs"]
mod node_syntax_test;
