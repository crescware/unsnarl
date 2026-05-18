//! Diagnostic record carried in `SerializedIR.diagnostics`.
//!
//! Flattened into a single file (rather than `diagnostic/diagnostic.rs`)
//! to avoid Rust's `module_inception` shape; the topic doesn't fan out
//! into multiple files yet.

use serde::Serialize;

use crate::diagnostic_kind::DiagnosticKind;
use crate::primitive::Span;

#[derive(Serialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
}
