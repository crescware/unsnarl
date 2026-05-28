use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement};
use oxc_span::Span;

use unsnarl_ir::reference::HeadExpression;
use unsnarl_oxc_parity::{AssignOperator, UpdateOperator};

use crate::analyzer_fixtures::parse_ts;

use super::build_head_expression;

fn expression_of<'a>(program: &'a oxc_ast::ast::Program<'a>) -> &'a Expression<'a> {
    match program.body.first().expect("statement") {
        Statement::ExpressionStatement(es) => &es.expression,
        _ => unreachable!(),
    }
}

fn fallback_span<'a>(program: &'a oxc_ast::ast::Program<'a>) -> Span {
    program.body.first().expect("statement").span()
}

use oxc_span::GetSpan;

#[test]
fn identifier_becomes_identifier_head() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "foo;");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::Identifier { name } => assert_eq!(name, "foo"),
        _ => panic!("expected identifier head"),
    }
}

#[test]
fn static_member_chain_becomes_member_head() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "foo.bar.baz;");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::Member { object, property } => {
            assert_eq!(property, "baz");
            match *object {
                HeadExpression::Member {
                    object: inner_obj,
                    property: inner_prop,
                } => {
                    assert_eq!(inner_prop, "bar");
                    assert!(
                        matches!(*inner_obj, HeadExpression::Identifier { name } if name == "foo")
                    );
                }
                _ => panic!("expected nested member head"),
            }
        }
        _ => panic!("expected member head"),
    }
}

#[test]
fn computed_member_is_unrecognised_and_falls_back_to_raw() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "foo[0];");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    assert!(matches!(head, HeadExpression::Raw { .. }));
}

#[test]
fn call_expression_collapses_to_call_head() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "foo();");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::Call { callee } => {
            assert!(matches!(*callee, HeadExpression::Identifier { name } if name == "foo"))
        }
        _ => panic!("expected call head"),
    }
}

#[test]
fn new_expression_collapses_to_new_head() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "new Foo();");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::New { callee } => {
            assert!(matches!(*callee, HeadExpression::Identifier { name } if name == "Foo"))
        }
        _ => panic!("expected new head"),
    }
}

#[test]
fn await_expression_collapses_to_await_head() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "async function f() { await foo; }");
    let func = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::FunctionDeclaration(f) => f,
        _ => unreachable!(),
    };
    let stmt = func
        .body
        .as_ref()
        .expect("function declaration has a body (test fixture is not abstract)")
        .statements
        .first()
        .expect("function body has at least one statement (test source)");
    let expr = match stmt {
        Statement::ExpressionStatement(es) => &es.expression,
        _ => unreachable!(),
    };
    let head = build_head_expression(Some(expr), Span::new(0, 0));
    match head {
        HeadExpression::Await { argument } => {
            assert!(matches!(*argument, HeadExpression::Identifier { name } if name == "foo"))
        }
        _ => panic!("expected await head"),
    }
}

#[test]
fn assignment_expression_collapses_to_assign_head() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "x = y;");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::Assign {
            operator,
            left,
            right,
        } => {
            assert!(matches!(operator, AssignOperator::Assign));
            assert!(matches!(left.head, HeadExpression::Identifier { name } if name == "x"));
            assert!(matches!(right.head, HeadExpression::Identifier { name } if name == "y"));
        }
        _ => panic!("expected assign head"),
    }
}

#[test]
fn compound_assignment_preserves_operator() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "x += y;");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::Assign { operator, .. } => {
            assert!(matches!(operator, AssignOperator::AddAssign));
        }
        _ => panic!("expected assign head"),
    }
}

#[test]
fn assignment_operator_translation_covers_every_variant() {
    // The 16 entries below pair every JavaScript / TypeScript assignment operator with the
    // `AssignOperator` variant `convert_assign_operator` is expected to
    // produce. The list mirrors `oxc_syntax::operator::AssignmentOperator`
    // exhaustively so a mismatch in the table is caught.
    let cases: &[(&str, AssignOperator)] = &[
        ("x = y;", AssignOperator::Assign),
        ("x += y;", AssignOperator::AddAssign),
        ("x -= y;", AssignOperator::SubAssign),
        ("x *= y;", AssignOperator::MulAssign),
        ("x /= y;", AssignOperator::DivAssign),
        ("x %= y;", AssignOperator::RemAssign),
        ("x **= y;", AssignOperator::ExpAssign),
        ("x <<= y;", AssignOperator::ShlAssign),
        ("x >>= y;", AssignOperator::ShrAssign),
        ("x >>>= y;", AssignOperator::UnsignedShrAssign),
        ("x |= y;", AssignOperator::BitOrAssign),
        ("x ^= y;", AssignOperator::BitXorAssign),
        ("x &= y;", AssignOperator::BitAndAssign),
        ("x ||= y;", AssignOperator::LogicalOrAssign),
        ("x &&= y;", AssignOperator::LogicalAndAssign),
        ("x ??= y;", AssignOperator::NullishAssign),
    ];
    for (src, expected) in cases {
        let alloc = Allocator::default();
        let program = parse_ts(&alloc, src);
        let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
        match head {
            HeadExpression::Assign { operator, .. } => {
                assert_eq!(&operator, expected, "operator mismatch for source `{src}`");
            }
            _ => panic!("expected assign head for source `{src}`"),
        }
    }
}

#[test]
fn assignment_with_elided_rhs_collapses_to_raw_when_both_sides_elided() {
    // Both sides reduce to elided (`[a]` is a destructuring pattern which we
    // don't reduce, `[1]` is an array literal which we don't reduce either),
    // so the whole expression falls back to raw.
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "[a] = [1];");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    assert!(matches!(head, HeadExpression::Raw { .. }));
}

#[test]
fn update_expression_collapses_to_update_head() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "x++;");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::Update {
            operator,
            prefix,
            argument,
        } => {
            assert!(matches!(operator, UpdateOperator::Increment));
            assert!(!prefix);
            assert!(matches!(argument.head, HeadExpression::Identifier { name } if name == "x"));
        }
        _ => panic!("expected update head"),
    }
}

#[test]
fn prefix_update_carries_prefix_flag() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "--x;");
    let head = build_head_expression(Some(expression_of(&program)), fallback_span(&program));
    match head {
        HeadExpression::Update {
            operator, prefix, ..
        } => {
            assert!(matches!(operator, UpdateOperator::Decrement));
            assert!(prefix);
        }
        _ => panic!("expected update head"),
    }
}

#[test]
fn unrecognised_expression_falls_back_to_raw_with_expression_span() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "1 + 2;");
    let expr = expression_of(&program);
    let head = build_head_expression(Some(expr), Span::new(99, 100));
    match head {
        HeadExpression::Raw {
            start_offset,
            end_offset,
        } => {
            assert_eq!(start_offset.0, expr.span().start);
            assert_eq!(end_offset.0, expr.span().end);
        }
        _ => panic!("expected raw head"),
    }
}

#[test]
fn missing_expression_falls_back_to_raw_with_fallback_span() {
    let head = build_head_expression(None, Span::new(7, 13));
    match head {
        HeadExpression::Raw {
            start_offset,
            end_offset,
        } => {
            assert_eq!(start_offset.0, 7);
            assert_eq!(end_offset.0, 13);
        }
        _ => panic!("expected raw head"),
    }
}
