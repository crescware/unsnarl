//! End-to-end pipeline helpers for the CLI and the parity harness.
//!
//! Parse the source with `OxcParser`, analyse it with
//! [`run_analysis`], hand the analysed IR + annotations to
//! [`FlatSerializer`], and render the result with [`IrEmitter`] (or
//! one of the other emitters, with optional visual-graph pruning /
//! highlight applied beforehand).

pub mod highlight;
pub mod plugin;
pub mod prune;

use std::path::Path;

use oxc_allocator::Allocator;
use unsnarl_analyzer::run_analysis;
use unsnarl_emitter::{EmitOptions, Emitter, IRSerializer, SerializeContext, SerializeSourceMeta};
use unsnarl_emitter_ir::{FlatSerializer, IrEmitter};
use unsnarl_emitter_json::JsonEmitter;
use unsnarl_emitter_markdown::MarkdownEmitter;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::ColorTheme;
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_emitter_stats::StatsEmitter;
use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::{
    default_source_type_for, OxcParser, ParseError, ParseOptions, SourceType,
};
use unsnarl_plugin::UnsnarlPlugin;
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

use crate::pipeline::plugin::apply_plugins;
use crate::pipeline::prune::PruningRunOptions;

/// Per-emit run knobs that the emitter helpers consume directly.
/// Optional fields keep individual `emit_*_text` call sites stable
/// when new transforms enter the visual-graph build.
#[derive(Default, Clone, Copy)]
pub struct PipelineRunOptions<'a> {
    pub pruning: Option<&'a PruningRunOptions>,
    pub depths: Option<&'a NestingDepths>,
    pub highlight: Option<&'a HighlightRunOptions>,
    /// Activated plugins to fold over the serialized IR. The slice
    /// is `&[&dyn UnsnarlPlugin]` so callers can pass the activated
    /// set returned by [`plugin::activate`] without taking ownership
    /// of the trait objects.
    pub plugins: &'a [&'a dyn UnsnarlPlugin],
}

/// Map a path's extension to a [`Language`]. `.mjs` / `.cjs` map to
/// `Js` because they are JavaScript at the parser level;
/// module-vs-script is resolved separately via
/// [`source_type_from_path`].
pub fn language_for_path(path: &str) -> Option<Language> {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match ext {
        "ts" => Some(Language::Ts),
        "tsx" => Some(Language::Tsx),
        "jsx" => Some(Language::Jsx),
        "js" | "mjs" | "cjs" => Some(Language::Js),
        _ => None,
    }
}

/// `.mjs` / `.cjs` are spec-pinned to module / script; every other
/// extension falls back to the language-level default.
pub fn source_type_from_path(path: &str, language: Language) -> SourceType {
    if path.ends_with(".mjs") {
        return SourceType::Module;
    }
    if path.ends_with(".cjs") {
        return SourceType::Script;
    }
    default_source_type_for(language)
}

/// Detailed result of a pipeline run.
///
/// The IR path leaves `pruning` / `resolutions` at `None` (those
/// fields belong to the visual-graph-aware emit paths).
pub struct PipelineRunDetails {
    pub text: String,
    pub pruning: Option<Vec<PrunePerQueryDetail>>,
    pub resolutions: Option<Vec<RootQueryResolution>>,
    /// No-match / no-path warnings raised by a `-H <queries>` path /
    /// direction query, surfaced on stderr by the CLI layer. `None` on
    /// every path that does not run the highlight collector.
    pub highlight_warnings: Option<Vec<HighlightWarning>>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Per-query match count surfaced alongside the rendered text:
/// `query` is the raw token the user typed (after `-r` parsing) and
/// `matched` is the number of nodes the prune walk treated as roots
/// for that query.
pub struct PrunePerQueryDetail {
    pub query: String,
    pub matched: u32,
}

/// Run the full parse -> analyse -> serialize -> emit pipeline for the
/// `ir` format and return the rendered text.
///
/// The `code` slice is owned by the caller; the AST and IR are built
/// inside this function and dropped before it returns, so no oxc
/// lifetime escapes to the caller.
pub fn emit_ir_text(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    plugins: &[&dyn UnsnarlPlugin],
) -> Result<String, ParseError> {
    emit_ir_detailed(code, source_path, language, pretty_json, plugins).map(|d| d.text)
}

/// Detailed variant of [`emit_ir_text`]. Returns the rendered text
/// together with the analyser diagnostics so the CLI orchestration
/// can emit `var`-detected warnings on stderr. The IR branch leaves
/// `pruning` / `resolutions` at `None`.
pub fn emit_ir_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    plugins: &[&dyn UnsnarlPlugin],
) -> Result<PipelineRunDetails, ParseError> {
    let serialized = apply_plugins(serialize_ir(code, source_path, language)?, plugins);
    let diagnostics = serialized.diagnostics.clone();
    let text = {
        let _span = unsnarl_instrumentation::span!("emit", format = "ir");
        IrEmitter.emit(
            &serialized,
            &EmitOptions {
                pretty_json,
                debug: false,
                pruned_graph: None,
                resolutions: None,
                depths: None,
                highlight_ids: None,
                highlight_point_ids: None,
                highlight: None,
            },
        )
    };
    Ok(PipelineRunDetails {
        text,
        pruning: None,
        resolutions: None,
        highlight_warnings: None,
        diagnostics,
    })
}

/// Same as [`emit_ir_text`] but routes the parsed IR through
/// [`JsonEmitter`], which builds a `VisualGraph` and serialises it
/// as JSON. Used by the `-f json` CLI handler and the parity
/// harness's `expected.json` comparison.
pub fn emit_json_text(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_json_detailed(code, source_path, language, pretty_json, run).map(|d| d.text)
}

/// Detailed variant of [`emit_json_text`]. Returns text + warnings
/// for the CLI orchestration to surface on stderr.
pub fn emit_json_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    emit_pruning_aware_with(
        code,
        source_path,
        language,
        &JsonEmitter,
        run,
        EmitOptionsBase {
            pretty_json,
            debug: false,
        },
    )
}

/// Same as [`emit_ir_text`] but routes the parsed IR through
/// [`MermaidEmitter`]. The strategy / theme decisions are made by
/// the caller (CLI flags `--mermaid-renderer` / `--color-theme`)
/// rather than baked into the pipeline.
pub fn emit_mermaid_text(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_mermaid_detailed(code, source_path, language, strategy, theme, debug, run).map(|d| d.text)
}

/// Detailed variant of [`emit_mermaid_text`].
pub fn emit_mermaid_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    let emitter = MermaidEmitter::new(strategy, theme);
    emit_pruning_aware_with(
        code,
        source_path,
        language,
        &emitter,
        run,
        EmitOptionsBase {
            pretty_json: false,
            debug,
        },
    )
}

/// Same as [`emit_ir_text`] but routes the parsed IR through
/// [`MarkdownEmitter`]. The markdown emitter composes a configured
/// [`MermaidEmitter`] so the caller picks the renderer / theme on
/// behalf of the embedded `## Mermaid` block — the same configured
/// `MermaidEmitter` instance that `-f mermaid` uses directly.
pub fn emit_markdown_text(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_markdown_detailed(code, source_path, language, strategy, theme, debug, run).map(|d| d.text)
}

/// Detailed variant of [`emit_markdown_text`].
pub fn emit_markdown_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    let mermaid = MermaidEmitter::new(strategy, theme);
    let emitter = MarkdownEmitter::new(mermaid);
    emit_pruning_aware_with(
        code,
        source_path,
        language,
        &emitter,
        run,
        EmitOptionsBase {
            pretty_json: false,
            debug,
        },
    )
}

/// Same as [`emit_ir_text`] but routes the parsed IR through
/// [`StatsEmitter`], which builds a `VisualGraph` and renders a
/// wc-like TSV table of per-node edge counts. Used by the `-f stats`
/// CLI handler and the parity harness's `expected.stats` comparison.
pub fn emit_stats_text(
    code: &str,
    source_path: &str,
    language: Language,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_stats_detailed(code, source_path, language, run).map(|d| d.text)
}

/// Detailed variant of [`emit_stats_text`].
pub fn emit_stats_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    emit_pruning_aware_with(
        code,
        source_path,
        language,
        &StatsEmitter,
        run,
        EmitOptionsBase {
            pretty_json: false,
            debug: false,
        },
    )
}

#[derive(Clone, Copy)]
struct EmitOptionsBase {
    pretty_json: bool,
    debug: bool,
}

/// Output of the pre-emit visual-graph orchestration: the pruned
/// graph (when `-r` was given), the `LineOrName` disambiguation log
/// (now also fed by `-H`'s `..` endpoints), the per-query match counts
/// (so `emit-pruning-warnings` can flag `matched === 0` queries), the
/// highlight id list (when `-H` was given) plus its point subset, the
/// highlight no-match / no-path warnings, and the kept-as-given
/// highlight request so the markdown emitter can reconstruct `-H` in
/// the Query block.
struct PreparedEmit {
    pruned_graph: Option<VisualGraph>,
    resolutions: Option<Vec<RootQueryResolution>>,
    per_query: Option<Vec<PrunePerQueryDetail>>,
    highlight_ids: Option<Vec<String>>,
    highlight_point_ids: Option<Vec<String>>,
    highlight_warnings: Option<Vec<HighlightWarning>>,
    highlight: Option<HighlightRunOptions>,
}

/// Build the visual graph once and run pruning / highlight on it.
///
/// Pruning runs first (since `-H` in roots mode follows the prune
/// walk's root ids); highlight then resolves against the working
/// graph — the pruned one when pruning is active, the base one
/// otherwise.
fn prepare_emit(
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

fn emit_pruning_aware_with(
    code: &str,
    source_path: &str,
    language: Language,
    emitter: &dyn Emitter,
    run: PipelineRunOptions<'_>,
    base_opts: EmitOptionsBase,
) -> Result<PipelineRunDetails, ParseError> {
    let serialized = apply_plugins(serialize_ir(code, source_path, language)?, run.plugins);
    let diagnostics = serialized.diagnostics.clone();
    let needs_visual =
        run.pruning.map(|p| !p.roots.is_empty()).unwrap_or(false) || run.highlight.is_some();
    let prepared = if needs_visual {
        prepare_emit(&serialized, run.pruning, run.depths, run.highlight)
    } else {
        PreparedEmit {
            pruned_graph: None,
            resolutions: None,
            per_query: None,
            highlight_ids: None,
            highlight_point_ids: None,
            highlight_warnings: None,
            highlight: None,
        }
    };
    let resolutions_for_details = prepared.resolutions.clone();
    let per_query_for_details = prepared.per_query;
    let highlight_warnings_for_details = prepared.highlight_warnings;
    let text = {
        let _span = unsnarl_instrumentation::span!("emit");
        emitter.emit(
            &serialized,
            &EmitOptions {
                pretty_json: base_opts.pretty_json,
                debug: base_opts.debug,
                pruned_graph: prepared.pruned_graph,
                resolutions: prepared.resolutions,
                depths: run.depths.cloned(),
                highlight_ids: prepared.highlight_ids,
                highlight_point_ids: prepared.highlight_point_ids,
                highlight: prepared.highlight,
            },
        )
    };
    Ok(PipelineRunDetails {
        text,
        pruning: per_query_for_details,
        resolutions: resolutions_for_details,
        highlight_warnings: highlight_warnings_for_details,
        diagnostics,
    })
}

fn serialize_ir(
    code: &str,
    source_path: &str,
    language: Language,
) -> Result<SerializedIR, ParseError> {
    let source_type = source_type_from_path(source_path, language);
    let allocator = Allocator::default();
    let parser = OxcParser;
    let parsed = {
        let _span = unsnarl_instrumentation::span!("parse", bytes = code.len());
        parser.parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: source_path.to_string(),
                source_type,
            },
        )?
    };
    let analyzed = {
        let _span = unsnarl_instrumentation::span!("analyze");
        run_analysis(&parsed.program, parsed.source_type, language, parsed.raw)
    };
    let serializer = FlatSerializer;
    let ctx = SerializeContext {
        arena: &analyzed.arena,
        root_scope: analyzed.root_scope,
        annotations: &analyzed.annotations,
        source: SerializeSourceMeta {
            path: source_path.to_string(),
            language,
        },
        diagnostics: &analyzed.diagnostics,
        raw: analyzed.raw,
    };
    let _span = unsnarl_instrumentation::span!("serialize");
    let ir = serializer.serialize(&ctx);
    tracing::info!(
        scopes = ir.scopes.len(),
        variables = ir.variables.len(),
        references = ir.references.len(),
        diagnostics = ir.diagnostics.len(),
        "ir counts",
    );
    Ok(ir)
}

#[cfg(test)]
#[path = "pipeline_test.rs"]
mod pipeline_test;
