//! Sibling tests for [`enclosing_statement_offset`].

use std::collections::HashMap;

use unsnarl_ir::serialized::{SerializedExpressionStatementContainer, SerializedHeadExpression};

use super::enclosing_statement_offset;
use crate::builder::builder_fixtures::span_offset_line;

fn container(start: u32, end: u32) -> SerializedExpressionStatementContainer {
    SerializedExpressionStatementContainer {
        start_span: span_offset_line(start, 1),
        end_span: span_offset_line(end, 1),
        // The head is irrelevant to containment; only the spans matter.
        head: SerializedHeadExpression::identifier("x".to_string()),
    }
}

#[test]
fn returns_none_when_no_statement_is_registered() {
    let map: HashMap<u32, &SerializedExpressionStatementContainer> = HashMap::new();
    assert_eq!(enclosing_statement_offset(10, 20, &map), None);
}

#[test]
fn returns_the_offset_of_a_containing_statement() {
    let c = container(0, 30);
    let mut map = HashMap::new();
    map.insert(0u32, &c);
    assert_eq!(enclosing_statement_offset(5, 25, &map), Some(0));
}

#[test]
fn ignores_a_statement_that_ends_before_the_span() {
    let c = container(0, 8);
    let mut map = HashMap::new();
    map.insert(0u32, &c);
    // The callback span [10, 20] starts after the statement ends, so
    // a variable-bound / sibling callback is not grouped under it.
    assert_eq!(enclosing_statement_offset(10, 20, &map), None);
}

#[test]
fn picks_the_innermost_of_nested_statements() {
    let outer = container(0, 100);
    let inner = container(40, 60);
    let mut map = HashMap::new();
    map.insert(0u32, &outer);
    map.insert(40u32, &inner);
    // [45, 55] is contained by both; the innermost (largest start) wins.
    assert_eq!(enclosing_statement_offset(45, 55, &map), Some(40));
}

#[test]
fn exact_boundary_spans_count_as_contained() {
    let c = container(10, 20);
    let mut map = HashMap::new();
    map.insert(10u32, &c);
    assert_eq!(enclosing_statement_offset(10, 20, &map), Some(10));
}
