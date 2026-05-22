//! Pipeline-level pruning orchestration.
//!
//! `PruningRunOptions` is the inputs the pipeline runner accepts
//! and then hands to `prune_visual_graph`. The CLI / parity harness
//! pass through `(roots, descendants, ancestors)` without depending
//! on the inner `unsnarl-visual-graph` types directly.

pub mod pruning_run_options;

pub use pruning_run_options::PruningRunOptions;
