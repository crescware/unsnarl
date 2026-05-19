//! `AstNode` / `AstIdentifier` → `Span` (line / column / offset).
//!
//! Mirrors `spanOf` in `ts/src/serializer/flat/span-of.ts`. The TS
//! source falls back to `node.start ?? 0`; the Rust IR span is always
//! present so we read it directly.

use unsnarl_ir::primitive::{span_from_offset, AstIdentifier, AstNode, Span};

pub fn span_of_node(node: &AstNode, raw: &str) -> Span {
    span_from_offset(raw, node.span.start as usize)
}

pub fn span_of_identifier(node: &AstIdentifier, raw: &str) -> Span {
    span_from_offset(raw, node.span.start as usize)
}
