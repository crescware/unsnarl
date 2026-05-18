//! Diagnostic categorization. Ports `ts/src/analyzer/diagnostic-kind.ts`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticKind {
    VarDetected,
    UnresolvedIdentifier,
    ParseError,
}
