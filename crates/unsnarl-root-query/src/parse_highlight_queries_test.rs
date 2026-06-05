use super::parse_highlight_queries;
use crate::parsed_root_query::ParsedRootQuery;
use crate::root_query::{Direction, RootQuery};

#[test]
fn a_bare_name_parses_as_a_single_point_query() {
    let qs = parse_highlight_queries("foo").expect("valid highlight query");
    assert_eq!(qs.len(), 1);
    assert!(matches!(
        &qs[0],
        RootQuery::Single {
            query: ParsedRootQuery::Name { name, .. },
            ..
        } if name == "foo"
    ));
}

#[test]
fn a_path_endpoint_may_carry_a_line_or_range_constraint() {
    // `123:foo..123-9999:bar` is the issue #90 example for pinning a
    // path's direction with a line range; both endpoints must parse.
    let qs = parse_highlight_queries("123:foo..123-9999:bar").expect("valid highlight query");
    assert!(matches!(
        &qs[0],
        RootQuery::Path {
            lhs: ParsedRootQuery::LineName { name: lhs, .. },
            rhs: ParsedRootQuery::RangeName { name: rhs, .. },
            ..
        } if lhs == "foo" && rhs == "bar"
    ));
}

#[test]
fn a_double_dot_with_a_name_rhs_parses_as_a_path() {
    let qs = parse_highlight_queries("foo..bar").expect("valid highlight query");
    assert!(matches!(&qs[0], RootQuery::Path { .. }));
}

#[test]
fn a_double_dot_with_a_direction_token_parses_as_a_direction() {
    let after = parse_highlight_queries("foo..+a").expect("valid highlight query");
    assert!(matches!(
        &after[0],
        RootQuery::Direction {
            dir: Direction::After,
            level: None,
            ..
        }
    ));
    let context = parse_highlight_queries("foo..+c").expect("valid highlight query");
    assert!(matches!(
        &context[0],
        RootQuery::Direction {
            dir: Direction::Context,
            ..
        }
    ));
}

#[test]
fn a_comma_separates_multiple_queries() {
    let qs = parse_highlight_queries("foo..+a,bar").expect("valid highlight query");
    assert_eq!(qs.len(), 2);
}

#[test]
fn a_generation_level_is_reserved_and_rejected() {
    // `+a3` parses structurally but ROOT_QUERY_SCOPE_HIGHLIGHT leaves
    // direction_level off (the `+aN` radius is reserved for the future,
    // issue #90), so it must be rejected here.
    assert!(parse_highlight_queries("foo..+a3").is_err());
}

#[test]
fn an_empty_value_is_rejected() {
    // Mirrors `parse_root_queries`' `rejects_empty_value`: an empty
    // `-H` value never produces a query list.
    assert_eq!(
        parse_highlight_queries(""),
        Err("empty --highlight value".to_string())
    );
}

#[test]
fn an_empty_token_in_the_middle_is_rejected() {
    // `split(',')` yields an empty token for `foo,,bar`; the per-token
    // parse rejects it, so the whole value is rejected (mirrors
    // `parse_root_queries`' `rejects_empty_token_in_middle`).
    assert!(parse_highlight_queries("foo,,bar").is_err());
}
