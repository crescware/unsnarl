//! Sibling tests for `format_case_test`.
//!
//! The TS test suite uses programmatic `AstNode` literals (arbitrary
//! `start`/`end` plus optional `name` / `value` fields). The Rust
//! port lifts those scenarios to real `oxc_parser` snippets so the
//! `&Expression` shape is genuine; only the "type-specific
//! fallback" branches remain assertable on synthetic spans.

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement};

use crate::testing::parse_ts;

use super::format_case_test;

fn case_test_of<'a>(program: &'a oxc_ast::ast::Program<'a>) -> &'a Expression<'a> {
    let switch = match program.body.first().expect("statement") {
        Statement::SwitchStatement(s) => s,
        _ => unreachable!(),
    };
    switch
        .cases
        .first()
        .expect("case")
        .test
        .as_ref()
        .expect("non-default case")
}

#[test]
fn uses_raw_slice_when_span_in_bounds() {
    let alloc = Allocator::default();
    let raw = "switch (x) { case x === 1: ; }";
    let program = parse_ts(&alloc, raw);
    let test_expr = case_test_of(&program);
    // `x === 1` lives at byte offsets 18..25 in this fixture; we don't
    // assert exact offsets, only that the returned slice matches.
    let result = format_case_test(test_expr, raw);
    assert_eq!(result, "x === 1");
}

#[test]
fn identifier_outside_raw_falls_back_to_name() {
    let alloc = Allocator::default();
    // Parse a tiny program so we get a real `Expression::Identifier`,
    // but pass empty `raw` so the raw-slice branch is skipped.
    let program = parse_ts(&alloc, "switch (x) { case foo: ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "foo");
}

#[test]
fn literal_fallback_for_number() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case 42: ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "42");
}

#[test]
fn literal_fallback_for_string() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case \"hi\": ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "\"hi\"");
}

#[test]
fn literal_fallback_for_boolean() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case true: ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "true");
}

#[test]
fn literal_fallback_for_null() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case null: ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "null");
}

#[test]
fn unknown_expression_falls_back_to_placeholder() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "switch (x) { case [1]: ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "<expr>");
}

#[test]
fn long_expression_falls_back_to_type_specific_format() {
    // The expression `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` (34 chars)
    // exceeds CASE_TEST_MAX_LENGTH (32), so the raw-slice branch is
    // skipped and the identifier name is returned.
    let alloc = Allocator::default();
    let raw = "switch (x) { case aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: ; }";
    let program = parse_ts(&alloc, raw);
    let test_expr = case_test_of(&program);
    assert_eq!(
        format_case_test(test_expr, raw),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
}
