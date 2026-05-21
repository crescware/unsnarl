//! Pretty-printed JSON serialization for `VisualGraph`.
//!
//! The CLI (and therefore the parity harness via `emit_json_text` →
//! `emit_pruning_aware_with` → `JsonEmitter::emit`) always renders
//! JSON with `pretty_json = true`, so this path is the one the
//! parity sweep exercises. The compact counterpart lives in
//! `serialize_compact.rs` and is only reached when callers flip the
//! flag programmatically; keeping the two serialisations in separate
//! files makes that asymmetry visible from the coverage report.

use unsnarl_visual_graph::VisualGraph;

pub(crate) fn serialize(graph: &VisualGraph) -> String {
    serde_json::to_string_pretty(graph).expect("VisualGraph is serializable")
}
