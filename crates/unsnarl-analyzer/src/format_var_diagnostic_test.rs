use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::primitive::{SourceColumn, SourceLine, SourceOffset, Span};

use super::format_var_diagnostic;

#[test]
fn renders_single_line_warning_with_line_column_and_message() {
    let diagnostic = Diagnostic {
        kind: DiagnosticKind::VarDetected,
        message: "var declaration detected; rendered as node only (no edges).".to_string(),
        span: Span {
            line: SourceLine(3),
            column: SourceColumn(0),
            offset: SourceOffset(11),
        },
    };
    let lines = format_var_diagnostic(&diagnostic);
    assert_eq!(
        lines,
        vec![
            "uns: warning: L3:0: var declaration detected; rendered as node only (no edges)."
                .to_string()
        ]
    );
}
