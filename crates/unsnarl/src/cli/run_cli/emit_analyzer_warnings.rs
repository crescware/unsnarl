//! Stderr emitter for analyser diagnostics.
//!
//! Mirrors `ts/src/cli/run-cli/emit-analyzer-warnings.ts`. Only the
//! `VarDetected` kind currently produces stderr output; other
//! diagnostic kinds are propagated through `PipelineRunDetails`
//! for other consumers but ignored here, matching the TS filter.

use std::io::Write;

use unsnarl_analyzer::format_var_diagnostic;
use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::diagnostic_kind::DiagnosticKind;

pub fn emit_analyzer_warnings(diagnostics: &[Diagnostic], stderr: &mut dyn Write) {
    for diagnostic in diagnostics {
        if !matches!(diagnostic.kind, DiagnosticKind::VarDetected) {
            continue;
        }
        let lines = format_var_diagnostic(diagnostic);
        let _ = writeln!(stderr, "{}", lines.join("\n"));
    }
}

#[cfg(test)]
#[path = "emit_analyzer_warnings_test.rs"]
mod emit_analyzer_warnings_test;
