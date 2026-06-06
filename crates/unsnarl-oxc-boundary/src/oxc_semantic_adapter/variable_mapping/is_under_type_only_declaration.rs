//! Detect whether a symbol's declaration sits under a TypeScript
//! type-only construct.

use oxc_ast::AstKind;
use oxc_semantic::Scoping;
use oxc_syntax::symbol::SymbolId;

/// Returns true if any ancestor of `symbol_id`'s declaration node is
/// a TypeScript type-only construct that
/// [`unsnarl_oxc_parity::is_type_only_subtree`] marks as type-only.
/// The parity baseline never declares the inner binding in such
/// subtrees, so the variable is omitted from the IR variable list.
pub(super) fn is_under_type_only_declaration(
    scoping: &Scoping,
    nodes: &oxc_semantic::AstNodes<'_>,
    symbol_id: SymbolId,
) -> bool {
    let mut cur = scoping.symbol_declaration(symbol_id);
    loop {
        if matches!(
            nodes.kind(cur),
            AstKind::TSImportEqualsDeclaration(_)
                | AstKind::TSExportAssignment(_)
                | AstKind::TSNamespaceExportDeclaration(_)
                | AstKind::TSInterfaceDeclaration(_)
                | AstKind::TSTypeAliasDeclaration(_)
                | AstKind::TSEnumDeclaration(_)
                | AstKind::TSModuleDeclaration(_),
        ) {
            return true;
        }
        let next = nodes.parent_id(cur);
        if next == cur {
            return false;
        }
        cur = next;
    }
}
