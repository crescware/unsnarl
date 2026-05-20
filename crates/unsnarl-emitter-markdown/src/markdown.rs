//! `MarkdownEmitter`: wraps the mermaid emitter's output in a
//! Markdown preview document.
//!
//! Mirrors `MarkdownEmitter` in `ts/src/emitter/markdown/markdown.ts`.
//! The TS implementation composes a configured `MermaidEmitter`,
//! strips trailing newlines from both the source and the mermaid
//! render, and assembles the `# <path>` / `## Notice` / `## Input` /
//! `## Query` / `## Mermaid` sections. The Rust port keeps the same
//! composition and section order.
//!
//! `EmitOptions` in this workspace does not yet carry `resolutions`,
//! `prunedGraph`, `depths`, or `highlight` (those fields land in
//! Steps 17–19 alongside their feature implementations), so the
//! `## Notice` block here only surfaces var-detected diagnostics and
//! the `## Query` block is skipped entirely. The TS emitter's
//! corresponding code paths come back online when the option fields
//! land.

use unsnarl_analyzer::format_var_diagnostic;
use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::serialized::SerializedIR;

use crate::code_fence_lang::code_fence_lang;

pub struct MarkdownEmitter {
    mermaid: MermaidEmitter,
}

impl MarkdownEmitter {
    pub const FORMAT: &'static str = "markdown";
    pub const CONTENT_TYPE: &'static str = "text/markdown";
    pub const EXTENSION: &'static str = "md";

    pub fn new(mermaid: MermaidEmitter) -> Self {
        Self { mermaid }
    }
}

impl Emitter for MarkdownEmitter {
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
        let mermaid = strip_trailing_newlines(&self.mermaid.emit(ir, opts));
        let raw = strip_trailing_newlines(&ir.raw);
        let fence = code_fence_lang(ir.source.language);

        let mut lines: Vec<String> = vec![format!("# {}", ir.source.path), String::new()];

        let var_diagnostics: Vec<&_> = ir
            .diagnostics
            .iter()
            .filter(|d| matches!(d.kind, DiagnosticKind::VarDetected))
            .collect();
        if !var_diagnostics.is_empty() {
            lines.push("## Notice".to_string());
            lines.push(String::new());
            lines.push("```".to_string());
            for diagnostic in &var_diagnostics {
                lines.extend(format_var_diagnostic(diagnostic));
            }
            lines.push("```".to_string());
            lines.push(String::new());
        }

        lines.push("## Input".to_string());
        lines.push(String::new());
        lines.push(format!("```{fence}"));
        lines.push(raw);
        lines.push("```".to_string());
        lines.push(String::new());

        lines.push("## Mermaid".to_string());
        lines.push(String::new());
        lines.push("```mermaid".to_string());
        lines.push(mermaid);
        lines.push("```".to_string());
        lines.push(String::new());

        lines.join("\n")
    }
}

fn strip_trailing_newlines(s: &str) -> String {
    s.trim_end_matches('\n').to_string()
}
