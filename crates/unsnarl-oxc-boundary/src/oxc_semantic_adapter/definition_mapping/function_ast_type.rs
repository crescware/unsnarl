//! Map a `Function` node's `FunctionType` to the IR `AstType`.

use oxc_ast::ast::FunctionType;

use unsnarl_oxc_parity::AstType;

pub(super) fn function_ast_type(f: &oxc_ast::ast::Function<'_>) -> AstType {
    match f.r#type {
        FunctionType::FunctionExpression | FunctionType::TSEmptyBodyFunctionExpression => {
            AstType::FunctionExpression
        }
        FunctionType::FunctionDeclaration | FunctionType::TSDeclareFunction => {
            AstType::FunctionDeclaration
        }
    }
}
