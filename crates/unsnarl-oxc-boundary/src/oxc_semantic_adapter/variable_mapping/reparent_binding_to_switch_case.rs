//! Reparent a binding to the per-case Block scope inside a switch.

use std::collections::HashMap;

use oxc_index::IndexVec;
use oxc_span::Span;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::scope::ScopeData;

/// Reparent a binding to the per-case Block scope inside a switch.
///
/// Only relocate when the binding's declaring scope IS the switch
/// scope itself — i.e. `oxc_semantic` placed the binding directly on
/// the switch row (a `let` / `const` / function declaration written
/// directly under a `case` consequent with no wrapping block). In
/// that situation the parity baseline expects the binding to live on
/// the synthetic per-`SwitchCase` `Block` row, so the adapter picks
/// the case-Block whose span contains the binding (falling back to
/// the bare switch when the binding sits outside every case head).
///
/// Crucially, bindings whose `oxc_semantic` declaring scope is an
/// *ancestor* of a switch must stay put. The motivating case is
/// `function f() { switch (k) { case 1: var x; } }`: `var x` is
/// hoisted to the function scope (matching ECMAScript semantics and
/// `oxc_semantic`'s `Binder` for `VariableDeclarator`), so
/// `iter_bindings_in(function_scope)` yields the symbol with
/// `symbol_span` pointing inside the case. Reparenting it to the
/// case-Block would silently move `x` out of the function scope's
/// `set` / `variables`, breaking lookups from any code outside the
/// switch.
pub(super) fn reparent_binding_to_switch_case(
    ir_scope: ScopeId,
    span: Span,
    scopes: &IndexVec<ScopeId, ScopeData>,
    switch_cases: &HashMap<ScopeId, Vec<(Span, ScopeId)>>,
) -> ScopeId {
    let Some(cases) = switch_cases.get(&ir_scope) else {
        return ir_scope;
    };
    let switch_span = scopes[ir_scope].block.span;
    if span.start < switch_span.start || span.end > switch_span.end {
        return ir_scope;
    }
    for (case_span, case_ir) in cases {
        if case_span.start <= span.start && span.end <= case_span.end {
            return *case_ir;
        }
    }
    ir_scope
}
