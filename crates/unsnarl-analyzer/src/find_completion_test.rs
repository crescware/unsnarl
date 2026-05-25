use oxc_span::Span;

use unsnarl_ir::reference::ReferenceCompletion;
use unsnarl_oxc_parity::AstType;

use crate::path_entry::PathEntry;
use crate::testing::ast_node_with_end;

use super::find_completion;

fn assert_return(actual: ReferenceCompletion, start: u32, end: u32) {
    match actual {
        ReferenceCompletion::Return {
            start_offset,
            end_offset,
        } => {
            assert_eq!(start_offset.0, start);
            assert_eq!(end_offset.0, end);
        }
        other => panic!(
            "expected Return {{ start: {start}, end: {end} }}, got {other:?}",
            other = describe(&other)
        ),
    }
}

fn assert_throw(actual: ReferenceCompletion, start: u32, end: u32) {
    match actual {
        ReferenceCompletion::Throw {
            start_offset,
            end_offset,
        } => {
            assert_eq!(start_offset.0, start);
            assert_eq!(end_offset.0, end);
        }
        other => panic!(
            "expected Throw {{ start: {start}, end: {end} }}, got {other:?}",
            other = describe(&other)
        ),
    }
}

fn assert_normal(actual: ReferenceCompletion) {
    assert!(
        matches!(actual, ReferenceCompletion::Normal),
        "expected Normal, got {}",
        describe(&actual)
    );
}

fn describe(c: &ReferenceCompletion) -> &'static str {
    match c {
        ReferenceCompletion::Normal => "Normal",
        ReferenceCompletion::Return { .. } => "Return",
        ReferenceCompletion::Throw { .. } => "Throw",
    }
}

#[test]
fn return_completion_when_return_statement_on_path() {
    let path = vec![
        PathEntry::new(
            ast_node_with_end(AstType::FunctionDeclaration, 0, 100),
            None,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 15, 100),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ReturnStatement, 20, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 27, 28),
            Some("argument"),
        ),
    ];
    assert_return(find_completion(&path), 20, 50);
}

#[test]
fn return_completion_with_body_span_for_expression_body_arrow() {
    let body_span = Span::new(30, 50);
    let path = vec![
        PathEntry::with_arrow_body(
            ast_node_with_end(AstType::ArrowFunctionExpression, 10, 60),
            None,
            body_span,
            false,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BinaryExpression, 30, 50),
            Some("body"),
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 30, 31), Some("left")),
    ];
    assert_return(find_completion(&path), 30, 50);
}

#[test]
fn normal_completion_for_block_body_arrow_without_inner_return() {
    let body_span = Span::new(25, 60);
    let path = vec![
        PathEntry::with_arrow_body(
            ast_node_with_end(AstType::ArrowFunctionExpression, 10, 60),
            None,
            body_span,
            true,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 25, 60),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ExpressionStatement, 30, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 30, 31),
            Some("expression"),
        ),
    ];
    assert_normal(find_completion(&path));
}

#[test]
fn prefers_inner_return_over_enclosing_arrow_body() {
    let body_span = Span::new(25, 60);
    let path = vec![
        PathEntry::with_arrow_body(
            ast_node_with_end(AstType::ArrowFunctionExpression, 10, 60),
            None,
            body_span,
            true,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 25, 60),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ReturnStatement, 30, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 37, 38),
            Some("argument"),
        ),
    ];
    assert_return(find_completion(&path), 30, 50);
}

#[test]
fn normal_completion_at_function_declaration_without_inner_exit() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::FunctionDeclaration, 0, 80), None),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 15, 80),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ExpressionStatement, 20, 35),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 20, 21),
            Some("expression"),
        ),
    ];
    assert_normal(find_completion(&path));
}

#[test]
fn normal_completion_for_top_level_identifier_without_exit_ancestor() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::Program, 0, 100), None),
        PathEntry::new(
            ast_node_with_end(AstType::ExpressionStatement, 0, 10),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 0, 5),
            Some("expression"),
        ),
    ];
    assert_normal(find_completion(&path));
}

#[test]
fn throw_completion_when_throw_statement_on_path() {
    let path = vec![
        PathEntry::new(
            ast_node_with_end(AstType::FunctionDeclaration, 0, 100),
            None,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 15, 100),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ThrowStatement, 20, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 26, 27),
            Some("argument"),
        ),
    ];
    assert_throw(find_completion(&path), 20, 50);
}

#[test]
fn throw_completion_for_top_level_throw_without_enclosing_function() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::Program, 0, 60), None),
        PathEntry::new(
            ast_node_with_end(AstType::ThrowStatement, 0, 30),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 6, 7),
            Some("argument"),
        ),
    ];
    assert_throw(find_completion(&path), 0, 30);
}

#[test]
fn stops_at_inner_arrow_boundary_when_throw_is_in_enclosing_function() {
    let body_span = Span::new(28, 85);
    let path = vec![
        PathEntry::new(
            ast_node_with_end(AstType::FunctionDeclaration, 0, 100),
            None,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 15, 100),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ThrowStatement, 20, 90),
            Some("body"),
        ),
        PathEntry::with_arrow_body(
            ast_node_with_end(AstType::ArrowFunctionExpression, 26, 85),
            Some("argument"),
            body_span,
            true,
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 30, 31), Some("body")),
    ];
    assert_normal(find_completion(&path));
}

#[test]
fn stops_at_class_expression_boundary() {
    let path = vec![
        PathEntry::new(
            ast_node_with_end(AstType::FunctionDeclaration, 0, 100),
            None,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 15, 100),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ReturnStatement, 20, 90),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ClassExpression, 27, 85),
            Some("argument"),
        ),
        PathEntry::new(ast_node_with_end(AstType::ClassBody, 33, 85), Some("body")),
        PathEntry::new(
            ast_node_with_end(AstType::PropertyDefinition, 35, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 39, 42),
            Some("value"),
        ),
    ];
    assert_normal(find_completion(&path));
}

#[test]
fn stops_at_class_declaration_boundary() {
    let path = vec![
        PathEntry::new(
            ast_node_with_end(AstType::FunctionDeclaration, 0, 120),
            None,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 15, 120),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ReturnStatement, 20, 110),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ClassDeclaration, 27, 100),
            Some("argument"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Decorator, 27, 31),
            Some("decorators"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 28, 31),
            Some("expression"),
        ),
    ];
    assert_normal(find_completion(&path));
}
