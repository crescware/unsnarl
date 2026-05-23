//! Builds a `L<start>` or `L<start>-<end>` label fragment for a
//! subgraph's source-line range.

use std::fmt::Write as _;

use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

pub fn line_range_label(sg: &VisualSubgraph) -> String {
    let mut out = String::new();
    line_range_label_into(&mut out, sg);
    out
}

/// Destination-arg variant of [`line_range_label`]: writes the
/// `L<start>` / `L<start>-<end>` fragment directly into `out` so the
/// caller building a larger subgraph label avoids the per-subgraph
/// `String` allocation that the owned-return form would force.
pub fn line_range_label_into(out: &mut String, sg: &VisualSubgraph) {
    let line = sg.line();
    match sg.end_line() {
        Some(end) if end != line => {
            write!(out, "L{line}-{end}").expect("writing to String is infallible");
        }
        _ => {
            write!(out, "L{line}").expect("writing to String is infallible");
        }
    }
}

#[cfg(test)]
#[path = "line_range_label_test.rs"]
mod line_range_label_test;
