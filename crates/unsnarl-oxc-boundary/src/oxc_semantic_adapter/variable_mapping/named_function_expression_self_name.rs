//! Detect the self-name of a named function expression.

use oxc_ast::ast::FunctionType;
use oxc_ast::AstKind;

/// If `anchor` is the `Function` node of a named function expression,
/// return its self-name. Used by [`super::build_variables`] to detect
/// the binding that must be skipped per the parity baseline (see the
/// module header).
pub(super) fn named_function_expression_self_name<'a>(anchor: &'a AstKind<'_>) -> Option<&'a str> {
    let AstKind::Function(func) = anchor else {
        return None;
    };
    if !matches!(
        func.r#type,
        FunctionType::FunctionExpression | FunctionType::TSEmptyBodyFunctionExpression
    ) {
        return None;
    }
    func.id.as_ref().map(|id| id.name.as_str())
}
