//! Translate the parsed CLI [`Args`] into the pipeline's run options
//! (depth / highlight / pruning).

use unsnarl_emitter::DEFAULT_DEPTH;
use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_visual_graph::highlight::HighlightRunOptions;

use crate::cli::args::{Args, Highlight};
use crate::pipeline::prune::PruningRunOptions;

/// Default generations used when the user gives `-r/--roots` but no
/// `-A`/`-B`/`-C`.
pub(super) const DEFAULT_GENERATIONS: u32 = 10;

/// Translate the CLI's `--depth` / `--depth-function` / `--depth-block`
/// flags into a [`NestingDepths`]. `--depth <N>` seeds both axes,
/// then `--depth-function` / `--depth-block` override their
/// respective halves. Unset fields fall back to [`DEFAULT_DEPTH`].
pub(super) fn depths_from_args(args: &Args) -> NestingDepths {
    let general = args.depth.unwrap_or(DEFAULT_DEPTH);
    let function = args.depth_function.unwrap_or(general);
    let block = args.depth_block.unwrap_or(general);
    NestingDepths {
        function,
        r#if: block,
        r#for: block,
        r#while: block,
        switch: block,
        try_catch_finally: block,
        block,
    }
}

/// Translate the CLI's `-H` / `--highlight` flag into the pipeline's
/// [`HighlightRunOptions`]: `Highlight::Absent` -> `None`,
/// `Highlight::NoValue` -> `Roots` (the highlight follows
/// `-r/--roots`), `Highlight::Value(queries)` -> `Queries(queries)`.
pub(super) fn highlight_from_args(args: &Args) -> Option<HighlightRunOptions> {
    match &args.highlight {
        Highlight::Absent => None,
        Highlight::NoValue => Some(HighlightRunOptions::Roots),
        Highlight::Value(queries) => Some(HighlightRunOptions::Queries(queries.clone())),
    }
}

/// Translate the CLI's `-r/-A/-B/-C` flags into the pipeline's
/// [`PruningRunOptions`]. Returns `None` when no `-r` queries are
/// present so the pipeline skips the prune step entirely.
pub(super) fn pruning_from_args(args: &Args) -> Option<PruningRunOptions> {
    if args.roots.is_empty() {
        return None;
    }
    let no_flag = args.descendants.is_none() && args.ancestors.is_none() && args.context.is_none();
    // grep -A/-B semantics: an explicit -A says "I asked for
    // descendants only," so the unspecified side falls to 0 instead
    // of the symmetric DEFAULT. -C still fills in for whichever
    // side is unspecified.
    let fallback = if no_flag {
        DEFAULT_GENERATIONS
    } else {
        args.context.map(|g| g.0).unwrap_or(0)
    };
    let descendants = args.descendants.map(|g| g.0).unwrap_or(fallback);
    let ancestors = args.ancestors.map(|g| g.0).unwrap_or(fallback);
    let roots: Vec<_> = args.roots.iter().map(clone_parsed_root_query).collect();
    Some(PruningRunOptions {
        roots,
        descendants,
        ancestors,
    })
}

fn clone_parsed_root_query(
    q: &unsnarl_root_query::ParsedRootQuery,
) -> unsnarl_root_query::ParsedRootQuery {
    use unsnarl_root_query::ParsedRootQuery;
    match q {
        ParsedRootQuery::Line { line, raw } => ParsedRootQuery::Line {
            line: *line,
            raw: raw.clone(),
        },
        ParsedRootQuery::LineName { line, name, raw } => ParsedRootQuery::LineName {
            line: *line,
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::Range { start, end, raw } => ParsedRootQuery::Range {
            start: *start,
            end: *end,
            raw: raw.clone(),
        },
        ParsedRootQuery::RangeName {
            start,
            end,
            name,
            raw,
        } => ParsedRootQuery::RangeName {
            start: *start,
            end: *end,
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::Name { name, raw } => ParsedRootQuery::Name {
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::LineOrName { line, name, raw } => ParsedRootQuery::LineOrName {
            line: *line,
            name: name.clone(),
            raw: raw.clone(),
        },
    }
}
