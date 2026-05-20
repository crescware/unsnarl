//! `formatPruningQuery`: render the `-r ROOTS -C/-A/-B N` form for
//! the markdown `## Query` section.
//!
//! Mirrors `ts/src/emitter/markdown/format-pruning-query.ts`.

use unsnarl_visual_graph::visual_graph_pruning::VisualGraphPruning;

pub fn format_pruning_query(pruning: &VisualGraphPruning) -> String {
    let roots = pruning
        .roots
        .iter()
        .map(|v| v.query.as_str())
        .collect::<Vec<_>>()
        .join(",");
    if pruning.descendants == pruning.ancestors {
        format!("-r {roots} -C {}", pruning.descendants)
    } else {
        format!(
            "-r {roots} -A {} -B {}",
            pruning.descendants, pruning.ancestors
        )
    }
}
