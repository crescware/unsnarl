//! IR-side primitive types.

pub mod ast_identifier;
pub mod ast_node;
pub mod offset;
pub mod source_index;
pub mod span;

pub use ast_identifier::AstIdentifier;
pub use ast_node::AstNode;
pub use offset::{Utf16CodeUnitOffset, Utf8ByteOffset};
pub use source_index::SourceIndex;
pub use span::{span_from_offset, SourceColumn, SourceLine, Span};
