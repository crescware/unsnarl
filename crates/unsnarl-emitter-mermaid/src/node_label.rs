//! Builds the full mermaid label string for a node (head + line
//! range + optional `<br/>kind` suffix for `--debug`).

use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

use crate::node_head::node_head;

pub fn node_label(n: &VisualNode, debug: bool) -> String {
    let base = base_label(n);
    if debug {
        format!("{base}<br/>{}", n.kind().as_str())
    } else {
        base
    }
}

fn base_label(n: &VisualNode) -> String {
    match n.kind() {
        NodeKind::SyntheticIfStatementTest => format!("if ()<br/>L{}", n.line()),
        NodeKind::SyntheticSwitchStatementDiscriminant => {
            format!("switch ()<br/>L{}", n.line())
        }
        NodeKind::SyntheticWhileStatementTest => format!("while ()<br/>L{}", n.line()),
        NodeKind::SyntheticDoWhileStatementTest => format!("do while ()<br/>L{}", n.line()),
        NodeKind::SyntheticForStatementHeader
        | NodeKind::SyntheticForInStatementHeader
        | NodeKind::SyntheticForOfStatementHeader => format!("for ()<br/>L{}", n.line()),
        NodeKind::SyntheticBeyondDepth => {
            // The stub stands in for an arbitrary range of source
            // lines that collapsed past the queried depth; printing
            // a single line number here would be misleading, and
            // printing the full range would duplicate the
            // surrounding subgraph's L<x>-<y> label.
            node_head(n)
        }
        NodeKind::SyntheticModuleSink => "module".to_string(),
        NodeKind::SyntheticImplicitGlobal => node_head(n),
        _ => {
            let head = node_head(n);
            // Unused declarations are surfaced via a textual prefix
            // instead of a dashed border. This keeps the visual cue
            // legible even when the node already has another
            // classDef applied (boundary stub, nest level, ...).
            let prefixed = if n.unused() {
                format!("unused {head}")
            } else {
                head
            };
            let range = match n.end_line() {
                Some(end) if end != n.line() => format!("L{}-{end}", n.line()),
                _ => format!("L{}", n.line()),
            };
            format!("{prefixed}<br/>{range}")
        }
    }
}

#[cfg(test)]
#[path = "node_label_test.rs"]
mod node_label_test;
