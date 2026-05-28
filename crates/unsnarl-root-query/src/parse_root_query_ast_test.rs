use super::*;
use unsnarl_ir::SourceLine;

use crate::generation_count::GenerationCount;
use crate::parsed_root_query::ParsedRootQuery;
use crate::root_query::Direction;
use crate::root_query_scope::ROOT_QUERY_SCOPE_POINT_ONLY;

const SCOPE_FULL: RootQueryScope = RootQueryScope {
    point: true,
    path: true,
    direction: true,
    direction_level: true,
};

fn assert_err_contains(r: &Result<RootQuery, Vec<ParseError>>, needle: &str) {
    let err = r.as_ref().expect_err("expected error");
    let msg = &err[0].message;
    assert!(
        msg.contains(needle),
        "expected error to contain {needle:?}, got {msg:?}",
    );
}

#[test]
fn wraps_bare_line_as_single_under_point_only() {
    assert_eq!(
        parse_root_query_ast("10", &ROOT_QUERY_SCOPE_POINT_ONLY),
        Ok(RootQuery::Single {
            query: ParsedRootQuery::Line {
                line: SourceLine(10),
                raw: "10".to_string(),
            },
            raw: "10".to_string(),
        }),
    );
}

#[test]
fn wraps_identifier_as_single_under_point_only() {
    assert_eq!(
        parse_root_query_ast("foo", &ROOT_QUERY_SCOPE_POINT_ONLY),
        Ok(RootQuery::Single {
            query: ParsedRootQuery::Name {
                name: "foo".to_string(),
                raw: "foo".to_string(),
            },
            raw: "foo".to_string(),
        }),
    );
}

#[test]
fn rejects_empty_token() {
    assert_eq!(
        parse_root_query_ast("", &ROOT_QUERY_SCOPE_POINT_ONLY),
        Err(vec![ParseError {
            message: "empty root query".to_string(),
        }]),
    );
}

#[test]
fn syntactically_accepts_zero_line() {
    assert_eq!(
        parse_root_query_ast("0", &ROOT_QUERY_SCOPE_POINT_ONLY),
        Ok(RootQuery::Single {
            query: ParsedRootQuery::Line {
                line: SourceLine(0),
                raw: "0".to_string(),
            },
            raw: "0".to_string(),
        }),
    );
}

#[test]
fn syntactically_accepts_descending_range() {
    assert_eq!(
        parse_root_query_ast("5-1", &ROOT_QUERY_SCOPE_POINT_ONLY),
        Ok(RootQuery::Single {
            query: ParsedRootQuery::Range {
                start: SourceLine(5),
                end: SourceLine(1),
                raw: "5-1".to_string(),
            },
            raw: "5-1".to_string(),
        }),
    );
}

#[test]
fn parses_path_query() {
    assert_eq!(
        parse_root_query_ast("foo..bar", &SCOPE_FULL),
        Ok(RootQuery::Path {
            lhs: ParsedRootQuery::Name {
                name: "foo".to_string(),
                raw: "foo".to_string(),
            },
            rhs: ParsedRootQuery::Name {
                name: "bar".to_string(),
                raw: "bar".to_string(),
            },
            raw: "foo..bar".to_string(),
        }),
    );
}

#[test]
fn parses_path_with_mixed_endpoint_kinds() {
    assert_eq!(
        parse_root_query_ast("10..L20", &SCOPE_FULL),
        Ok(RootQuery::Path {
            lhs: ParsedRootQuery::Line {
                line: SourceLine(10),
                raw: "10".to_string(),
            },
            rhs: ParsedRootQuery::LineOrName {
                line: SourceLine(20),
                name: "L20".to_string(),
                raw: "L20".to_string(),
            },
            raw: "10..L20".to_string(),
        }),
    );
}

#[test]
fn parses_direction_without_level() {
    for (suffix, dir) in [
        ("+a", Direction::After),
        ("+b", Direction::Before),
        ("+c", Direction::Context),
    ] {
        let token = format!("foo..{suffix}");
        assert_eq!(
            parse_root_query_ast(&token, &SCOPE_FULL),
            Ok(RootQuery::Direction {
                lhs: ParsedRootQuery::Name {
                    name: "foo".to_string(),
                    raw: "foo".to_string(),
                },
                dir,
                level: None,
                raw: token.clone(),
            }),
        );
    }
}

#[test]
fn parses_direction_with_level() {
    let r = parse_root_query_ast("foo..+a3", &SCOPE_FULL)
        .expect("syntactically valid direction-with-level input must parse");
    match r {
        RootQuery::Direction { dir, level, .. } => {
            assert_eq!(dir, Direction::After);
            assert_eq!(level, Some(GenerationCount(3)));
        }
        other => panic!("expected Direction, got {other:?}"),
    }

    let r = parse_root_query_ast("foo..+a0", &SCOPE_FULL)
        .expect("syntactically valid direction-with-level input must parse");
    match r {
        RootQuery::Direction { dir, level, .. } => {
            assert_eq!(dir, Direction::After);
            assert_eq!(level, Some(GenerationCount(0)));
        }
        other => panic!("expected Direction, got {other:?}"),
    }
}

#[test]
fn rejects_empty_lhs_of_dotdot() {
    let r = parse_root_query_ast("..foo", &SCOPE_FULL);
    assert_err_contains(&r, "unexpected empty left-hand side of '..'");
}

#[test]
fn rejects_empty_rhs_of_dotdot() {
    let r = parse_root_query_ast("foo..", &SCOPE_FULL);
    assert_err_contains(&r, "unexpected empty right-hand side of '..'");
}

#[test]
fn rejects_duplicate_dotdot() {
    let r = parse_root_query_ast("foo..bar..baz", &SCOPE_FULL);
    assert_err_contains(&r, "unexpected duplicate '..'");
}

#[test]
fn rejects_invalid_direction_tokens() {
    let r = parse_root_query_ast("foo..+x", &SCOPE_FULL);
    assert_err_contains(&r, "unexpected direction token");
}

#[test]
fn point_only_rejects_path_with_syntax_error_only() {
    let r = parse_root_query_ast("foo..bar", &ROOT_QUERY_SCOPE_POINT_ONLY);
    let err =
        r.expect_err("input is constructed to violate SCOPE_POINT_ONLY constraints and must error");
    let msg = &err[0].message;
    assert!(
        msg.contains("unexpected '..'"),
        "expected 'unexpected '..'', got {msg:?}",
    );
    let banned = [
        "未実装",
        "未サポート",
        "reserved",
        "not yet",
        "unsupported",
        "coming soon",
    ];
    let lower = msg.to_lowercase();
    for word in banned {
        assert!(
            !lower.contains(&word.to_lowercase()),
            "error message should not contain {word:?}: {msg:?}",
        );
    }
}

#[test]
fn point_only_rejects_direction_with_syntax_error() {
    let r = parse_root_query_ast("foo..+a", &ROOT_QUERY_SCOPE_POINT_ONLY);
    assert_err_contains(&r, "unexpected direction token");
}

#[test]
fn scope_with_path_only_rejects_direction_tokens() {
    let scope = RootQueryScope {
        point: true,
        path: true,
        direction: false,
        direction_level: false,
    };
    let r = parse_root_query_ast("foo..+a", &scope);
    assert_err_contains(&r, "unexpected direction token");
}

#[test]
fn scope_with_direction_only_rejects_paths() {
    let scope = RootQueryScope {
        point: true,
        path: false,
        direction: true,
        direction_level: false,
    };
    let r = parse_root_query_ast("foo..bar", &scope);
    assert_err_contains(&r, "unexpected '..'");
}

#[test]
fn scope_with_direction_but_no_level_rejects_level() {
    let scope = RootQueryScope {
        point: true,
        path: false,
        direction: true,
        direction_level: false,
    };
    let r = parse_root_query_ast("foo..+a3", &scope);
    assert_err_contains(&r, "unexpected level in direction token");
}

#[test]
fn scope_with_point_disabled_rejects_bare_endpoint() {
    let scope = RootQueryScope {
        point: false,
        path: true,
        direction: true,
        direction_level: true,
    };
    let r = parse_root_query_ast("10", &scope);
    assert_err_contains(&r, "unexpected token");
}

#[test]
fn propagates_lhs_endpoint_parse_error_in_path() {
    let r = parse_root_query_ast("foo-bar..1", &SCOPE_FULL);
    assert_err_contains(&r, "unexpected character in identifier");
}

#[test]
fn propagates_rhs_endpoint_parse_error_in_path() {
    let r = parse_root_query_ast("1..foo-bar", &SCOPE_FULL);
    assert_err_contains(&r, "unexpected character in identifier");
}
