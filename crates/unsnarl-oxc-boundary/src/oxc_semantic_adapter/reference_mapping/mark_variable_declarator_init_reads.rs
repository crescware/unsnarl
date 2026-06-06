//! Stamp `init = true` on the read reference sitting at a
//! `VariableDeclarator`'s immediate `init` identifier position.

use std::collections::HashSet;

use oxc_ast::AstKind;
use oxc_index::IndexVec;

use unsnarl_ir::ids::ReferenceId;
use unsnarl_ir::reference::ReferenceData;

/// Stamp `init = true` on a read reference whose identifier is the
/// *immediate* `init` of a [`oxc_ast::ast::VariableDeclarator`].
///
/// References from the resolved- and unresolved-reference loops default
/// `init` to `false`. Only the immediate-child identifier is flagged;
/// an identifier nested inside a wrapping expression (`const x = a + b`)
/// stays `false`, matching the parity baseline (which flags only
/// identifiers whose immediate parent is a `VariableDeclarator`).
pub(super) fn mark_variable_declarator_init_reads(
    semantic: &oxc_semantic::Semantic<'_>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
) {
    use oxc_ast::ast::Expression;

    let mut init_spans: HashSet<(u32, u32)> = HashSet::new();
    for node in semantic.nodes().iter() {
        let AstKind::VariableDeclarator(vd) = node.kind() else {
            continue;
        };
        let Some(init) = &vd.init else {
            continue;
        };
        if let Expression::Identifier(id) = init {
            init_spans.insert((id.span.start, id.span.end));
        }
    }
    if init_spans.is_empty() {
        return;
    }
    for r in references.iter_mut() {
        if init_spans.contains(&(r.identifier.span.start, r.identifier.span.end)) {
            r.init = true;
        }
    }
}
