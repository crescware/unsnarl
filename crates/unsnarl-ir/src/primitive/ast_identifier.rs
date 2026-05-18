//! Materialised Identifier / JSXIdentifier node.

use oxc_span::Span;

use unsnarl_ast_type::AstType;

pub struct AstIdentifier {
    pub r#type: AstType,
    pub name: String,
    pub span: Span,
}
