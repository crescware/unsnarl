//! Read the start offset off an IR `AstIdentifier`. The IR span is
//! always present, so this simply returns `span.start`.
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
