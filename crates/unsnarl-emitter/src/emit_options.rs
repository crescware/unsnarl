//! Options passed into `Emitter::emit`.
//!
//! Carries fields for pruned graphs, root-query resolutions,
//! highlight sets, debug flags, and depths alongside `pretty_json`.

use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_visual_graph::highlight::HighlightRunOptions;
use unsnarl_visual_graph::prune::RootQueryResolution;
use unsnarl_visual_graph::visual_graph::VisualGraph;

pub struct EmitOptions {
    pub pretty_json: bool,
    /// Annotate node / subgraph labels with the underlying
    /// `NODE_KIND` / `SUBGRAPH_KIND`. Set by the CLI's `--debug`
    /// flag. Only the mermaid emitter honors this today; other
    /// emitters ignore it.
    pub debug: bool,
    /// When `Some`, every emitter that would otherwise build a
    /// `VisualGraph` from `SerializedIR` uses this graph instead.
    /// Mirrors `opts.prunedGraph` in the TS port: the pipeline runs
    /// `prune_visual_graph` once and hands the result to all
    /// downstream emitters so the JSON / mermaid / stats / markdown
    /// renders stay consistent with each other for the same `-r`
    /// query.
    pub pruned_graph: Option<VisualGraph>,
    /// `LineOrName` disambiguator log produced by
    /// `resolve_ambiguous_queries`. Surfaced by the markdown emitter
    /// in the `## Notice` block and by the CLI's stderr emitter.
    pub resolutions: Option<Vec<RootQueryResolution>>,
    /// Per-`NestingKind` depth ceiling applied at visual-graph build
    /// time. `None` keeps every scope rendered (the TS default when
    /// no `--depth*` flag is given); `Some` collapses scopes whose
    /// recorded `nestingDepths[kind]` strictly exceed the matching
    /// threshold. Mirrors `opts.depths` in the TS port: the markdown
    /// emitter additionally surfaces the chosen depths in its
    /// `## Query` block via `formatDepthQuery`.
    pub depths: Option<NestingDepths>,
    /// Ordered list of `VisualNode` ids the renderer should paint as
    /// "highlighted". `None` means "no highlight". An empty list means
    /// "highlight requested but matched nothing"; the renderer treats
    /// that the same as `None` but the distinction lets the pipeline
    /// produce a stderr warning upstream. Mirrors `opts.highlightIds`
    /// in the TS port, where the underlying `ReadonlySet<string>`
    /// iterates in insertion order; the mermaid renderer's inline
    /// `style` block depends on that order to reproduce the TS
    /// baselines byte-for-byte.
    pub highlight_ids: Option<Vec<String>>,
    /// Original highlight request, propagated so emitters that surface
    /// the CLI form (currently markdown) can reconstruct `-H` /
    /// `--highlight <queries>` in the rendered Query block. `None`
    /// means the highlight flag was not given. Independent from
    /// [`Self::highlight_ids`] because that one can be empty even when
    /// the request was non-empty (the query missed every node in the
    /// pruned graph).
    pub highlight: Option<HighlightRunOptions>,
}
