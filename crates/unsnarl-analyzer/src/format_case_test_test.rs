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
fn string_literal_fallback_escapes_double_quote_and_backslash() {
    let alloc = Allocator::default();
    // `"a\\b\"c"` — the source has a backslash and an escaped double quote;
    // after parsing the StringLiteral value contains `a\b"c`. With raw=""
    // we hit the type-specific fallback which calls `json_string`.
    let program = parse_ts(&alloc, "switch (x) { case \"a\\\\b\\\"c\": ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "\"a\\\\b\\\"c\"");
}

#[test]
fn string_literal_fallback_escapes_newline_carriage_return_and_tab() {
    let alloc = Allocator::default();
    // After parsing, the StringLiteral value contains the actual `\n`, `\r`,
    // and `\t` control characters. `json_string` should escape each back.
    let program = parse_ts(&alloc, "switch (x) { case \"\\n\\r\\t\": ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "\"\\n\\r\\t\"");
}

#[test]
fn string_literal_fallback_escapes_low_control_characters() {
    let alloc = Allocator::default();
    // `` is a low control character; `json_string` should emit
    // `` in the `< 0x20` branch.
    let program = parse_ts(&alloc, "switch (x) { case \"\\u0001\": ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "\"\\u0001\"");
}

#[test]
fn number_literal_fallback_for_non_integer_uses_to_string() {
    let alloc = Allocator::default();
    // `3.14` has non-zero fractional part: the integer-formatting branch is
    // skipped and `n.to_string()` runs.
    let program = parse_ts(&alloc, "switch (x) { case 3.14: ; }");
    let test_expr = case_test_of(&program);
    assert_eq!(format_case_test(test_expr, ""), "3.14");
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
