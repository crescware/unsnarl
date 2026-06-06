//! Build the per-occurrence `AstIdentifier` list for a symbol.

use oxc_semantic::Scoping;
use oxc_syntax::symbol::SymbolId;

use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_oxc_parity::AstType;

pub(super) fn build_identifiers(
    scoping: &Scoping,
    symbol_id: SymbolId,
    name: &str,
) -> Vec<AstIdentifier> {
    let redeclarations = scoping.symbol_redeclarations(symbol_id);
    if redeclarations.is_empty() {
        vec![AstIdentifier::new(
            AstType::Identifier,
            name.to_string(),
            scoping.symbol_span(symbol_id),
        )]
    } else {
        redeclarations
            .iter()
            .map(|r| AstIdentifier::new(AstType::Identifier, name.to_string(), r.span))
            .collect()
    }
}
