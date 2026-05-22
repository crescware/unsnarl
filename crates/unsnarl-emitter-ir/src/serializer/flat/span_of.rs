//! `AstNode` / `AstIdentifier` → `Span` (line / column / offset).
//!
//! The IR span is always present, so this reads `span.start`
//! directly.

use unsnarl_ir::primitive::{span_from_offset, AstIdentifier, AstNode, Span};

pub fn span_of_node(node: &AstNode, raw: &str) -> Span {
    span_from_offset(raw, node.span.start as usize)
}

pub fn span_of_identifier(node: &AstIdentifier, raw: &str) -> Span {
    span_from_offset(raw, node.span.start as usize)
}

#[cfg(test)]
#[path = "span_of_test.rs"]
mod span_of_test;
