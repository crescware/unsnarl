use super::*;
use unsnarl_ir::SourceLine;

#[test]
fn parses_single_token() {
    let queries = parse_root_queries("10:foo").expect("should parse");
    assert_eq!(queries.len(), 1);
    assert!(matches!(
        queries[0],
        ParsedRootQuery::LineName {
            line: SourceLine(10),
            ..
        },
    ));
}

#[test]
fn parses_comma_separated_tokens() {
    let queries = parse_root_queries("10:foo,42,9-13:bar").expect("should parse");
    assert_eq!(queries.len(), 3);
    assert!(matches!(queries[0], ParsedRootQuery::LineName { .. }));
    assert!(matches!(queries[1], ParsedRootQuery::Line { .. }));
    assert!(matches!(queries[2], ParsedRootQuery::RangeName { .. }));
}

#[test]
fn parses_l_prefixed_forms_in_comma_list() {
    let queries = parse_root_queries("L10,L5-10,L20").expect("should parse");
    assert_eq!(queries.len(), 3);
    assert_eq!(
        queries[0],
        ParsedRootQuery::LineOrName {
            line: SourceLine(10),
            name: "L10".to_string(),
            raw: "L10".to_string(),
        },
    );
    assert_eq!(
        queries[1],
        ParsedRootQuery::Range {
            start: SourceLine(5),
            end: SourceLine(10),
            raw: "L5-10".to_string(),
        },
    );
    assert_eq!(
        queries[2],
        ParsedRootQuery::LineOrName {
            line: SourceLine(20),
            name: "L20".to_string(),
            raw: "L20".to_string(),
        },
    );
}

#[test]
fn rejects_empty_value() {
    assert_eq!(
        parse_root_queries(""),
        Err("empty --roots value".to_string())
    );
}

#[test]
fn rejects_whitespace_around_tokens() {
    assert!(parse_root_queries("10, 42").is_err());
    assert!(parse_root_queries(" 10").is_err());
}

#[test]
fn rejects_trailing_or_leading_comma() {
    assert!(parse_root_queries("10,").is_err());
    assert!(parse_root_queries(",10").is_err());
}

#[test]
fn rejects_empty_token_in_middle() {
    assert!(parse_root_queries("10,,42").is_err());
}

#[test]
fn propagates_offending_token_in_error() {
    let err = parse_root_queries("10,foo-bar").unwrap_err();
    assert!(
        err.contains("foo-bar"),
        "expected error to mention 'foo-bar', got {err:?}",
    );
}
