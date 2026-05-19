//! Materialised Identifier / JSXIdentifier node.

use oxc_span::Span;

use unsnarl_oxc_parity::AstType;

#[derive(Clone)]
pub struct AstIdentifier {
    pub r#type: AstType,
    name: String,
    pub span: Span,
}

impl AstIdentifier {
    pub fn new(r#type: AstType, name: String, span: Span) -> Self {
        assert!(!name.is_empty(), "AstIdentifier.name must be non-empty");
        Self { r#type, name, span }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
