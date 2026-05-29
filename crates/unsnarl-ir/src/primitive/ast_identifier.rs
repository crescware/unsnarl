//! Materialised Identifier / JSXIdentifier node.

use oxc_span::Span;

use unsnarl_oxc_parity::AstType;

use crate::non_empty::assert_non_empty;

#[derive(Clone)]
pub struct AstIdentifier {
    pub r#type: AstType,
    name: String,
    pub span: Span,
}

impl AstIdentifier {
    pub fn new(r#type: AstType, name: String, span: Span) -> Self {
        assert_non_empty(&name, "AstIdentifier.name");
        Self { r#type, name, span }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
