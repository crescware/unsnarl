//! Options passed into `Emitter::emit`.
//!
//! Mirrors `EmitOptions` in `ts/src/pipeline/emit/emit-options.ts`.
//! The TS shape carries fields for pruned graphs, root-query
//! resolutions, highlight sets, debug flags, and depths in addition
//! to `prettyJson`. The Rust port grows the struct one field at a
//! time as each emitter starts consuming the corresponding TS
//! field; pruning / depth / highlight land alongside Steps 17–19.

pub struct EmitOptions {
    pub pretty_json: bool,
    /// Annotate node / subgraph labels with the underlying
    /// `NODE_KIND` / `SUBGRAPH_KIND`. Set by the CLI's `--debug`
    /// flag. Only the mermaid emitter honors this today; other
    /// emitters ignore it.
    pub debug: bool,
}
