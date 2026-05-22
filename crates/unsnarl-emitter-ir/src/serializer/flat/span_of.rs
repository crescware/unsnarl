//! `AstNode` / `AstIdentifier` → `Span` (line / column / offset).
//!
//! The IR span is always present, so this reads `span.start`
//! directly.

use unsnarl_ir::primitive::{AstIdentifier, AstNode, SourceIndex, Span};

pub fn span_of_node(node: &AstNode, index: &SourceIndex<'_>) -> Span {
    index.span_at(node.span.start as usize)
}

pub fn span_of_identifier(node: &AstIdentifier, index: &SourceIndex<'_>) -> Span {
    index.span_at(node.span.start as usize)
}

#[cfg(test)]
#[path = "span_of_test.rs"]
mod span_of_test;
