//! Diagnostic categorization.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticKind {
    VarDetected,
    UnresolvedIdentifier,
    ParseError,
}
