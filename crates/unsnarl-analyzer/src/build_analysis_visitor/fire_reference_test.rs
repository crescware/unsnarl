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

use super::ternary_statement_head_expr;

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
