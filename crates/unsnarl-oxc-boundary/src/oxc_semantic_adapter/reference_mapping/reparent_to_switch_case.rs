//! Reparent a reference to the per-case Block scope inside a switch.

use std::collections::HashMap;

use oxc_index::IndexVec;
use oxc_span::Span;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::scope::ScopeData;

/// Reparent a reference to the per-case Block scope inside a switch.
///
/// `oxc_semantic` places the discriminant / case-test identifiers of a
/// `SwitchStatement` in the switch's *parent* scope (the discriminant
/// is evaluated before the switch's body opens) and places case-body
/// identifiers on the bare switch scope without a per-case Block.
/// `super::scope_mapping` synthesises one Block scope per `SwitchCase`,
/// and this helper redirects every reference whose identifier span
/// lies inside any switch in `switch_cases` to the most specific
/// owner (the case-Block scope, falling back to the bare switch
/// scope).
///
/// Walk every recorded switch and collect the innermost (smallest
/// width) match whose:
///
/// * `switch_span` contains the reference's identifier span, *and*
/// * `from` is either the switch scope itself or any ancestor of it
///   (i.e. `from` is not a descendant scope — a function nested inside
///   a case body would have `from` deeper than the switch, and its
///   identifiers must stay inside that function scope).
///
/// For the chosen switch, the case-Block scope is preferred over the
/// bare switch scope whenever the span lies inside a specific case.
/// Nested switches naturally select the deepest one because their
/// `switch_span` is the smallest.
pub(super) fn reparent_to_switch_case(
    from: ScopeId,
    span: Span,
    scopes: &IndexVec<ScopeId, ScopeData>,
    switch_cases: &HashMap<ScopeId, Vec<(Span, ScopeId)>>,
) -> ScopeId {
    let mut best: Option<(u32, ScopeId)> = None;
    for (&switch_ir, cases) in switch_cases {
        let switch_span = scopes[switch_ir].block.span;
        if span.start < switch_span.start || span.end > switch_span.end {
            continue;
        }
        if !is_ancestor_or_self(scopes, switch_ir, from) {
            continue;
        }
        let mut candidate_span = switch_span;
        let mut candidate_ir = switch_ir;
        for (case_span, case_ir) in cases {
            if case_span.start <= span.start && span.end <= case_span.end {
                candidate_span = *case_span;
                candidate_ir = *case_ir;
                break;
            }
        }
        let width = candidate_span.end - candidate_span.start;
        if best.is_none_or(|(w, _)| width < w) {
            best = Some((width, candidate_ir));
        }
    }
    best.map(|(_, s)| s).unwrap_or(from)
}

/// Is `candidate` either `descendant` itself or any of its ancestors
/// walked through `ScopeData::upper`?
fn is_ancestor_or_self(
    scopes: &IndexVec<ScopeId, ScopeData>,
    descendant: ScopeId,
    candidate: ScopeId,
) -> bool {
    let mut cur = Some(descendant);
    while let Some(s) = cur {
        if s == candidate {
            return true;
        }
        cur = scopes[s].upper;
    }
    false
}
