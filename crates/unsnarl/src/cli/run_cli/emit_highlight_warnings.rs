//! Stderr warning emitter for `-H` path / direction queries that
//! selected nothing.
//!
//! The pipeline runner exposes `PipelineRunDetails.highlight_warnings`
//! as the source of truth (the reachability collector returns the
//! warnings rather than printing them, so this library-free CLI layer
//! owns all stderr output). One line per warning, mirroring the
//! `uns: warning: ...` shape used by [`super::emit_pruning_warnings`].

use std::io::Write;

use unsnarl_visual_graph::highlight::HighlightWarning;

pub fn emit_highlight_warnings(warnings: Option<&[HighlightWarning]>, stderr: &mut dyn Write) {
    let Some(warnings) = warnings else { return };
    for w in warnings {
        let _ = match w {
            HighlightWarning::NoMatch { raw } => writeln!(
                stderr,
                "uns: warning: highlight query '{raw}' matched no node"
            ),
            HighlightWarning::NoPath { raw } => writeln!(
                stderr,
                "uns: warning: highlight query '{raw}' has no connecting path"
            ),
        };
    }
}

#[cfg(test)]
#[path = "emit_highlight_warnings_test.rs"]
mod emit_highlight_warnings_test;
