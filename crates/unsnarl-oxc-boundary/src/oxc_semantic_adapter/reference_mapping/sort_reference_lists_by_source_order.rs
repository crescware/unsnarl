//! Sort each scope's / variable's reference lists by source offset so
//! the serialized IR matches the parity baseline's source order.

use oxc_index::IndexVec;

use unsnarl_ir::ids::{ReferenceId, ScopeId, VariableId};
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{ScopeData, VariableData};

/// Sort each scope's `references` / `through` list and each variable's
/// `references` list by the underlying identifier's source offset.
///
/// The parity baseline lists references in source order, but the
/// symbol-keyed reference walk plus the later AST-walking synthesis
/// passes leave these lists interleaved by category rather than by
/// source position. The IR emitter
/// [`unsnarl_emitter_ir::serializer::flat`] preserves these lists'
/// order, so without this final sort the serialized
/// `scope.references` / `scope.through` / `variable.references` lists
/// would emit out-of-order ids relative to the parity baseline.
pub(super) fn sort_reference_lists_by_source_order(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &IndexVec<ReferenceId, ReferenceData>,
) {
    let key = |r: &ReferenceId| references[*r].identifier.span.start;
    for scope in scopes.iter_mut() {
        scope.references.sort_by_key(key);
        scope.through.sort_by_key(key);
    }
    for var in variables.iter_mut() {
        var.references.sort_by_key(key);
    }
}
