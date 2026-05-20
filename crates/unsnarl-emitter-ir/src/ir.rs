//! `IrEmitter`: renders a `SerializedIR` as JSON.
//!
//! Mirrors `IrEmitter` in `ts/src/emitter/ir/ir.ts`. The output is
//! `JSON.stringify(ir, null, 2)\n` when `pretty_json` is true, and
//! `JSON.stringify(ir)\n` otherwise.

use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_ir::serialized::SerializedIR;

pub struct IrEmitter;

impl IrEmitter {
    pub const FORMAT: &'static str = "ir";
    pub const CONTENT_TYPE: &'static str = "application/json";
    pub const EXTENSION: &'static str = "json";
}

impl Default for IrEmitter {
    fn default() -> Self {
        Self
    }
}

impl Emitter for IrEmitter {
    fn format(&self) -> &'static str {
        Self::FORMAT
    }

    fn content_type(&self) -> &'static str {
        Self::CONTENT_TYPE
    }

    fn extension(&self) -> &'static str {
        Self::EXTENSION
    }

    fn emit(&self, ir: &SerializedIR, opts: &EmitOptions) -> String {
        let text = if opts.pretty_json {
            serde_json::to_string_pretty(ir).expect("SerializedIR is serializable")
        } else {
            serde_json::to_string(ir).expect("SerializedIR is serializable")
        };
        format!("{text}\n")
    }
}
