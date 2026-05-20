//! Builds a `L<start>` or `L<start>-<end>` label fragment for a
//! subgraph's source-line range.
//!
//! Mirrors `ts/src/emitter/mermaid/line-range-label.ts`.

use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

pub fn line_range_label(sg: &VisualSubgraph) -> String {
    let line = sg.line();
    match sg.end_line() {
        Some(end) if end != line => format!("L{line}-{end}"),
        _ => format!("L{line}"),
    }
}
