//! Mirrors `ts/src/serializer/flat/span-of.test.ts`. The TS test
//! pinned that `spanOf` reads `node.start` and projects it through
//! `spanFromOffset`. In the Rust IR `node.span.start` is always
//! present (no `?? 0` fallback to test), so the meaningful invariant
//! to pin is "the returned `Span.offset` matches `node.span.start`
//! when the raw source contains that offset".

use oxc_span::Span as OxcSpan;
use unsnarl_ir::primitive::{AstIdentifier, AstNode, SourceOffset};
use unsnarl_oxc_parity::AstType;

use super::{span_of_identifier, span_of_node};

#[test]
fn span_of_node_derives_offset_from_node_span_start() {
    let node = AstNode {
        r#type: AstType::Identifier,
        span: OxcSpan::new(5, 6),
    };
    let span = span_of_node(&node, "abc\ndef");
    assert_eq!(span.offset, SourceOffset(5));
}

#[test]
fn span_of_node_returns_zero_offset_when_start_is_zero() {
    let node = AstNode {
        r#type: AstType::Identifier,
        span: OxcSpan::new(0, 1),
    };
    let span = span_of_node(&node, "abc");
    assert_eq!(span.offset, SourceOffset(0));
}

#[test]
fn span_of_identifier_derives_offset_from_node_span_start() {
    let id = AstIdentifier::new(AstType::Identifier, "x".to_string(), OxcSpan::new(5, 6));
    let span = span_of_identifier(&id, "abc\ndef");
    assert_eq!(span.offset, SourceOffset(5));
}

#[test]
fn span_of_identifier_returns_zero_offset_when_start_is_zero() {
    let id = AstIdentifier::new(AstType::Identifier, "x".to_string(), OxcSpan::new(0, 1));
    let span = span_of_identifier(&id, "abc");
    assert_eq!(span.offset, SourceOffset(0));
}
