//! Wraps a node's label in the right mermaid shape syntax
//! (`["..."]`, `((...))`, `{"..."}`, ...).
//!
//! Mirrors `ts/src/emitter/mermaid/node-syntax.ts`.

use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

use crate::node_label::node_label;

pub fn node_syntax(n: &VisualNode, debug: bool) -> String {
    let label = node_label(n, debug);
    match n.kind() {
        NodeKind::WriteReference => format!(r#"(["{label}"])"#),
        NodeKind::SyntheticModuleSink => format!("(({label}))"),
        NodeKind::SyntheticBeyondDepth => {
            // Circle shape mirrors the pruning boundary stub; both
            // stand in for "more graph keeps going past this
            // rendered boundary".
            format!("(({label}))")
        }
        NodeKind::SyntheticIfStatementTest | NodeKind::SyntheticSwitchStatementDiscriminant => {
            format!(r#"{{"{label}"}}"#)
        }
        _ => format!(r#"["{label}"]"#),
    }
}

#[cfg(test)]
#[path = "node_syntax_test.rs"]
mod node_syntax_test;
