//! Builds the full mermaid label string for a node (head + line
//! range + optional `<br/>kind` suffix for `--debug`).

use std::fmt::Write as _;

use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

use crate::node_head::node_head_into;

pub fn node_label(n: &VisualNode, debug: bool) -> String {
    let mut out = String::new();
    node_label_into(&mut out, n, debug);
    out
}

/// Destination-arg variant of [`node_label`]: writes the full label
/// (head + line range + optional debug suffix) directly into `out`,
/// reusing one growing buffer instead of allocating a fresh `String`
/// at every nesting level.
pub fn node_label_into(out: &mut String, n: &VisualNode, debug: bool) {
    base_label_into(out, n);
    if debug {
        out.push_str("<br/>");
        out.push_str(n.kind().as_str());
    }
}

fn base_label_into(out: &mut String, n: &VisualNode) {
    match n.kind() {
        NodeKind::SyntheticIfStatementTest => {
            write!(out, "if ()<br/>L{}", n.line()).expect("writing to String is infallible");
        }
        NodeKind::SyntheticSwitchStatementDiscriminant => {
            write!(out, "switch ()<br/>L{}", n.line()).expect("writing to String is infallible");
        }
        NodeKind::SyntheticConditionalTest => {
            write!(out, "ternary ?:<br/>L{}", n.line()).expect("writing to String is infallible");
        }
        NodeKind::SyntheticWhileStatementTest => {
            write!(out, "while ()<br/>L{}", n.line()).expect("writing to String is infallible");
        }
        NodeKind::SyntheticDoWhileStatementTest => {
            write!(out, "do while ()<br/>L{}", n.line()).expect("writing to String is infallible");
        }
        NodeKind::SyntheticForStatementHeader
        | NodeKind::SyntheticForInStatementHeader
        | NodeKind::SyntheticForOfStatementHeader => {
            write!(out, "for ()<br/>L{}", n.line()).expect("writing to String is infallible");
        }
        NodeKind::SyntheticBeyondDepth => {
            // The stub stands in for an arbitrary range of source
            // lines that collapsed past the queried depth; printing
            // a single line number here would be misleading, and
            // printing the full range would duplicate the
            // surrounding subgraph's L<x>-<y> label.
            node_head_into(out, n);
        }
        NodeKind::SyntheticModuleSink => {
            out.push_str("module");
        }
        NodeKind::SyntheticImplicitGlobal => {
            node_head_into(out, n);
        }
        _ => {
            // Unused declarations are surfaced via a textual prefix
            // instead of a dashed border. This keeps the visual cue
            // legible even when the node already has another
            // classDef applied (boundary stub, nest level, ...).
            if n.unused() {
                out.push_str("unused ");
            }
            node_head_into(out, n);
            out.push_str("<br/>");
            match n.end_line() {
                Some(end) if end != n.line() => {
                    write!(out, "L{}-{end}", n.line()).expect("writing to String is infallible");
                }
                _ => {
                    write!(out, "L{}", n.line()).expect("writing to String is infallible");
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "node_label_test.rs"]
mod node_label_test;
