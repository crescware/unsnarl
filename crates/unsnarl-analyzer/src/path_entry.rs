//! Path-entry carrier used by analyzer functions that inspect an
//! ancestor chain.
//!
//! Mirrors the TS `PathEntry` shape (`{ node: AstNode, key: string | null }`).
//! The Rust port carries the materialised [`AstNode`] (type + span)
//! plus the slot key on the parent, which is all the path-traversing
//! analyzers (`find_predicate_container`, `find_jsx_element_span`,
//! `find_completion`, `find_expression_statement_container`,
//! `if_chain_root_offset`, `find_reference_owners`) actually inspect.
//!
//! `arrow_body` is a side-channel populated only for
//! `ArrowFunctionExpression` entries. The TS analyzer's
//! `findCompletion` reads `entry.node.body` to distinguish
//! expression-body arrows from block-body arrows; the Rust port pre-
//! materialises that single piece of structural information at
//! visitor time so the analyzer surface stays in terms of
//! `(AstType, Span, key)` tuples.

use oxc_span::Span;

use unsnarl_ir::primitive::AstNode;

#[derive(Clone)]
pub struct PathEntry {
    pub node: AstNode,
    pub key: Option<&'static str>,
    pub arrow_body: Option<ArrowBodyInfo>,
}

#[derive(Clone, Copy)]
pub struct ArrowBodyInfo {
    pub span: Span,
    pub is_block: bool,
}
