//! Pipeline-level pruning orchestration.
//!
//! Mirrors `ts/src/pipeline/prune/`. The TS file defines
//! `PruningRunOptions` as the inputs the pipeline runner accepts from
//! `runDetailed({ pruning })` and then hands to `pruneVisualGraph`.
//! The Rust port keeps the same shape so the CLI / parity harness can
//! pass through `(roots, descendants, ancestors)` without depending
//! on the inner `unsnarl-visual-graph` types directly.

pub mod pruning_run_options;

pub use pruning_run_options::PruningRunOptions;
