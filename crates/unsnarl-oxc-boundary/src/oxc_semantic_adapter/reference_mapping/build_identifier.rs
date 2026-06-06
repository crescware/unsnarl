//! Build the IR `AstIdentifier` row for a reference's AST node.

use oxc_ast::AstKind;

use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_oxc_parity::AstType;

/// Build the IR `AstIdentifier` row for the reference's AST node.
///
/// An `IdentifierReference` nested under a `JSXMemberExpression` or a
/// `JSXOpeningElement.name` slot carries `AstType::JSXIdentifier`
/// because oxc represents the JSX-tag `<a.b />`'s `a` as
/// `JSXMemberExpressionObject::IdentifierReference` rather than a
/// `JSXIdentifier`. The IR contract still expects the JSX shape on
/// the resulting reference / implicit-global definition rows, so
/// detect the parent here and adjust the type.
pub(super) fn build_identifier(
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_semantic::NodeId,
) -> AstIdentifier {
    let kind = nodes.kind(node_id);
    match kind {
        AstKind::IdentifierReference(ident) => {
            let parent_kind = nodes.parent_kind(node_id);
            let ast_type = if matches!(
                parent_kind,
                AstKind::JSXMemberExpression(_) | AstKind::JSXOpeningElement(_),
            ) {
                AstType::JSXIdentifier
            } else {
                AstType::Identifier
            };
            AstIdentifier::new(ast_type, ident.name.as_str().to_string(), ident.span)
        }
        AstKind::JSXIdentifier(ident) => AstIdentifier::new(
            AstType::JSXIdentifier,
            ident.name.as_str().to_string(),
            ident.span,
        ),
        other => panic!(
            "reference_mapping: expected IdentifierReference or JSXIdentifier at reference node, \
             got {other:?}",
        ),
    }
}
