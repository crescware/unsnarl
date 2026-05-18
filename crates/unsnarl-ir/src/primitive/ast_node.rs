//! Materialised AST node carried inside the IR.
//!
//! The IR can't hold parser-owned references (the arena outlives the
//! parser allocation), so the only AST data we keep on each node is
//! what the IR actually reads: the normalised `AstType` plus the
//! span. Richer parser-side fields are re-derived at boundary time.

use oxc_span::Span;

use unsnarl_ast_type::AstType;

pub struct AstNode {
    pub r#type: AstType,
    pub span: Span,
}
