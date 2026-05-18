use super::*;
use crate::parse_root_query_ast::parse_root_query_ast;
use crate::root_query_scope::RootQueryScope;

const SCOPE_FULL: RootQueryScope = RootQueryScope {
    point: true,
    path: true,
    direction: true,
    direction_level: true,
};

fn parse_full(text: &str) -> RootQuery {
    parse_root_query_ast(text, &SCOPE_FULL)
        .unwrap_or_else(|e| panic!("parse failed for {text:?}: {:?}", e[0].message))
}

#[test]
fn rejects_zero_line_in_each_form() {
    for input in ["0", "0..foo", "foo..0", "0..+a"] {
        let r = validate_root_query(&parse_full(input));
        let err = r.unwrap_err();
        assert!(
            err[0].message.contains("line must be >= 1"),
            "expected 'line must be >= 1' for {input:?}, got {:?}",
            err[0].message,
        );
    }
}

#[test]
fn accepts_valid_root_queries() {
    for input in [
        "foo", "10", "foo..bar", "1..10", "foo..+a", "foo..+a0", "foo..+a3",
    ] {
        validate_root_query(&parse_full(input)).expect("should validate");
    }
}
