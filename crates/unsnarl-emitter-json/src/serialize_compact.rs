//! Compact (single-line) JSON serialization for `VisualGraph`.
//!
//! Only reached when callers pass `pretty_json = false` to
//! `JsonEmitter::emit`. The CLI never sets that flag and the parity
//! harness pins it to `true`, so this file is intentionally outside
//! the parity-coverage envelope. Its companion `serialize_pretty.rs`
//! holds the pretty-printed path that parity does cover.

use unsnarl_visual_graph::VisualGraph;

pub(crate) fn serialize(graph: &VisualGraph) -> String {
    serde_json::to_string(graph).expect("VisualGraph is serializable")
}
