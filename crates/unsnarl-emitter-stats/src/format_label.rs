//! Formats the third TSV column of a stats row: `path:line [unused ]name`.
//!
//! Mirrors `ts/src/emitter/stats/format-label.ts`. The leading
//! `path:line` is what makes the row clickable in editors that
//! pick up jump-to-source targets; the `unused ` prefix surfaces
//! the IR-level "unused" annotation already attached by the
//! analyzer.

use unsnarl_visual_graph::visual_node::VisualNode;

pub fn format_label(path: &str, n: &VisualNode) -> String {
    let prefix = if n.unused() { "unused " } else { "" };
    format!(
        "{path}:{line} {prefix}{name}",
        line = n.line(),
        name = n.name()
    )
}

#[cfg(test)]
#[path = "format_label_test.rs"]
mod format_label_test;
