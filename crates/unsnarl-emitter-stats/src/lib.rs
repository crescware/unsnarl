//! Stats emitter.
//!
//! Mirrors `ts/src/emitter/stats/`. The emitter consumes a
//! `SerializedIR`, builds the same visual graph the mermaid /
//! markdown emitters use, and renders one TSV row per node plus a
//! trailing `<N> total` summary. The output is shell-friendly
//! (sort / awk / editor jump-to-source) and the per-row format is
//! `descendants <TAB> ancestors <TAB> path:line [unused ]name`.

pub mod collect_nodes;
pub mod format_label;
pub mod stats;

pub use stats::StatsEmitter;
