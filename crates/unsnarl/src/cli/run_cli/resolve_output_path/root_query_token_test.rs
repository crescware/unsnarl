use super::*;

use unsnarl_ir::SourceLine;

#[test]
fn name_uses_identifier_verbatim() {
    let q = ParsedRootQuery::Name {
        name: "value".to_string(),
        raw: "value".to_string(),
    };
    assert_eq!(root_query_token(&q), "value");
}

#[test]
fn line_yields_l_n() {
    let q = ParsedRootQuery::Line {
        line: SourceLine(42),
        raw: "42".to_string(),
    };
    assert_eq!(root_query_token(&q), "l42");
}

#[test]
fn line_name_yields_l_n_dash_id() {
    let q = ParsedRootQuery::LineName {
        line: SourceLine(42),
        name: "render".to_string(),
        raw: "42:render".to_string(),
    };
    assert_eq!(root_query_token(&q), "l42-render");
}

#[test]
fn range_yields_l_start_dash_end() {
    let q = ParsedRootQuery::Range {
        start: SourceLine(10),
        end: SourceLine(12),
        raw: "10-12".to_string(),
    };
    assert_eq!(root_query_token(&q), "l10-12");
}

#[test]
fn range_name_yields_l_start_dash_end_dash_id() {
    let q = ParsedRootQuery::RangeName {
        start: SourceLine(10),
        end: SourceLine(12),
        name: "render".to_string(),
        raw: "10-12:render".to_string(),
    };
    assert_eq!(root_query_token(&q), "l10-12-render");
}

#[test]
fn line_or_name_normalizes_to_plain_line_shape() {
    let q = ParsedRootQuery::LineOrName {
        line: SourceLine(12),
        name: "L12".to_string(),
        raw: "L12".to_string(),
    };
    assert_eq!(root_query_token(&q), "l12");
}
