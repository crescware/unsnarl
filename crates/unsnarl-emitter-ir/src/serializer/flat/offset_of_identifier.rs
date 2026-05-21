//! Read the start offset off an IR `AstIdentifier`.
//!
//! Mirrors the `AstIdentifier` arm of `offsetOf` in
//! `ts/src/serializer/flat/offset-of.ts`. The TS source falls back
//! to `0` when `node.start` is absent; in the Rust IR the span is
//! always present, so the fallback collapses to "take `span.start`".
//!
//! This is the production-hot arm: `flat_serializer.rs` calls it for
//! every reference identifier, so the parity sweep exercises it
//! tens of thousands of times. Keeping it separate from the
//! `AstNode` arm in `offset_of_node.rs` makes that split visible
//! from the coverage report.

use unsnarl_ir::primitive::AstIdentifier;

pub fn offset_of_identifier(node: &AstIdentifier) -> u32 {
    node.span.start
}

#[cfg(test)]
#[path = "offset_of_identifier_test.rs"]
mod offset_of_identifier_test;
