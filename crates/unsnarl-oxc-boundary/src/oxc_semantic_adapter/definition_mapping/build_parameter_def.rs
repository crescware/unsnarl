//! Build the `Parameter` definition for a `FormalParameter` anchor,
//! anchored to the surrounding function node.

use oxc_ast::AstKind;

use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use super::function_ast_type::function_ast_type;

pub(super) fn build_parameter_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
) -> Option<DefinitionData> {
    let owner = enclosing_function_node(nodes, node_id)?;
    Some(DefinitionData {
        r#type: DefinitionType::Parameter,
        name: identifier,
        node: owner,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    })
}

fn enclosing_function_node(
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_syntax::node::NodeId,
) -> Option<AstNode> {
    for ancestor in nodes.ancestor_kinds(node_id) {
        match ancestor {
            AstKind::Function(f) => {
                return Some(AstNode::new(function_ast_type(f), f.span));
            }
            AstKind::ArrowFunctionExpression(arrow) => {
                return Some(AstNode::new(AstType::ArrowFunctionExpression, arrow.span));
            }
            _ => {}
        }
    }
    None
}
