use super::*;
use unsnarl_ir::primitive::{SourceColumn, SourceLine, SourceOffset, Span};

fn capture(diagnostics: &[Diagnostic]) -> String {
    let mut buf = Vec::new();
    emit_analyzer_warnings(diagnostics, &mut buf);
    String::from_utf8(buf).expect("stderr should be valid UTF-8")
}

fn span_at(line: u32, column: u32) -> Span {
    Span {
        line: SourceLine(line),
        column: SourceColumn(column),
        offset: SourceOffset(0),
    }
}

#[test]
fn writes_nothing_when_diagnostics_is_empty() {
    assert_eq!(capture(&[]), "");
}

#[test]
fn writes_nothing_when_no_diagnostic_is_var_detected() {
    let diagnostics = vec![
        Diagnostic {
            kind: DiagnosticKind::UnresolvedIdentifier,
            message: "ignored".to_string(),
            span: span_at(1, 0),
        },
        Diagnostic {
            kind: DiagnosticKind::ParseError,
            message: "also ignored".to_string(),
            span: span_at(2, 0),
        },
    ];
    assert_eq!(capture(&diagnostics), "");
}

#[test]
fn writes_a_warning_line_for_a_var_detected_diagnostic() {
    let diagnostics = vec![Diagnostic {
        kind: DiagnosticKind::VarDetected,
        message: "`var` declaration detected; treat as block-scoped".to_string(),
        span: span_at(3, 7),
    }];
    assert_eq!(
        capture(&diagnostics),
        "uns: warning: L3:7: `var` declaration detected; treat as block-scoped\n"
    );
}

#[test]
fn skips_non_var_detected_diagnostics_while_writing_the_matching_ones() {
    let diagnostics = vec![
        Diagnostic {
            kind: DiagnosticKind::UnresolvedIdentifier,
            message: "ignored".to_string(),
            span: span_at(1, 0),
        },
        Diagnostic {
            kind: DiagnosticKind::VarDetected,
            message: "a".to_string(),
            span: span_at(2, 3),
        },
        Diagnostic {
            kind: DiagnosticKind::VarDetected,
            message: "b".to_string(),
            span: span_at(4, 5),
        },
    ];
    assert_eq!(
        capture(&diagnostics),
        "uns: warning: L2:3: a\nuns: warning: L4:5: b\n"
    );
}
