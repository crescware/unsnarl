//! Build the `FunctionName` definition for a `Function` anchor.

use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::DefinitionType;

use super::function_ast_type::function_ast_type;

pub(super) fn build_function_name_def(
    identifier: AstIdentifier,
    f: &oxc_ast::ast::Function<'_>,
) -> DefinitionData {
    DefinitionData {
        r#type: DefinitionType::FunctionName,
        name: identifier,
        node: AstNode::new(function_ast_type(f), f.span),
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    }
}
