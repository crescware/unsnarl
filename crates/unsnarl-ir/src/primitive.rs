//! IR-side primitive types.

pub mod ast_identifier;
pub mod ast_node;
pub mod span;

pub use ast_identifier::AstIdentifier;
pub use ast_node::AstNode;
pub use span::{span_from_offset, SourceColumn, SourceLine, SourceOffset, Span};
