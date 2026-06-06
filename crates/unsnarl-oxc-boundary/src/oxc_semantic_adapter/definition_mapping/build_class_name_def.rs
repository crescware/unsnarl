//! Build the `ClassName` definition for a `Class` anchor.

use oxc_ast::ast::ClassType;

use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

pub(super) fn build_class_name_def(
    identifier: AstIdentifier,
    c: &oxc_ast::ast::Class<'_>,
) -> DefinitionData {
    let ty = match c.r#type {
        ClassType::ClassDeclaration => AstType::ClassDeclaration,
        ClassType::ClassExpression => AstType::ClassExpression,
    };
    DefinitionData {
        r#type: DefinitionType::ClassName,
        name: identifier,
        node: AstNode::new(ty, c.span),
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    }
}
