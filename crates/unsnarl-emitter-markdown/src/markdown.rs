//! `MarkdownEmitter`: wraps the mermaid emitter's output in a
//! Markdown preview document.
//!
//! Composes a configured `MermaidEmitter`, strips trailing newlines
//! from both the source and the mermaid render, and assembles the
//! `# <path>` / `## Notice` / `## Input` / `## Query` / `## Mermaid`
//! sections in that order.

use unsnarl_analyzer::format_var_diagnostic;
use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_visual_graph::prune::format_resolution_notice;

use crate::code_fence_lang::code_fence_lang;
use crate::format_depth_query::format_depth_query;
use crate::format_highlight_query::format_highlight_query;
use crate::format_pruning_query::format_pruning_query;

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

        let resolutions = opts.resolutions.as_deref().unwrap_or(&[]);
        let var_diagnostics: Vec<&_> = ir
            .diagnostics
            .iter()
            .filter(|d| matches!(d.kind, DiagnosticKind::VarDetected))
            .collect();
        if !resolutions.is_empty() || !var_diagnostics.is_empty() {
            lines.push("## Notice".to_string());
            lines.push(String::new());
            lines.push("```".to_string());
            for resolution in resolutions {
                lines.extend(format_resolution_notice(resolution));
            }
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

        let pruning = opts.pruned_graph.as_ref().and_then(|g| g.pruning.as_ref());
        let depth_query = format_depth_query(opts.depths.as_ref());
        let highlight = opts.highlight.as_ref();
        if pruning.is_some() || depth_query.is_some() || highlight.is_some() {
            let mut parts: Vec<String> = Vec::new();
            if let Some(p) = pruning {
                parts.push(format_pruning_query(p));
            }
            if let Some(q) = depth_query {
                parts.push(q);
            }
            if let Some(h) = highlight {
                parts.push(format_highlight_query(h));
            }
            lines.push("## Query".to_string());
            lines.push(String::new());
            lines.push("```sh".to_string());
            lines.push(parts.join(" "));
            lines.push("```".to_string());
            lines.push(String::new());
        }

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
