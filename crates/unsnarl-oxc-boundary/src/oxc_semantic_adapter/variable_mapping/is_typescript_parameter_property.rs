//! Detect a TypeScript parameter property declaration.

use oxc_ast::AstKind;
use oxc_semantic::Scoping;
use oxc_syntax::symbol::SymbolId;

/// Returns true if `symbol_id`'s declaration node is a TypeScript
/// parameter property — a `FormalParameter` (or `FormalParameterRest`)
/// carrying `accessibility` / `readonly` / `override`. Those slots
/// represent class fields rather than parameters, so the parity
/// baseline does not record them as parameter bindings.
pub(super) fn is_typescript_parameter_property(
    scoping: &Scoping,
    nodes: &oxc_semantic::AstNodes<'_>,
    symbol_id: SymbolId,
) -> bool {
    let kind = nodes.kind(scoping.symbol_declaration(symbol_id));
    match kind {
        AstKind::FormalParameter(fp) => fp.accessibility.is_some() || fp.readonly || fp.r#override,
        _ => false,
    }
}
