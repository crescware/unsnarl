//! Stderr emitter for `LineOrName` resolution notices.
//!
//! Mirrors `ts/src/cli/run-cli/emit-resolution-notices.ts`. The
//! pipeline runner exposes `PipelineRunDetails.resolutions` as the
//! source of truth; this module formats one three-line notice per
//! resolution to the supplied stderr writer, using
//! [`format_resolution_notice`] so the wording stays in lock-step
//! with the markdown emitter's Notice section.

use std::io::Write;

use unsnarl_visual_graph::prune::{format_resolution_notice, RootQueryResolution};

pub fn emit_resolution_notices(
    resolutions: Option<&[RootQueryResolution]>,
    stderr: &mut dyn Write,
) {
    let Some(resolutions) = resolutions else {
        return;
    };
    for r in resolutions {
        let lines = format_resolution_notice(r);
        let _ = writeln!(stderr, "{}", lines.join("\n"));
    }
}

#[cfg(test)]
#[path = "emit_resolution_notices_test.rs"]
mod emit_resolution_notices_test;
