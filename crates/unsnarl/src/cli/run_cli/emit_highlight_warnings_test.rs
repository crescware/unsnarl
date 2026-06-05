use super::*;

fn capture(warnings: Option<&[HighlightWarning]>) -> String {
    let mut buf = Vec::new();
    emit_highlight_warnings(warnings, &mut buf);
    String::from_utf8(buf).expect("stderr should be valid UTF-8")
}

#[test]
fn writes_nothing_when_warnings_is_none() {
    assert_eq!(capture(None), "");
}

#[test]
fn writes_nothing_when_warnings_is_empty() {
    let empty: Vec<HighlightWarning> = Vec::new();
    assert_eq!(capture(Some(&empty)), "");
}

#[test]
fn writes_a_no_match_line_for_an_unmatched_endpoint() {
    let warnings = vec![HighlightWarning::NoMatch {
        raw: "ghost..+a".to_string(),
    }];
    assert_eq!(
        capture(Some(&warnings)),
        "uns: warning: highlight query 'ghost..+a' matched no node\n"
    );
}

#[test]
fn writes_a_no_path_line_for_a_disconnected_path() {
    let warnings = vec![HighlightWarning::NoPath {
        raw: "a..z".to_string(),
    }];
    assert_eq!(
        capture(Some(&warnings)),
        "uns: warning: highlight query 'a..z' has no connecting path\n"
    );
}

#[test]
fn writes_one_line_per_warning_in_order() {
    let warnings = vec![
        HighlightWarning::NoMatch {
            raw: "ghost..+c".to_string(),
        },
        HighlightWarning::NoPath {
            raw: "a..z".to_string(),
        },
    ];
    assert_eq!(
        capture(Some(&warnings)),
        "uns: warning: highlight query 'ghost..+c' matched no node\n\
         uns: warning: highlight query 'a..z' has no connecting path\n"
    );
}
