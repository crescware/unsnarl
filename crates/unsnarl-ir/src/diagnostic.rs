//! Diagnostic record carried in `SerializedIR.diagnostics`. Ports
//! `ts/src/ir/diagnostic/diagnostic.ts`.
//!
//! Flattened into a single file rather than `diagnostic/diagnostic.rs`
//! to avoid Rust's `module_inception` shape; TS had `diagnostic/` only
//! because the broader `ir/` tree groups files by topic.

use serde::Serialize;

use crate::diagnostic_kind::DiagnosticKind;
use crate::primitive::Span;

#[derive(Serialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
}
