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
fn a_generation_level_is_rejected_for_the_poc() {
    // `+a3` parses structurally but ROOT_QUERY_SCOPE_HIGHLIGHT leaves
    // direction_level off, so it must be rejected here.
    assert!(parse_highlight_queries("foo..+a3").is_err());
}
