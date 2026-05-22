//! Read the start offset off an IR `AstNode`. The IR span is
//! always present, so this simply returns `span.start`.
//!
//! Kept in its own file (and out of `offset_of_identifier.rs`) so
//! the coverage report can show that `pick_variable_offset` — the
//! only production caller of the family — never reaches this
//! identifier-vs-node distinction: every variable head it inspects
//! is an `AstIdentifier`, so `offset_of_node` is exercised only by
//! its sibling unit tests and not by the parity sweep.

use unsnarl_ir::primitive::AstNode;

pub fn offset_of_node(node: &AstNode) -> u32 {
    node.span.start
}

#[cfg(test)]
#[path = "offset_of_node_test.rs"]
mod offset_of_node_test;
