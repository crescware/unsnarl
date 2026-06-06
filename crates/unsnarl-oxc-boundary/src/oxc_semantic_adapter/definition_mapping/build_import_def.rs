//! Build the `ImportBinding` definition for an import-specifier anchor.

use oxc_ast::AstKind;

use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

pub(super) fn build_import_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
    spec_node: AstNode,
    imported_name: Option<String>,
) -> DefinitionData {
    let (parent, import_source) = match nodes.parent_kind(node_id) {
        AstKind::ImportDeclaration(decl) => (
            Some(AstNode::new(AstType::ImportDeclaration, decl.span)),
            Some(decl.source.value.as_str().to_string()),
        ),
        _ => (None, None),
    };
    DefinitionData {
        r#type: DefinitionType::ImportBinding,
        name: identifier,
        node: spec_node,
        parent,
        init: None,
        declaration_kind: None,
        import_source,
        imported_name,
    }
}
