//! Decide whether an `IdentifierReference` / `JSXIdentifier` node sits
//! in a slot the parity baseline treats as a non-reference (skip) site.

use oxc_ast::AstKind;

/// Decide whether a reference node sits in a slot that the parity
/// baseline treats as a non-reference (skip) site.
///
/// `oxc_semantic` emits `Reference` rows for identifiers that the
/// parity baseline drops. Two predicates are checked:
///
/// 1. Immediate-parent JSX skip: identifiers nested directly under a
///    `JSXClosingElement` duplicate the opening tag's reference.
/// 2. Ancestor walk for type-only enclosures: any ancestor among
///    `TSImportEqualsDeclaration` / `TSExportAssignment` /
///    `TSNamespaceExportDeclaration` makes the reference type-only
///    even when `ReferenceFlags::Type` is not set (oxc treats the
///    `x` in `export = x` as a value reference, but the parity
///    baseline drops it). The other type-only `AstKind`s
///    (`TSInterfaceDeclaration`, `TSTypeAliasDeclaration`,
///    `TSEnumDeclaration`, `TSDeclareFunction`, etc.) are already
///    pruned via `scope_mapping::is_filtered_out`.
pub(super) fn reference_is_skip_slot(
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_semantic::NodeId,
) -> bool {
    let parent_kind = nodes.parent_kind(node_id);
    if matches!(parent_kind, AstKind::JSXClosingElement(_)) {
        return true;
    }
    let mut cur = nodes.parent_id(node_id);
    loop {
        if matches!(
            nodes.kind(cur),
            AstKind::TSImportEqualsDeclaration(_)
                | AstKind::TSExportAssignment(_)
                | AstKind::TSNamespaceExportDeclaration(_),
        ) {
            return true;
        }
        let next = nodes.parent_id(cur);
        if next == cur {
            // `parent_id` is a self-loop at the program root.
            return false;
        }
        cur = next;
    }
}
