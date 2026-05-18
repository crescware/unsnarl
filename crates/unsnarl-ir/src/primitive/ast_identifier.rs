//! IR-side materialised Identifier / JSXIdentifier node. Ports
//! `ts/src/ir/primitive/ast-identifier.ts`.
//!
//! TS expresses this as `AstNode & { type: Identifier | JSXIdentifier;
//! name: string }`. The Rust port carries the same data in a flat
//! struct; consumers that need the AstNode shape can pull `(r#type,
//! span)` out.

use oxc_span::Span;

use crate::ast_type::AstType;

pub struct AstIdentifier {
    pub r#type: AstType,
    pub name: String,
    pub span: Span,
}
