//! Emits "callback result" edges.
//!
//! When a function scope is the argument of a call whose result
//! initializes (or is assigned to) a variable -- the
//! `const xs = arr.map((v) => v.id)` shape -- the callback and the
//! result variable belong to the same statement, yet the analyzer's
//! owner resolution cuts the callback off at the function boundary
//! (see `owner::find_reference_owners`): no owner crosses into a
//! function body, so the callback scope carries no edge to `xs` and
//! renders as a disconnected island far from the variable it feeds.
//!
//! The receiver (or the callee, for a bare `fn(cb)` call) is *outside*
//! that boundary, so its reference already owns the result variable
//! (`arr -> xs`). This pass reuses that fact: it locates the call's
//! anchor reference -- the receiver of `recv.method(cb)` or the callee
//! of `fn(cb)` -- and draws an edge from the callback's subgraph to
//! every result variable the anchor owns, at the same granularity as
//! the existing receiver edge. Callbacks whose call result is not
//! bound to a variable (e.g. `await xs.reduce(cb, init)` as a bare
//! statement) have no owner on the anchor and therefore get no edge.

use unsnarl_ir::serialized::SerializedHeadExpression;

use super::context::BuilderContext;
use super::is_function_subgraph::is_function_subgraph;
use super::node_id::node_id;
use super::push_edge::push_edge;
use super::state::BuildState;
use super::subgraph_scope_id::subgraph_scope_id;

/// Which reference in the enclosing call carries the `owners` (result
/// variables) the callback should connect to, plus the label to
/// render on the edge.
struct Anchor<'a> {
    /// Identifier name of the anchor reference: the call receiver's
    /// object (`recv` in `recv.method(cb)`) or the called function
    /// itself (`fn` in `fn(cb)`).
    name: &'a str,
    /// Edge label: the method name for `recv.method(cb)`, or the
    /// function name for `fn(cb)`.
    label: &'a str,
    /// `true` when the anchor must be a call receiver
    /// (`recv.method(cb)`); `false` when it is the callee (`fn(cb)`).
    want_receiver: bool,
}

/// Classify the call's `callee` head subtree into an [`Anchor`].
///
/// Only plain-identifier receivers / callees yield an anchor: a
/// computed or chained callee object (`a.b().c(cb)`, `a[k].m(cb)`)
/// has no single owning variable reference to borrow owners from, so
/// it is left unconnected.
fn anchor_of(callee: &SerializedHeadExpression) -> Option<Anchor<'_>> {
    match callee {
        SerializedHeadExpression::Member { object, property } => match object.as_ref() {
            SerializedHeadExpression::Identifier { name } => Some(Anchor {
                name,
                label: property,
                want_receiver: true,
            }),
            _ => None,
        },
        SerializedHeadExpression::Identifier { name } => Some(Anchor {
            name,
            label: name,
            want_receiver: false,
        }),
        _ => None,
    }
}

pub fn emit_callback_result_edges(state: &mut BuildState, ctx: &BuilderContext<'_>) {
    // Walk `ir.scopes` in source order so the emitted edge order stays
    // stable against the mermaid / json parity baselines.
    for scope in &ctx.ir.scopes {
        if !is_function_subgraph(scope) {
            continue;
        }
        // A collapsed scope has no rendered subgraph to anchor the
        // edge on; skip it the same way `emit_reference_edges` skips
        // references from collapsed scopes.
        if state.collapsed_root_by_scope.contains_key(scope.id.value()) {
            continue;
        }
        let Some(cb) = scope.callback_argument.as_ref() else {
            continue;
        };
        let Some(anchor) = anchor_of(&cb.callee) else {
            continue;
        };
        let Some(upper) = scope.upper.as_ref() else {
            continue;
        };
        let callback_offset = scope.block.span.offset.0;

        // The anchor reference is evaluated in the callback's
        // enclosing scope, immediately before the callback argument.
        // Of the candidate references (matching name, scope, and
        // receiver/call role) take the one nearest the callback -- the
        // largest offset still preceding it -- so chained or repeated
        // names resolve to this call's receiver rather than an earlier
        // one.
        let anchor_ref = ctx
            .ir
            .references
            .iter()
            .filter(|r| {
                r.identifier.name() == anchor.name
                    && r.from.value() == upper.value()
                    && r.identifier.span().offset.0 < callback_offset
                    && if anchor.want_receiver {
                        r.flags.receiver
                    } else {
                        r.flags.call
                    }
            })
            .max_by_key(|r| r.identifier.span().offset.0);

        let Some(anchor_ref) = anchor_ref else {
            continue;
        };

        let from = subgraph_scope_id(scope);
        for owner in &anchor_ref.owners {
            // Mirror the self-owner guard in `emit_reference_edges`: a
            // reference that owns its own resolved binding would draw a
            // self-loop.
            if anchor_ref
                .resolved
                .as_ref()
                .is_some_and(|res| res.value() == owner.value())
            {
                continue;
            }
            let to = node_id(owner.value());
            push_edge(
                &mut state.emitted_edges,
                &mut state.edges,
                &from,
                anchor.label,
                &to,
            );
        }
    }
}

#[cfg(test)]
#[path = "emit_callback_result_edges_test.rs"]
mod emit_callback_result_edges_test;
