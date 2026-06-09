//! Sibling tests for [`head_source_for_call`].
//!
//! The helper descends a ternary `bound` expression into whichever arm
//! contains the callback's enclosing call (by span containment),
//! recursing for nested ternaries, so the CallProxy is labelled by the
//! arm's own call rather than the whole `cond ? … : …` text. Each branch
//! is pinned by parsing a one-statement program, locating the call's
//! byte span by its source text, and slicing the source by the returned
//! head span.
//!
//! The fallback when the call sits in the ternary's `test` position
//! (matching no arm, so the whole expression is returned) has undefined
//! expected behaviour tracked in issue #278 and is intentionally not
//! asserted here.

use oxc_ast::ast::{Expression, Program, Statement};
use oxc_span::{GetSpan, Span};

use crate::analyzer_fixtures::parse_ts;

use super::head_source_for_call;

fn expression_of<'a>(program: &'a Program<'a>) -> &'a Expression<'a> {
    match program.body.first().expect("one statement") {
        Statement::ExpressionStatement(es) => &es.expression,
        _ => unreachable!("test source is an expression statement"),
    }
}

/// Run the helper on the program's single expression, taking the call
/// span from the byte range of `call_text` in `source`, and return the
/// slice of `source` covered by the returned head span.
fn head_slice<'a>(source: &'a str, program: &'a Program<'a>, call_text: &str) -> &'a str {
    let start = source.find(call_text).expect("call text present in source") as u32;
    let call_span = Span::new(start, start + call_text.len() as u32);
    let head = head_source_for_call(expression_of(program), call_span);
    let span = head.span();
    &source[span.start as usize..span.end as usize]
}

#[test]
fn non_ternary_bound_is_returned_unchanged() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "items.map(cb);";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program, "items.map(cb)"), "items.map(cb)");
}

#[test]
fn call_in_consequent_descends_into_that_arm() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? items.map(cb) : x;";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program, "items.map(cb)"), "items.map(cb)");
}

#[test]
fn call_in_alternate_descends_into_that_arm() {
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? x : items.map(cb);";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program, "items.map(cb)"), "items.map(cb)");
}

#[test]
fn call_in_nested_ternary_arm_recurses() {
    // Parses as `cond ? (inner ? deep.map(cb) : y) : z`: the call sits in
    // the inner consequent, so the helper descends the outer consequent
    // then recurses into the inner arm.
    let alloc = oxc_allocator::Allocator::default();
    let src = "cond ? inner ? deep.map(cb) : y : z;";
    let program = parse_ts(&alloc, src);
    assert_eq!(head_slice(src, &program, "deep.map(cb)"), "deep.map(cb)");
}
