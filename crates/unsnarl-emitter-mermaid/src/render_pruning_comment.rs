//! Emits the `%% pruning roots ...` comment line at the top of the
//! diagram when pruning produced a summary.
//!
//! Mirrors `ts/src/emitter/mermaid/render-pruning-comment.ts`. The
//! Rust port walks the same `VisualGraphPruning` shape; the comment
//! is omitted entirely when `graph.pruning` is `None` so the
//! pruning-free Step 14 baselines render byte-identical.

use unsnarl_visual_graph::visual_graph::VisualGraph;

pub fn render_pruning_comment(graph: &VisualGraph, lines: &mut Vec<String>) {
    let Some(pruning) = &graph.pruning else {
        return;
    };
    // Avoid `[ ]` in the comment payload because some Mermaid
    // versions misread a comment line that contains shape-like
    // brackets.
    let summary = pruning
        .roots
        .iter()
        .map(|v| format!("{}={}", v.query, v.matched))
        .collect::<Vec<_>>()
        .join(" ");
    lines.push(format!(
        "  %% pruning roots {summary} ancestors={} descendants={}",
        pruning.ancestors, pruning.descendants
    ));
    for r in &pruning.roots {
        if r.matched == 0 {
            lines.push(format!(
                "  %% pruning warning query {} matched 0 roots",
                r.query
            ));
        }
    }
}

#[cfg(test)]
#[path = "render_pruning_comment_test.rs"]
mod render_pruning_comment_test;
