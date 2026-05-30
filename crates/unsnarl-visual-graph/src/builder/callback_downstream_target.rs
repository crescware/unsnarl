//! Resolves the "result variable" that a callback's enclosing call
//! binds its result to.
//!
//! When a call with a callback argument is *not* a bare statement
//! (`const xs = arr.map(cb)`, `const v = useMemo(cb, deps)`), the call
//! subgraph is placed beside the result variable by containment (a
//! shared `wrap_` box) instead of being left as a disconnected island.
//! To do that the builder needs the result variable.
//!
//! The analyzer cuts the callback off from that variable at the
//! function boundary (`owner::find_reference_owners`), so the callback
//! itself carries no owner. But the call's *anchor* reference -- the
//! receiver of `recv.method(cb)` or the callee of `fn(cb)` -- lives
//! outside the boundary and already owns the result variable
//! (`arr -> xs`). This module reads the result variable back off that
//! anchor reference's `owners`, purely in the visual-graph layer (no
//! IR change).

use unsnarl_ir::serialized::{SerializedDefinition, SerializedHeadExpression, SerializedScope};

use super::context::BuilderContext;

/// Which reference in the enclosing call carries the result
/// variable's id in its `owners`.
struct Anchor<'a> {
    /// Identifier name: the receiver object (`recv` in
    /// `recv.method(cb)`) or the callee (`fn` in `fn(cb)`).
    name: &'a str,
    /// `true` when the anchor must be a call receiver, `false` when
    /// it is the callee.
    want_receiver: bool,
}

/// Classify the call's `callee` head subtree into an [`Anchor`].
/// Only plain-identifier receivers / callees resolve: a computed or
/// chained callee object (`a.b().c(cb)`) has no single owning
/// variable reference to read owners from.
fn anchor_of(callee: &SerializedHeadExpression) -> Option<Anchor<'_>> {
    match callee {
        SerializedHeadExpression::Member { object, .. } => match object.as_ref() {
            SerializedHeadExpression::Identifier { name } => Some(Anchor {
                name,
                want_receiver: true,
            }),
            _ => None,
        },
        SerializedHeadExpression::Identifier { name } => Some(Anchor {
            name,
            want_receiver: false,
        }),
        _ => None,
    }
}

/// The downstream attachment point for a result-bound callback call.
pub struct CallbackResultTarget {
    /// Id of the anchor reference whose owner became the result
    /// variable. Used to group multiple callbacks of the *same* call
    /// under one call subgraph, and to suppress the now-redundant
    /// owner edge.
    pub anchor_ref_id: String,
    /// Variable id the call result is bound to.
    pub owner_var_id: String,
}

/// Resolve the result variable a callback scope's call binds to, or
/// `None` when the call is not bound to a single variable (bare
/// statement, `return`, call-in-argument, computed receiver, or a
/// multi-binding destructure -- those keep their existing rendering).
pub fn callback_result_target(
    scope: &SerializedScope,
    ctx: &BuilderContext<'_>,
) -> Option<CallbackResultTarget> {
    let cb = scope.callback_argument.as_ref()?;
    let anchor = anchor_of(&cb.callee)?;
    let upper = scope.upper.as_ref()?;
    let callback_offset = scope.block.span.offset.0;

    // The anchor reference is evaluated in the callback's enclosing
    // scope, immediately before the callback argument. Take the one
    // nearest the callback (largest offset still preceding it) so
    // repeated names resolve to this call's receiver/callee.
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
        .max_by_key(|r| r.identifier.span().offset.0)?;

    // A single owner that is not the anchor's own resolved binding.
    // Multi-owner (destructuring) is left to existing rendering.
    if anchor_ref.owners.len() != 1 {
        return None;
    }
    let owner = anchor_ref.owners.first()?;
    if anchor_ref
        .resolved
        .as_ref()
        .is_some_and(|res| res.value() == owner.value())
    {
        return None;
    }

    // Accept only when this call is the result variable's DIRECT
    // initializer, not a call nested inside an argument. In
    // `const images = outer(data.map(cb))` the analyzer reports
    // `data` as owning `images` too (data flows into images through
    // the calls), but `images` is bound to `outer(...)`, not to
    // `data.map(...)`. The direct call's receiver/callee is the
    // earliest reference owning the result variable within its
    // initializer; if any owner reference starts earlier, this call is
    // nested and the callback keeps its existing (island) rendering.
    let owner_var = ctx.variable_map.get(owner.value())?;
    let init_start = match owner_var.defs.first()? {
        SerializedDefinition::Variable(d) => d.init()?.span.offset.0,
        _ => return None,
    };
    let anchor_offset = anchor_ref.identifier.span().offset.0;
    let earliest_owner_offset = ctx
        .ir
        .references
        .iter()
        .filter(|r| {
            r.identifier.span().offset.0 >= init_start
                && r.owners.iter().any(|o| o.value() == owner.value())
        })
        .map(|r| r.identifier.span().offset.0)
        .min()?;
    if anchor_offset != earliest_owner_offset {
        return None;
    }

    Some(CallbackResultTarget {
        anchor_ref_id: anchor_ref.id.value().to_string(),
        owner_var_id: owner.value().to_string(),
    })
}

#[cfg(test)]
#[path = "callback_downstream_target_test.rs"]
mod callback_downstream_target_test;
