//! Compute the IR `upper` for a non-merged, non-filtered oxc scope.

use std::collections::HashMap;

use oxc_index::IndexVec;
use oxc_semantic::{AstNodes, Scoping};
use oxc_span::GetSpan;
use oxc_syntax::scope::ScopeId as OxcScopeId;

use unsnarl_ir::ids::ScopeId;

use super::SwitchInfo;

/// Compute the IR `upper` for a non-merged, non-filtered oxc scope.
///
/// For most scopes the upper is the parent's translated IR id. When
/// the parent's anchor is a `SwitchStatement`, the upper is rewired
/// to the synthetic case `Block` scope whose span encloses this
/// scope's anchor (see [`super::build_scopes`]'s `SwitchCase`
/// synthesis).
pub(super) fn upper_for(
    oxc_id: OxcScopeId,
    scoping: &Scoping,
    nodes: &AstNodes<'_>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    switch_info: &HashMap<OxcScopeId, SwitchInfo>,
) -> Option<ScopeId> {
    let parent_oxc = scoping.scope_parent_id(oxc_id)?;
    let parent_ir = translation[parent_oxc]?;
    if let Some(info) = switch_info.get(&parent_oxc) {
        let anchor_span = nodes.kind(scoping.get_node_id(oxc_id)).span();
        for (case_span, case_ir) in &info.cases {
            if case_span.start <= anchor_span.start && anchor_span.end <= case_span.end {
                return Some(*case_ir);
            }
        }
    }
    Some(parent_ir)
}
