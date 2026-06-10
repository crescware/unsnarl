//! Sibling tests for [`ExpressionStatementIndex`].

use std::collections::HashMap;

use unsnarl_ir::serialized::{SerializedExpressionStatementContainer, SerializedHeadExpression};

use super::ExpressionStatementIndex;
use crate::builder::builder_fixtures::{base_serialized_reference, span_offset_line};

fn container(start: u32, end: u32) -> SerializedExpressionStatementContainer {
    SerializedExpressionStatementContainer {
        start_span: span_offset_line(start, 1),
        end_span: span_offset_line(end, 1),
        // The head is irrelevant to containment; only the spans matter.
        head: SerializedHeadExpression::identifier("x".to_string()),
        expression_start_span: None,
    }
}

/// Build an index directly from owned containers. Mirrors what
/// [`ExpressionStatementIndex::build`] produces, without threading a
/// full reference list through every containment case.
fn index_of<'a>(
    containers: &'a [SerializedExpressionStatementContainer],
) -> ExpressionStatementIndex<'a> {
    let mut by_offset = HashMap::new();
    for c in containers {
        by_offset.insert(c.start_span.offset.0, c);
    }
    ExpressionStatementIndex { by_offset }
}

#[test]
fn enclosing_returns_none_when_no_statement_is_registered() {
    let index = ExpressionStatementIndex::empty();
    assert!(index.enclosing(10, 20).is_none());
}

#[test]
fn enclosing_returns_the_containing_statement() {
    let containers = [container(0, 30)];
    let index = index_of(&containers);
    let found = index
        .enclosing(5, 25)
        .expect("the statement contains [5, 25]");
    assert_eq!(found.start_span.offset.0, 0);
}

#[test]
fn enclosing_ignores_a_statement_that_ends_before_the_span() {
    let containers = [container(0, 8)];
    let index = index_of(&containers);
    // The callback span [10, 20] starts after the statement ends, so a
    // variable-bound / sibling callback is not grouped under it.
    assert!(index.enclosing(10, 20).is_none());
}

#[test]
fn enclosing_picks_the_innermost_of_nested_statements() {
    let containers = [container(0, 100), container(40, 60)];
    let index = index_of(&containers);
    // [45, 55] is contained by both; the innermost (largest start) wins.
    let found = index
        .enclosing(45, 55)
        .expect("both statements contain [45, 55]");
    assert_eq!(found.start_span.offset.0, 40);
}

#[test]
fn enclosing_treats_exact_boundary_spans_as_contained() {
    let containers = [container(10, 20)];
    let index = index_of(&containers);
    let found = index
        .enclosing(10, 20)
        .expect("exact boundary counts as contained");
    assert_eq!(found.start_span.offset.0, 10);
}

#[test]
fn build_indexes_only_references_carrying_a_container() {
    // A reference whose nearest statement is a synthetic arrow body
    // leaves `expression_statement_container` as `None` upstream; such
    // references must not contribute an entry, so a span they would
    // have "enclosed" resolves to the real statement only.
    let mut with_container = base_serialized_reference();
    with_container.expression_statement_container = Some(container(0, 30));
    let without_container = base_serialized_reference(); // None by default

    let references = [with_container, without_container];
    let index = ExpressionStatementIndex::build(&references);

    let found = index
        .enclosing(5, 25)
        .expect("the registered statement contains [5, 25]");
    assert_eq!(found.start_span.offset.0, 0);
    // Nothing outside the single registered statement is reachable.
    assert!(index.enclosing(50, 60).is_none());
}
