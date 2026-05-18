//! Materialised Identifier / JSXIdentifier node.

use oxc_span::Span;

use unsnarl_ast_type::AstType;

pub struct AstIdentifier {
    pub r#type: AstType,
    name: String,
    pub span: Span,
}

impl AstIdentifier {
    pub fn new(r#type: AstType, name: impl Into<String>, span: Span) -> Self {
        let name = name.into();
        assert!(!name.is_empty(), "AstIdentifier.name must be non-empty");
        Self { r#type, name, span }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
