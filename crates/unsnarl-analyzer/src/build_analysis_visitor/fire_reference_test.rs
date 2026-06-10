//! Sibling tests for [`ternary_statement_head_expr`].
//!
//! The helper picks the sub-expression whose head should label a
//! ternary statement's CallProxy. Each branch is pinned by parsing a
//! one-statement program, running the helper on the statement's
//! expression, and slicing the source by the returned span so the exact
//! sub-expression is asserted by text.
//!
//! The call in a ternary's `test` position (`items.map(cb) ? a : b`)
//! falls outside every arm; defining and pinning that fallback is
//! tracked separately (issue #278) and is intentionally not asserted
//! here.

use oxc_ast::ast::{Expression, Program, Statement};
use oxc_span::GetSpan;

use crate::analyzer_fixtures::parse_ts;

use super::{parenthesized_conditional_start, ternary_statement_head_expr};

fn expression_of<'a>(program: &'a Program<'a>) -> &'a Expression<'a> {
    match program.body.first().expect("one statement") {
        Statement::ExpressionStatement(es) => &es.expression,
        _ => unreachable!("test source is an expression statement"),
    }
}

/// Run the helper on the program's single expression statement and
/// return the slice of `source` covered by the returned head span.
fn head_slice<'a>(source: &'a str, program: &'a Program<'a>) -> &'a str {
    let head = ternary_statement_head_expr(expression_of(program));
    let span = head.span();
    &source[span.start as usize..span.end as usize]
}

#[test]
fn non_ternary_statement_yields_the_expression_itself() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "items.map(cb);";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "items.map(cb)");
}

#[test]
fn consequent_call_is_chosen() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? items.map(cb) : x;";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "items.map(cb)");
}

#[test]
fn alternate_call_is_chosen_when_consequent_is_not_a_call() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? x : items.map(cb);";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "items.map(cb)");
}

#[test]
fn new_expression_arm_is_chosen() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? new Foo(cb) : x;";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "new Foo(cb)");
}

#[test]
fn nested_ternary_arm_recurses_to_its_call() {
    // Parses as `cond ? (inner ? deep.map(cb) : y) : z`: the outer
    // consequent is itself a ternary, so the helper recurses into it
    // and returns that inner consequent's call.
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? inner ? deep.map(cb) : y : z;";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "deep.map(cb)");
}

#[test]
fn ternary_with_no_call_arm_yields_the_expression_itself() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? a : b;";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "cond ? a : b");
}

#[test]
fn parenthesized_ternary_consequent_call_is_chosen() {
    // Parentheses around the whole ternary are stripped before the
    // arm scan, so the call arm is chosen exactly as in the
    // unparenthesized form (issue #276).
    let alloc = oxc_allocator::Allocator::default();
    let src = "(cond ? items.map(cb) : x);";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "items.map(cb)");
}

#[test]
fn doubly_parenthesized_ternary_consequent_call_is_chosen() {
    // Two wrapping layers (`((…))`): the parser keeps one
    // `ParenthesizedExpression` per pair (`preserve_parens` on), so
    // `strip_parens`' loop must iterate twice before the conditional is
    // reached. The call arm is then chosen exactly as in the single-
    // and zero-paren forms — pinning the multi-level strip (issue #276).
    let alloc = oxc_allocator::Allocator::default();
    let src = "((cond ? items.map(cb) : x));";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "items.map(cb)");
}

#[test]
fn parenthesized_nested_ternary_arm_recurses_to_its_call() {
    // The parenthesized counterpart of
    // `nested_ternary_arm_recurses_to_its_call`: the wrapping
    // parentheses are stripped, then the helper recurses through the
    // inner ternary to its call — the most complex paren + recursion
    // path (issue #276).
    let alloc = oxc_allocator::Allocator::default();
    let src = "(cond ? inner ? deep.map(cb) : y : z);";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "deep.map(cb)");
}

#[test]
fn parenthesized_value_only_ternary_yields_the_stripped_conditional() {
    // No call arm: the head is the conditional with the wrapping
    // parentheses stripped, not the verbatim `(cond ? a : b)`.
    let alloc = oxc_allocator::Allocator::default();
    let src = "(cond ? a : b);";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "cond ? a : b");
}

#[test]
fn parenthesized_non_ternary_yields_the_expression_as_given() {
    // A parenthesized non-conditional is out of scope here: its head
    // keeps the surrounding parentheses, unchanged from before.
    let alloc = oxc_allocator::Allocator::default();
    let src = "(items.map(cb));";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program), "(items.map(cb))");
}

#[test]
fn parenthesized_conditional_start_points_at_inner_conditional() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "(cond ? a : b);";
    let program = parse_ts(&alloc, src);
    let off = parenthesized_conditional_start(expression_of(&program))
        .expect("a parenthesized conditional records its inner start");
    // Just past the opening paren, at the ConditionalExpression itself.
    assert_eq!(off.0, 1);
    assert!(src[off.0 as usize..].starts_with("cond ? a : b"));
}

#[test]
fn doubly_parenthesized_conditional_start_points_at_inner_conditional() {
    // `strip_parens`' loop peels both `(` layers, so the recorded start
    // lands on the ConditionalExpression itself — two bytes past the
    // outer paren, not on the inner `(`. A single-strip regression would
    // leave a `ParenthesizedExpression` here and yield `None`.
    let alloc = oxc_allocator::Allocator::default();
    let src = "((cond ? a : b));";
    let program = parse_ts(&alloc, src);
    let off = parenthesized_conditional_start(expression_of(&program))
        .expect("a doubly-parenthesized conditional records its inner start");
    assert_eq!(off.0, 2);
    assert!(src[off.0 as usize..].starts_with("cond ? a : b"));
}

#[test]
fn parenthesized_conditional_start_is_none_for_unparenthesized_conditional() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? a : b;";
    let program = parse_ts(&alloc, src);
    assert!(parenthesized_conditional_start(expression_of(&program)).is_none());
}

#[test]
fn parenthesized_conditional_start_is_none_for_parenthesized_non_conditional() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "(foo());";
    let program = parse_ts(&alloc, src);
    assert!(parenthesized_conditional_start(expression_of(&program)).is_none());
}
