//! Pruning inputs the pipeline runner accepts.
//!
//! Mirrors `ts/src/pipeline/prune/pruning-run-options.ts`. The CLI's
//! `-r/--roots`, `-A`, `-B`, `-C` flags fold into this struct before
//! being handed to
//! [`prune_visual_graph`](unsnarl_visual_graph::prune::prune_visual_graph).

use unsnarl_root_query::ParsedRootQuery;

pub struct PruningRunOptions {
    pub roots: Vec<ParsedRootQuery>,
    pub descendants: u32,
    pub ancestors: u32,
}
