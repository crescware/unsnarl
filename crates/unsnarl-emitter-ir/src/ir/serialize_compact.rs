//! Compact (single-line) JSON serialization for `SerializedIR`.
//!
//! Only reached when callers pass `pretty_json = false` to
//! `IrEmitter::emit`. The CLI never sets that flag and the parity
//! harness pins it to `true`, so this file is intentionally outside
//! the parity-coverage envelope. Its companion `serialize_pretty.rs`
//! holds the pretty-printed path that parity does cover.

use unsnarl_ir::serialized::SerializedIR;

pub(super) fn serialize(ir: &SerializedIR) -> String {
    serde_json::to_string(ir).expect("SerializedIR is serializable")
}
