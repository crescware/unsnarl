//! Lines the var-detected diagnostic consists of.
//!
//! The same wording is shared between the stderr emitter and the
//! markdown Notice section so both stay in lock-step.

use unsnarl_ir::diagnostic::Diagnostic;

pub fn format_var_diagnostic(diagnostic: &Diagnostic) -> Vec<String> {
    vec![format!(
        "uns: warning: L{}:{}: {}",
        diagnostic.span.line.0, diagnostic.span.column.0, diagnostic.message
    )]
}

#[cfg(test)]
#[path = "format_var_diagnostic_test.rs"]
mod format_var_diagnostic_test;
