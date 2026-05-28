use unsnarl_ir::primitive::{SourceColumn, SourceLine, Span, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::{SerializedHeadExpression, SerializedHeadOperand};
use unsnarl_oxc_parity::{AssignOperator, UpdateOperator};

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

fn operand(head: SerializedHeadExpression, start: u32, end: u32) -> Box<SerializedHeadOperand> {
    Box::new(SerializedHeadOperand {
        head,
        start_span: span(start),
        end_span: span(end),
    })
}

fn assign(
    left: SerializedHeadExpression,
    left_start: u32,
    left_end: u32,
    right: SerializedHeadExpression,
    right_start: u32,
    right_end: u32,
) -> SerializedHeadExpression {
    SerializedHeadExpression::Assign {
        operator: AssignOperator::Assign,
        left: operand(left, left_start, left_end),
        right: operand(right, right_start, right_end),
    }
}

fn update(argument: SerializedHeadExpression, start: u32, end: u32) -> SerializedHeadExpression {
    SerializedHeadExpression::Update {
        operator: UpdateOperator::Increment,
        prefix: false,
        argument: operand(argument, start, end),
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

#[test]
fn descends_through_assign_right_to_locate_a_call_in_the_rhs() {
    // `x = a(() => {})` -- the call lives under `Assign.right.head`.
    // `x` occupies offsets 0..1; `a(...)` occupies 4..16 (the actual
    // numbers are irrelevant -- only the (start, end) pair fed to
    // the lookup matters).
    let head = assign(ident("x"), 0, 1, call(ident("a"), 4, 16), 4, 16);
    let callee = find_call_callee_in_head(&head, Utf16CodeUnitOffset(4), Utf16CodeUnitOffset(16))
        .expect("call under Assign.right must be found");
    assert_eq!(callee_name_of(callee), Some("a"));
}

#[test]
fn descends_through_assign_left_to_locate_a_call_in_the_lhs() {
    // Computed-member targets such as `arr[a()] = 1` keep their
    // call on the LHS once the head head is built; ensure the
    // walker reaches it. The Assign.right operand is an Elided
    // placeholder here so the lookup must succeed via the left
    // side alone.
    let head = assign(
        call(ident("a"), 4, 7),
        4,
        7,
        SerializedHeadExpression::Elided,
        11,
        12,
    );
    let callee = find_call_callee_in_head(&head, Utf16CodeUnitOffset(4), Utf16CodeUnitOffset(7))
        .expect("call under Assign.left must be found");
    assert_eq!(callee_name_of(callee), Some("a"));
}

#[test]
fn descends_through_update_argument_to_locate_a_call() {
    // `++a(...)` isn't valid JS today, but the recursion keeps the
    // walker symmetric with Assign -- exercising the arm pins the
    // descent behaviour so a future shape that puts a call under
    // an Update operand stays reachable.
    let head = update(call(ident("a"), 2, 5), 2, 5);
    let callee = find_call_callee_in_head(&head, Utf16CodeUnitOffset(2), Utf16CodeUnitOffset(5))
        .expect("call under Update.argument must be found");
    assert_eq!(callee_name_of(callee), Some("a"));
}

#[test]
fn returns_none_when_assign_holds_no_matching_call() {
    let head = assign(ident("x"), 0, 1, ident("y"), 4, 5);
    let res = find_call_callee_in_head(&head, Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(5));
    assert!(res.is_none());
}
