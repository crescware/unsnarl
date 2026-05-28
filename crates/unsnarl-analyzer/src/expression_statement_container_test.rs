use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;
use oxc_span::Span;

use unsnarl_ir::reference::HeadExpression;
use unsnarl_oxc_parity::AstType;

use crate::analyzer_fixtures::{ast_node_with_end, parse_ts};
use crate::path_entry::PathEntry;

use super::{build_expression_statement_container, nearest_expression_statement};

#[test]
fn nearest_expression_statement_finds_innermost_when_multiple_on_path() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::Program, 0, 100), None),
        PathEntry::new(
            ast_node_with_end(AstType::ExpressionStatement, 10, 80),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ArrowFunctionExpression, 20, 70),
            Some("expression"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ExpressionStatement, 30, 60),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 30, 33),
            Some("expression"),
        ),
    ];
    let nearest = nearest_expression_statement(&path).expect("Some");
    assert_eq!(nearest.node.span.start, 30);
    assert_eq!(nearest.node.span.end, 60);
}

#[test]
fn nearest_expression_statement_returns_none_when_path_has_no_expression_statement() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::Program, 0, 100), None),
        PathEntry::new(
            ast_node_with_end(AstType::IfStatement, 10, 80),
            Some("body"),
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 13, 14), Some("test")),
    ];
    assert!(nearest_expression_statement(&path).is_none());
}

#[test]
fn build_container_uses_statement_span_and_head_from_expression() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "foo;");
    let stmt = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::ExpressionStatement(es) => es,
        _ => unreachable!(),
    };
    let container = build_expression_statement_container(stmt.span, Some(&stmt.expression));
    assert_eq!(container.start_offset.0, stmt.span.start);
    assert_eq!(container.end_offset.0, stmt.span.end);
    assert!(matches!(
        container.head,
        HeadExpression::Identifier { ref name } if name == "foo"
    ));
}

#[test]
fn build_container_falls_back_to_raw_when_expression_is_unrecognised() {
    let alloc = Allocator::default();
    let program = parse_ts(&alloc, "1 + 2;");
    let stmt = match program
        .body
        .first()
        .expect("test source has at least one top-level statement")
    {
        Statement::ExpressionStatement(es) => es,
        _ => unreachable!(),
    };
    let container = build_expression_statement_container(stmt.span, Some(&stmt.expression));
    assert!(matches!(container.head, HeadExpression::Raw { .. }));
}

#[test]
fn build_container_falls_back_to_raw_when_expression_missing() {
    let container = build_expression_statement_container(Span::new(5, 10), None);
    match container.head {
        HeadExpression::Raw {
            start_offset,
            end_offset,
        } => {
            assert_eq!(start_offset.0, 5);
            assert_eq!(end_offset.0, 10);
        }
        _ => panic!("expected raw head"),
    }
}
