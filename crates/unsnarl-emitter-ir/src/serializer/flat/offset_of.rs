//! Read the start offset off an IR `AstNode` / `AstIdentifier`.
//!
//! Mirrors `offsetOf` in `ts/src/serializer/flat/offset-of.ts`. The
//! TS source falls back to `0` when `node.start` is absent; in the
//! Rust IR the span is always present, so the fallback collapses to
//! "take `span.start`".

use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_ir::primitive::AstNode;

pub fn offset_of_node(node: &AstNode) -> u32 {
    node.span.start
}

pub fn offset_of_identifier(node: &AstIdentifier) -> u32 {
    node.span.start
}
