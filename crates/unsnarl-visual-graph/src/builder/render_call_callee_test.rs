use unsnarl_ir::primitive::{SourceColumn, SourceIndex, SourceLine, Span, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::{SerializedHeadExpression, SerializedHeadOperand};
use unsnarl_oxc_parity::AssignOperator;

use super::render_call_callee;

fn idx() -> SourceIndex<'static> {
    SourceIndex::build("")
}

fn ident(name: &str) -> SerializedHeadExpression {
    SerializedHeadExpression::identifier(name.to_string())
}

fn member(object: SerializedHeadExpression, property: &str) -> SerializedHeadExpression {
    SerializedHeadExpression::member(object, property.to_string())
}

fn call(callee: SerializedHeadExpression) -> SerializedHeadExpression {
    SerializedHeadExpression::Call {
        callee: Box::new(callee),
    }
}

fn new_expr(callee: SerializedHeadExpression) -> SerializedHeadExpression {
    SerializedHeadExpression::New {
        callee: Box::new(callee),
    }
}

fn await_expr(argument: SerializedHeadExpression) -> SerializedHeadExpression {
    SerializedHeadExpression::Await {
        argument: Box::new(argument),
    }
}

fn span() -> Span {
    Span::new(SourceLine(1), SourceColumn(0), Utf16CodeUnitOffset(0))
}

#[test]
fn call_returns_callee_text_without_parens() {
    let head = call(ident("run"));
    assert_eq!(render_call_callee(&head, &idx()), Some("run".to_string()));
}

#[test]
fn call_with_member_callee_renders_dotted_path() {
    let head = call(member(member(ident("console"), "log"), "info"));
    assert_eq!(
        render_call_callee(&head, &idx()),
        Some("console.log.info".to_string())
    );
}

#[test]
fn new_expression_renders_with_new_prefix() {
    let head = new_expr(ident("Foo"));
    assert_eq!(
        render_call_callee(&head, &idx()),
        Some("new Foo".to_string())
    );
}

#[test]
fn await_unwraps_a_top_level_call() {
    let head = await_expr(call(member(ident("svc"), "fetch")));
    assert_eq!(
        render_call_callee(&head, &idx()),
        Some("svc.fetch".to_string())
    );
}

#[test]
fn await_unwraps_a_top_level_new() {
    let head = await_expr(new_expr(ident("Loader")));
    assert_eq!(
        render_call_callee(&head, &idx()),
        Some("new Loader".to_string())
    );
}

#[test]
fn nested_await_inside_await_is_not_unwrapped() {
    let head = await_expr(await_expr(call(ident("foo"))));
    assert_eq!(render_call_callee(&head, &idx()), None);
}

#[test]
fn bare_identifier_returns_none() {
    assert_eq!(render_call_callee(&ident("x"), &idx()), None);
}

#[test]
fn bare_member_returns_none() {
    let head = member(ident("a"), "b");
    assert_eq!(render_call_callee(&head, &idx()), None);
}

#[test]
fn assign_shape_returns_none() {
    let head = SerializedHeadExpression::Assign {
        operator: AssignOperator::Assign,
        left: Box::new(SerializedHeadOperand {
            head: ident("x"),
            start_span: span(),
            end_span: span(),
        }),
        right: Box::new(SerializedHeadOperand {
            head: ident("y"),
            start_span: span(),
            end_span: span(),
        }),
    };
    assert_eq!(render_call_callee(&head, &idx()), None);
}
