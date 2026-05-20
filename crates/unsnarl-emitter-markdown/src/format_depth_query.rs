//! `formatDepthQuery`: render the `--depth*` form for the markdown
//! `## Query` section, returning `None` when every kind is still at
//! `DEFAULT_DEPTH` (the user did not narrow anything).
//!
//! Mirrors `ts/src/emitter/markdown/format-depth-query.ts`. The CLI
//! surface exposes only `--depth`, `--depth-function`, and
//! `--depth-block`, so when the seven per-kind values reduce to a
//! `{function, block}` pair this picks the shortest equivalent of
//! those three flags. When the non-function kinds disagree (which is
//! only reachable through the programmatic API, not the CLI) the
//! function falls back to a `# nesting depth: kind=value ...`
//! sh-style comment so the snapshot still records exactly what was
//! applied.

use unsnarl_emitter::DEFAULT_DEPTH;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};

pub fn format_depth_query(depths: Option<&NestingDepths>) -> Option<String> {
    let depths = depths?;
    let function = depths.function;
    let block_values: [NestingDepth; 6] = [
        depths.r#if,
        depths.r#for,
        depths.r#while,
        depths.switch,
        depths.try_catch_finally,
        depths.block,
    ];
    let first_block = block_values[0];
    let block_uniform = block_values.iter().all(|v| *v == first_block);

    if block_uniform {
        let block = first_block;
        if function == DEFAULT_DEPTH && block == DEFAULT_DEPTH {
            return None;
        }
        if function == block {
            return Some(format!("--depth {}", function.0));
        }
        if function == DEFAULT_DEPTH {
            return Some(format!("--depth-block {}", block.0));
        }
        if block == DEFAULT_DEPTH {
            return Some(format!("--depth-function {}", function.0));
        }
        return Some(format!(
            "--depth-function {} --depth-block {}",
            function.0, block.0
        ));
    }

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

#[cfg(test)]
#[path = "format_depth_query_test.rs"]
mod format_depth_query_test;
