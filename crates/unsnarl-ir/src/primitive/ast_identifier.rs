//! Materialised Identifier / JSXIdentifier node.

use oxc_span::Span;

use crate::ast_type::AstType;

pub struct AstIdentifier {
    pub r#type: AstType,
    pub name: String,
    pub span: Span,
}
