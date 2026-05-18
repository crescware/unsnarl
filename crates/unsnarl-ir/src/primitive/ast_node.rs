//! IR-side materialised AST node. Ports `ts/src/ir/primitive/ast-node.ts`.
//!
//! The TS counterpart is `Readonly<{ type: string; start?: number;
//! end?: number; [key: string]: unknown }>` — the raw oxc-parser node
//! shape with arbitrary extra fields. The Rust IR cannot hold the
//! parser-owned object, so we materialise the fields the IR actually
//! reads: `type` (normalised via `as_ast_type`) and the span.

use oxc_span::Span;

use crate::ast_type::AstType;

pub struct AstNode {
    pub r#type: AstType,
    pub span: Span,
}
