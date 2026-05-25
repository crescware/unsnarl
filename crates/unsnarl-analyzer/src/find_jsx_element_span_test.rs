//! Unit tests for `find_jsx_element_span` driving the path-walking
//! logic directly.
//!
//! These cover the decision function in isolation. Integration
//! coverage of the same fixtures lives in the parity harness.

use unsnarl_oxc_parity::AstType;

use crate::analyzer_fixtures::{ast_node, ast_node_with_end};
use crate::path_entry::PathEntry;

use super::find_jsx_element_span;

#[test]
fn empty_path_returns_none() {
    assert!(find_jsx_element_span(&[]).is_none());
}

#[test]
fn opening_element_with_jsx_element_grandparent_returns_element_span() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::JSXElement, 10, 40), Some("body")),
        PathEntry::new(
            ast_node_with_end(AstType::JSXOpeningElement, 10, 20),
            Some("openingElement"),
        ),
    ];
    let span = find_jsx_element_span(&path).expect("Some");
    assert_eq!(span.start_offset.0, 10);
    assert_eq!(span.end_offset.0, 40);
}

#[test]
fn opening_element_at_path_root_returns_none() {
    let path = vec![PathEntry::new(
        ast_node_with_end(AstType::JSXOpeningElement, 10, 20),
        Some("openingElement"),
    )];
    assert!(find_jsx_element_span(&path).is_none());
}

#[test]
fn opening_element_without_jsx_element_grandparent_returns_none() {
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::JSXOpeningElement, 10, 20),
            Some("openingElement"),
        ),
    ];
    assert!(find_jsx_element_span(&path).is_none());
}

#[test]
fn member_expression_segment_is_walked_through() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::JSXElement, 10, 40), Some("body")),
        PathEntry::new(
            ast_node_with_end(AstType::JSXOpeningElement, 10, 30),
            Some("openingElement"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::JSXMemberExpression, 11, 22),
            Some("name"),
        ),
    ];
    let span = find_jsx_element_span(&path).expect("Some");
    assert_eq!(span.start_offset.0, 10);
    assert_eq!(span.end_offset.0, 40);
}

#[test]
fn unrelated_innermost_entry_returns_none() {
    let path = vec![
        PathEntry::new(ast_node_with_end(AstType::JSXElement, 10, 40), Some("body")),
        PathEntry::new(
            ast_node_with_end(AstType::JSXOpeningElement, 10, 30),
            Some("openingElement"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::JSXAttribute, 12, 28),
            Some("attribute"),
        ),
    ];
    assert!(find_jsx_element_span(&path).is_none());
}
