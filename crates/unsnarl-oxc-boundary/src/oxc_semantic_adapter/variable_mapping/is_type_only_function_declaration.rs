//! Detect a TypeScript type-only function declaration / overload.

use oxc_ast::ast::FunctionType;
use oxc_ast::AstKind;
use oxc_semantic::Scoping;
use oxc_syntax::symbol::SymbolId;

/// Returns true if `symbol_id`'s declaration is a TypeScript
/// type-only function (`declare function f(): void`, parsed by oxc as
/// `Function { type: TSDeclareFunction, ... }`, or an overload
/// signature parsed as `TSEmptyBodyFunctionExpression`). The parity
/// baseline drops such functions, so the binding never makes it into
/// the IR variable list.
pub(super) fn is_type_only_function_declaration(
    scoping: &Scoping,
    nodes: &oxc_semantic::AstNodes<'_>,
    symbol_id: SymbolId,
) -> bool {
    let kind = nodes.kind(scoping.symbol_declaration(symbol_id));
    let AstKind::Function(func) = kind else {
        return false;
    };
    matches!(
        func.r#type,
        FunctionType::TSDeclareFunction | FunctionType::TSEmptyBodyFunctionExpression
    )
}
