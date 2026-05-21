//! `JsonEmitter`: renders a `SerializedIR` as a JSON-encoded
//! `VisualGraph`.
//!
//! Mirrors `JsonEmitter` in `ts/src/emitter/json/json.ts`. The
//! visual-graph build runs once per emit; the resulting `VisualGraph`
//! is serialized with `serialize_pretty::serialize` when
//! `pretty_json` is true, otherwise with `serialize_compact::serialize`.
//! The two serialisations live in sibling files so the coverage
//! report can show parity exercising only the pretty path (the CLI
//! never sets `pretty_json = false`). The returned text always ends
//! in a trailing newline (matches the TS `${text}\n`).

mod serialize_compact;
mod serialize_pretty;

use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_visual_graph::builder::build_visual_graph::build_visual_graph;
use unsnarl_visual_graph::builder::context::BuildVisualGraphOptions;

pub struct JsonEmitter;

impl JsonEmitter {
    pub const FORMAT: &'static str = "json";
    pub const CONTENT_TYPE: &'static str = "application/json";
    pub const EXTENSION: &'static str = "json";
}

impl Default for JsonEmitter {
    fn default() -> Self {
        Self
    }
}

impl Emitter for JsonEmitter {
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
        let built;
        let graph = if let Some(pruned) = &opts.pruned_graph {
            pruned
        } else {
            built = build_visual_graph(
                ir,
                &BuildVisualGraphOptions {
                    depths: opts.depths.clone(),
                },
            );
            &built
        };
        let text = if opts.pretty_json {
            serialize_pretty::serialize(graph)
        } else {
            serialize_compact::serialize(graph)
        };
        format!("{text}\n")
    }
}
