//! Options passed into `Emitter::emit`.
//!
//! Mirrors `EmitOptions` in `ts/src/pipeline/emit/emit-options.ts`.
//! The TS shape carries fields for pruned graphs, root-query
//! resolutions, highlight sets, debug flags, and depths in addition
//! to `prettyJson`. The Rust port grows the struct one field at a
//! time as each emitter starts consuming the corresponding TS
//! field; pruning lands in Step 17, depth / highlight follow in
//! Steps 18–19.

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
    /// in the `## Notice` block (and by the CLI's stderr emitter
    /// once Step 21 lands).
    pub resolutions: Option<Vec<RootQueryResolution>>,
}
