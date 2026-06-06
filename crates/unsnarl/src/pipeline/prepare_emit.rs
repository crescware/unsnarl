//! Build the visual graph once and run pruning / highlight on it,
//! producing the [`PreparedEmit`] inputs the emitters consume.

use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_root_query::ParsedRootQuery;
use unsnarl_visual_graph::builder::build_visual_graph::build_visual_graph;
use unsnarl_visual_graph::builder::context::BuildVisualGraphOptions;
use unsnarl_visual_graph::highlight::{
    collect_highlight_path_ids, HighlightRunOptions, HighlightSelection, HighlightWarning,
};
use unsnarl_visual_graph::prune::{
    prune_visual_graph, resolve_ambiguous_queries, PruneOptions, RootQueryResolution,
};
use unsnarl_visual_graph::visual_graph::VisualGraph;

use crate::pipeline::prune::PruningRunOptions;

use super::PrunePerQueryDetail;

/// Output of the pre-emit visual-graph orchestration: the pruned
/// graph (when `-r` was given), the `LineOrName` disambiguation log
/// (now also fed by `-H`'s `..` endpoints), the per-query match counts
/// (so `emit-pruning-warnings` can flag `matched === 0` queries), the
/// highlight id list (when `-H` was given) plus its point subset, the
/// highlight no-match / no-path warnings, and the kept-as-given
/// highlight request so the markdown emitter can reconstruct `-H` in
/// the Query block.
pub(super) struct PreparedEmit {
    pub(super) pruned_graph: Option<VisualGraph>,
    pub(super) resolutions: Option<Vec<RootQueryResolution>>,
    pub(super) per_query: Option<Vec<PrunePerQueryDetail>>,
    pub(super) highlight_ids: Option<Vec<String>>,
    pub(super) highlight_point_ids: Option<Vec<String>>,
    pub(super) highlight_warnings: Option<Vec<HighlightWarning>>,
    pub(super) highlight: Option<HighlightRunOptions>,
}

/// Build the visual graph once and run pruning / highlight on it.
///
/// Pruning runs first (since `-H` in roots mode follows the prune
/// walk's root ids); highlight then resolves against the working
/// graph — the pruned one when pruning is active, the base one
/// otherwise.
pub(super) fn prepare_emit(
    ir: &SerializedIR,
    pruning: Option<&PruningRunOptions>,
    depths: Option<&NestingDepths>,
    highlight: Option<&HighlightRunOptions>,
) -> PreparedEmit {
    let base = {
        let _span = unsnarl_instrumentation::span!("build_visual_graph");
        let graph = build_visual_graph(
            ir,
            &BuildVisualGraphOptions {
                depths: depths.cloned(),
            },
        );
        tracing::info!(
            elements = graph.elements.len(),
            edges = graph.edges.len(),
            boundary_edges = graph.boundary_edges.len(),
            "visual graph counts",
        );
        graph
    };

    let mut pruned_graph: Option<VisualGraph> = None;
    let mut resolutions_out: Option<Vec<RootQueryResolution>> = None;
    let mut per_query_out: Option<Vec<PrunePerQueryDetail>> = None;
    let mut prune_root_ids: Option<Vec<String>> = None;
    if let Some(p) = pruning {
        if !p.roots.is_empty() {
            let _span = unsnarl_instrumentation::span!("prune", roots = p.roots.len());
            let resolved = resolve_ambiguous_queries(&base, &p.roots);
            let result = prune_visual_graph(
                &base,
                &PruneOptions {
                    roots: resolved.resolved,
                    descendants: p.descendants,
                    ancestors: p.ancestors,
                },
            );
            per_query_out = Some(
                result
                    .per_query
                    .iter()
                    .map(|m| PrunePerQueryDetail {
                        query: raw_root_query(&m.query).to_string(),
                        matched: m.matched,
                    })
                    .collect(),
            );
            prune_root_ids = Some(result.root_ids);
            tracing::info!(
                kept_elements = result.graph.elements.len(),
                kept_edges = result.graph.edges.len(),
                "pruned graph counts",
            );
            pruned_graph = Some(result.graph);
            resolutions_out = Some(resolved.resolutions);
        }
    }

    let mut highlight_ids: Option<Vec<String>> = None;
    let mut highlight_point_ids: Option<Vec<String>> = None;
    let mut highlight_warnings: Option<Vec<HighlightWarning>> = None;
    if let Some(h) = highlight {
        let _span = unsnarl_instrumentation::span!("highlight");
        let sel = match h {
            // Roots mode mirrors `-r`'s match set verbatim, so it inherits
            // `NAME_QUERY_EXCLUDED` for bare name queries (so `-r counter`
            // and `-r counter -H` exclude the same use-site kinds). When
            // `-r` was not given the prune root set is empty — paint
            // nothing. Roots highlight is a point treatment, so every id
            // is also a point id (radius-1 edge rule); it never resolves
            // `..` endpoints or raises path warnings.
            HighlightRunOptions::Roots => {
                let ids = prune_root_ids.clone().unwrap_or_default();
                HighlightSelection {
                    point_ids: ids.clone(),
                    ids,
                    resolutions: Vec::new(),
                    warnings: Vec::new(),
                }
            }
            // Queries mode (`-H <raw>`) uses the looser highlight matcher
            // so explicit highlight queries paint every occurrence of the
            // identifier. The path / direction reachability collector
            // (issue #90) handles point, `a..b`, and `a..+a/+b/+c` shapes:
            // it batch-resolves each endpoint's `LineOrName` ambiguity,
            // reports which ids are point hits (vs reachability hits), and
            // returns no-match / no-path warnings for the CLI to print.
            HighlightRunOptions::Queries(queries) => {
                let working = pruned_graph.as_ref().unwrap_or(&base);
                collect_highlight_path_ids(working, queries)
            }
        };
        // Append the `..` endpoints' `LineOrName` resolutions after any
        // pruning resolutions, so both the markdown Notice block and the
        // stderr notice list a `10:foo` endpoint's line-vs-name decision.
        if !sel.resolutions.is_empty() {
            resolutions_out
                .get_or_insert_with(Vec::new)
                .extend(sel.resolutions);
        }
        highlight_warnings = Some(sel.warnings);
        highlight_point_ids = Some(sel.point_ids);
        highlight_ids = Some(sel.ids);
    }

    PreparedEmit {
        pruned_graph,
        resolutions: resolutions_out,
        per_query: per_query_out,
        highlight_ids,
        highlight_point_ids,
        highlight_warnings,
        highlight: highlight.cloned(),
    }
}

/// Extract the user-typed token from a [`ParsedRootQuery`]. Every
/// variant carries the raw string so this is just a structural
/// projection.
fn raw_root_query(q: &ParsedRootQuery) -> &str {
    match q {
        ParsedRootQuery::Line { raw, .. }
        | ParsedRootQuery::LineName { raw, .. }
        | ParsedRootQuery::Range { raw, .. }
        | ParsedRootQuery::RangeName { raw, .. }
        | ParsedRootQuery::Name { raw, .. }
        | ParsedRootQuery::LineOrName { raw, .. } => raw,
    }
}
