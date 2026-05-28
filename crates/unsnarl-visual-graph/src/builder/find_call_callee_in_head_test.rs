use unsnarl_ir::primitive::{SourceColumn, SourceLine, Span, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::SerializedHeadExpression;

use super::find_call_callee_in_head;

fn span(offset: u32) -> Span {
    Span::new(
        SourceLine(1),
        SourceColumn(offset),
        Utf16CodeUnitOffset(offset),
    )
}

fn ident(name: &str) -> SerializedHeadExpression {
    SerializedHeadExpression::identifier(name.to_string())
}

fn member(object: SerializedHeadExpression, property: &str) -> SerializedHeadExpression {
    SerializedHeadExpression::member(object, property.to_string())
}

fn call(callee: SerializedHeadExpression, start: u32, end: u32) -> SerializedHeadExpression {
    SerializedHeadExpression::Call {
        callee: Box::new(callee),
        start_span: span(start),
        end_span: span(end),
    }
}

fn new_expr(callee: SerializedHeadExpression, start: u32, end: u32) -> SerializedHeadExpression {
    SerializedHeadExpression::New {
        callee: Box::new(callee),
        start_span: span(start),
        end_span: span(end),
    }
}

fn await_(argument: SerializedHeadExpression) -> SerializedHeadExpression {
    SerializedHeadExpression::Await {
        argument: Box::new(argument),
    }
}

fn callee_name_of(node: &SerializedHeadExpression) -> Option<&str> {
    match node {
        SerializedHeadExpression::Identifier { name } => Some(name.as_str()),
        SerializedHeadExpression::Member { property, .. } => Some(property.as_str()),
        _ => None,
    }
}

#[test]
fn returns_none_when_head_has_no_call_or_new() {
    let head = ident("foo");
    let res = find_call_callee_in_head(&head, Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(3));
    assert!(res.is_none());
}

#[test]
fn matches_top_level_call_by_exact_span_pair() {
    // `run()` at offsets 0..5
    let head = call(ident("run"), 0, 5);
    let callee = find_call_callee_in_head(&head, Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(5))
        .expect("matching call must be found");
    assert_eq!(callee_name_of(callee), Some("run"));
}

#[test]
fn distinguishes_inner_and_outer_calls_in_a_chain_by_end_offset() {
    // `a()` at 0..3, then `.b()` at 0..6 (chain root shares span.start)
    let inner = call(ident("a"), 0, 3);
    let outer = call(member(inner, "b"), 0, 6);

    let inner_callee =
        find_call_callee_in_head(&outer, Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(3))
            .expect("inner call");
    assert_eq!(callee_name_of(inner_callee), Some("a"));

    let outer_callee =
        find_call_callee_in_head(&outer, Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(6))
            .expect("outer call");
    assert_eq!(callee_name_of(outer_callee), Some("b"));
}

#[test]
fn descends_through_await_to_locate_the_underlying_call() {
    // `await foo()` -- the call lives one level below the Await.
    let head = await_(call(ident("foo"), 6, 11));
    let callee = find_call_callee_in_head(&head, Utf16CodeUnitOffset(6), Utf16CodeUnitOffset(11))
        .expect("call under await must be found");
    assert_eq!(callee_name_of(callee), Some("foo"));
}

#[test]
fn matches_new_expression_by_span_pair() {
    let head = new_expr(ident("Service"), 0, 11);
    let callee = find_call_callee_in_head(&head, Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(11))
        .expect("new expression must be found");
    assert_eq!(callee_name_of(callee), Some("Service"));
}

#[test]
fn returns_none_when_no_call_matches_the_span_pair() {
    let head = call(ident("foo"), 0, 5);
    let res = find_call_callee_in_head(&head, Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(99));
    assert!(res.is_none());
}
