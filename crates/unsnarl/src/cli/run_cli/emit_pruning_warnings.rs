//! Stderr warning emitter for `-r` queries that matched zero nodes.
//!
//! Mirrors `ts/src/cli/run-cli/emit-pruning-warnings.ts`. The
//! pipeline runner exposes `PipelineRunDetails.pruning` as the
//! source of truth; this module formats one warning line per
//! zero-match entry to the supplied stderr writer.

use std::io::Write;

use crate::pipeline::PrunePerQueryDetail;

pub fn emit_pruning_warnings(pruning: Option<&[PrunePerQueryDetail]>, stderr: &mut dyn Write) {
    let Some(pruning) = pruning else { return };
    for r in pruning {
        if r.matched == 0 {
            let _ = writeln!(stderr, "uns: warning: query '{}' matched 0 roots", r.query);
        }
    }
}

#[cfg(test)]
#[path = "emit_pruning_warnings_test.rs"]
mod emit_pruning_warnings_test;
