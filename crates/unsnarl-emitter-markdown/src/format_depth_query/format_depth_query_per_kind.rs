//! Per-kind fallback for `format_depth_query`.
//!
//! The fallback renders a `# nesting depth: kind=value ...` sh-style
//! comment when the seven per-kind values in `NestingDepths` cannot
//! be collapsed to a `{function, block}` pair. That shape is only
//! reachable through the programmatic API — the CLI surface exposes
//! only `--depth`, `--depth-function`, and `--depth-block`, so the
//! parity harness (which drives the pipeline through the same option
//! set the CLI offers) never asks for this branch.
//!
//! The fallback therefore lives in its own file so the coverage
//! report can mark it as zero from a parity-only sweep without
//! pulling down the surrounding CLI-reachable formatter in
//! `format_depth_query.rs`. Its behaviour is still covered by the
//! sibling `format_depth_query_test.rs` unit tests under a full
//! workspace test run.

use unsnarl_emitter::DEFAULT_DEPTH;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};

pub(super) fn format(depths: &NestingDepths, function: NestingDepth) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if function != DEFAULT_DEPTH {
        parts.push(format!("function={}", function.0));
    }
    let labeled: [(&str, NestingDepth); 6] = [
        ("if", depths.r#if),
        ("for", depths.r#for),
        ("while", depths.r#while),
        ("switch", depths.switch),
        ("try-catch-finally", depths.try_catch_finally),
        ("block", depths.block),
    ];
    for (label, value) in labeled {
        if value != DEFAULT_DEPTH {
            parts.push(format!("{label}={}", value.0));
        }
    }
    if parts.is_empty() {
        return None;
    }
    Some(format!("# nesting depth: {}", parts.join(" ")))
}
