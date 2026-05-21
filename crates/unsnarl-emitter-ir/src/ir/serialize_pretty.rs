//! Pretty-printed JSON serialization for `SerializedIR`.
//!
//! The CLI (and therefore the parity harness via `emit_ir_text` →
//! `IrEmitter::emit`) always renders IR JSON with
//! `pretty_json = true`, so this path is the one the parity sweep
//! exercises. The compact counterpart lives in
//! `serialize_compact.rs` and is only reached when callers flip the
//! flag programmatically; keeping the two serialisations in separate
//! files makes that asymmetry visible from the coverage report.

use unsnarl_ir::serialized::SerializedIR;

pub(super) fn serialize(ir: &SerializedIR) -> String {
    serde_json::to_string_pretty(ir).expect("SerializedIR is serializable")
}
