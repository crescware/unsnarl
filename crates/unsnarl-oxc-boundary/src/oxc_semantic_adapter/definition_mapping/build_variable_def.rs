//! Build the `Variable` definition for a `VariableDeclarator` anchor.

use oxc_ast::ast::VariableDeclarationKind;
use oxc_ast::AstKind;

use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind as IrVariableDeclarationKind};

use crate::materialise::ast_node_of_expression;

pub(super) fn build_variable_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
    vd: &oxc_ast::ast::VariableDeclarator<'_>,
) -> DefinitionData {
    let declarator_node = AstNode::new(AstType::VariableDeclarator, vd.span);
    let init = vd.init.as_ref().map(ast_node_of_expression);
    let (parent, declaration_kind) = match nodes.parent_kind(node_id) {
        AstKind::VariableDeclaration(decl) => (
            Some(AstNode::new(AstType::VariableDeclaration, decl.span)),
            Some(ir_variable_declaration_kind(decl.kind)),
        ),
        _ => (None, None),
    };
    DefinitionData {
        r#type: DefinitionType::Variable,
        name: identifier,
        node: declarator_node,
        parent,
        init,
        declaration_kind,
        import_source: None,
        imported_name: None,
    }
}

fn ir_variable_declaration_kind(kind: VariableDeclarationKind) -> IrVariableDeclarationKind {
    match kind {
        VariableDeclarationKind::Var => IrVariableDeclarationKind::Var,
        VariableDeclarationKind::Let => IrVariableDeclarationKind::Let,
        VariableDeclarationKind::Const => IrVariableDeclarationKind::Const,
        VariableDeclarationKind::Using => IrVariableDeclarationKind::Using,
        VariableDeclarationKind::AwaitUsing => IrVariableDeclarationKind::AwaitUsing,
    }
}
