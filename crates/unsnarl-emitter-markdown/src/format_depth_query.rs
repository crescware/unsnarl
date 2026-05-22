//! `format_depth_query`: render the `--depth*` form for the markdown
//! `## Query` section, returning `None` when every kind is still at
//! `DEFAULT_DEPTH` (the user did not narrow anything).
//!
//! The CLI surface exposes only `--depth`, `--depth-function`, and
//! `--depth-block`, so when the seven per-kind values reduce to a
//! `{function, block}` pair this picks the shortest equivalent of
//! those three flags. When the non-function kinds disagree (which is
//! only reachable through the programmatic API, not the CLI) the
//! function delegates to `format_depth_query_per_kind::format`,
//! which lives in its own file so the coverage report can isolate
//! the CLI-unreachable fallback from this CLI-driven entry point.

mod format_depth_query_per_kind;

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

    if !block_uniform {
        return format_depth_query_per_kind::format(depths, function);
    }
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
    Some(format!(
        "--depth-function {} --depth-block {}",
        function.0, block.0
    ))
}

#[cfg(test)]
#[path = "format_depth_query_test.rs"]
mod format_depth_query_test;
