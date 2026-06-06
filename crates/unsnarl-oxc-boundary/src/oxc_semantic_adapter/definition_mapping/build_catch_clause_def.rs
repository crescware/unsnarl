//! Build the `CatchClause` definition for a `CatchParameter` anchor,
//! anchored to the surrounding catch clause node.

use oxc_ast::AstKind;

use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

pub(super) fn build_catch_clause_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
) -> Option<DefinitionData> {
    let owner = enclosing_catch_clause_node(nodes, node_id)?;
    Some(DefinitionData {
        r#type: DefinitionType::CatchClause,
        name: identifier,
        node: owner,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    })
}

fn enclosing_catch_clause_node(
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_syntax::node::NodeId,
) -> Option<AstNode> {
    for ancestor in nodes.ancestor_kinds(node_id) {
        if let AstKind::CatchClause(c) = ancestor {
            return Some(AstNode::new(AstType::CatchClause, c.span));
        }
    }
    None
}
