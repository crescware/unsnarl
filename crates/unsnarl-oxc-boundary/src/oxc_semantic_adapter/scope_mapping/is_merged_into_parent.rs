//! Predicate: does this oxc scope merge into its parent's IR row?

use oxc_ast::AstKind;
use oxc_semantic::{AstNodes, Scoping};
use oxc_syntax::scope::ScopeId as OxcScopeId;

/// Predicate: does this oxc scope have no IR row of its own, instead
/// merging into the parent's row?
///
/// Three merge cases:
///
/// * Catch body `BlockStatement`: `oxc_semantic` emits a separate
///   `BlockStatement` scope for `catch (e) { ... }`'s body block,
///   while the parity baseline folds both the catch parameter and the
///   body's declarations into a single `Catch` scope. The body block
///   is detected by its parent in [`Scoping::scope_parent_id`] being
///   a `CatchClause` scope.
/// * `WithStatement`: `oxc_semantic` allocates a dedicated With scope
///   (`ScopeFlags::With`), but the parity baseline carries no
///   separate With scope row — the body's `BlockStatement` is
///   parented directly under the enclosing scope. Mirror that by
///   treating the `WithStatement` scope as merged into its parent;
///   its body block scope (a regular `BlockStatement`) keeps its own
///   IR row with the With's parent as its `upper`.
/// * `StaticBlock`: `oxc_semantic` opens a fresh scope for
///   `class C { static { ... } }`, but the parity baseline keeps the
///   static block's body identifiers in the enclosing `Class` scope.
///   Treat the `StaticBlock` as merged into its `Class` parent so
///   references / bindings inside the static block surface on the
///   class scope.
pub(super) fn is_merged_into_parent(
    oxc_id: OxcScopeId,
    scoping: &Scoping,
    nodes: &AstNodes<'_>,
) -> bool {
    let kind = nodes.kind(scoping.get_node_id(oxc_id));
    if matches!(kind, AstKind::WithStatement(_) | AstKind::StaticBlock(_)) {
        return true;
    }
    let Some(parent) = scoping.scope_parent_id(oxc_id) else {
        return false;
    };
    let parent_kind = nodes.kind(scoping.get_node_id(parent));
    matches!(kind, AstKind::BlockStatement(_)) && matches!(parent_kind, AstKind::CatchClause(_))
}
