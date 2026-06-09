//! Emit per-reference edges (read / write / owner / predicate /
//! completion) for every resolved reference in the IR.

use std::collections::{HashMap, HashSet};

use unsnarl_oxc_parity::AstType;

use super::arena::{BuildArena, Container};
use super::context::BuilderContext;
use super::edge_label_of_ref::edge_label_of_ref;
use super::enclosing_function_var::enclosing_function_var_borrowed;
use super::ensure_expression_statement_node::ensure_expression_statement_node;
use super::find_host_scope_id::find_host_scope_id;
use super::find_host_subgraph::find_host_subgraph;
use super::node_id::node_id;
use super::owner_target_id::owner_target_id;
use super::predicate_target_id::{predicate_target_id_borrowed, PredicateAnchorMaps};
use super::push_edge::push_edge;
use super::read_origins::read_origins;
use super::resolve_read_target_id::resolve_read_target_id;
use super::set_predecessor_of::set_predecessor_of;
use super::state::BuildState;
use super::state_ref_id::state_ref_id;
use super::write_op_node_id::write_op_node_id;

pub fn emit_reference_edges(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    var_var_ids: &HashSet<&str>,
) {
    // Start offsets of every `ConditionalExpression` (shared by its two
    // arms via `parent_span_offset`). A statement whose container starts
    // here is a *bare ternary statement* (`cond ? a : b;`) — its value is
    // discarded — as opposed to a ternary nested in a consumer
    // (`foo(cond ? a : b);`), whose arm values reach the consumer's
    // container instead. Used below to drop a discarded ternary arm's
    // spurious edge.
    let ternary_stmt_starts: HashSet<u32> = ctx
        .scope_map
        .values()
        .filter_map(|s| {
            let c = s.block_context.as_ref()?;
            (matches!(c.parent_type(), AstType::ConditionalExpression)
                && (c.key() == "consequent" || c.key() == "alternate"))
                .then(|| c.parent_span_offset().0)
        })
        .collect();

    for r in &ctx.ir.references {
        let Some(resolved) = r.resolved.as_ref() else {
            continue;
        };
        if var_var_ids.contains(resolved.value()) {
            continue;
        }

        // Refs whose containing scope (or any ancestor) was collapsed.
        if let Some(collapsed_root) = state.collapsed_root_by_scope.get(r.from.value()).cloned() {
            if r.flags.write {
                continue;
            }
            let Some(target) = state.collapsed_anchor_by_root.get(&collapsed_root).cloned() else {
                continue;
            };
            let from_ids = read_origins(
                resolved.value(),
                r.identifier.span().offset.0,
                r.from.value(),
                ctx,
            );
            let label = edge_label_of_ref(r);
            for from_id in &from_ids {
                push_edge(
                    &mut state.emitted_edges,
                    &mut state.edges,
                    from_id,
                    label,
                    &target,
                );
            }
            continue;
        }

        // Predicate-anchor targets.
        let anchors = PredicateAnchorMaps {
            if_test: &state.if_test_anchor_by_offset,
            switch_discriminant: &state.switch_discriminant_anchor_by_offset,
            while_test: &state.while_test_anchor_by_offset,
            do_while_test: &state.do_while_test_anchor_by_offset,
            for_test: &state.for_test_anchor_by_offset,
            conditional_test: &state.conditional_test_anchor_by_offset,
        };
        let predicate_target = predicate_target_id_borrowed(r, &anchors);
        if let Some(target) = predicate_target {
            if !r.flags.write {
                let from_ids = read_origins(
                    resolved.value(),
                    r.identifier.span().offset.0,
                    r.from.value(),
                    ctx,
                );
                let label = edge_label_of_ref(r);
                for from_id in &from_ids {
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        from_id,
                        label,
                        target,
                    );
                }
                continue;
            }
        }
        if predicate_target.is_none() && r.predicate_container.is_some() && !r.flags.write {
            if let Some(pc) = r.predicate_container.as_ref() {
                if let Some(redirect) = state
                    .suppressed_predicate_redirect
                    .get(&pc.offset.0)
                    .cloned()
                {
                    let from_ids = read_origins(
                        resolved.value(),
                        r.identifier.span().offset.0,
                        r.from.value(),
                        ctx,
                    );
                    let label = edge_label_of_ref(r);
                    for from_id in &from_ids {
                        push_edge(
                            &mut state.emitted_edges,
                            &mut state.edges,
                            from_id,
                            label,
                            &redirect,
                        );
                    }
                }
            }
            continue;
        }

        if r.flags.write {
            if r.flags.call || (r.flags.read && !r.owners.is_empty()) {
                let from_id = state_ref_id(r.id.value(), resolved.value(), ctx);
                let label = edge_label_of_ref(r);
                for owner_id in &r.owners {
                    if owner_id.value() == resolved.value() {
                        continue;
                    }
                    let target_id = retarget_owner_target(
                        owner_target_id(
                            owner_id.value(),
                            r.identifier.span().offset.0,
                            &ctx.write_ops_by_variable,
                        ),
                        owner_id.value(),
                        r.identifier.span().offset.0,
                        &state.result_proxy_by_var,
                        &state.result_proxy_arm_span,
                        &state.result_proxy_by_write_op,
                        &state.result_proxy_write_op_arm_span,
                    );
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        &from_id,
                        label,
                        &target_id,
                    );
                }
            }
            if r.flags.read {
                if let Some(op) = ctx.write_op_by_ref.get(r.id.value()) {
                    let wr_target_id = write_op_node_id(r.id.value());
                    let set_pred_id = set_predecessor_of(
                        op,
                        ctx.write_ops_by_variable
                            .get(resolved.value())
                            .map(Vec::as_slice)
                            .unwrap_or(&[]),
                        &ctx.scope_map,
                    );
                    let from_ids = read_origins(
                        resolved.value(),
                        r.identifier.span().offset.0,
                        r.from.value(),
                        ctx,
                    );
                    for from_id in &from_ids {
                        if from_id == &set_pred_id || from_id == &wr_target_id {
                            continue;
                        }
                        push_edge(
                            &mut state.emitted_edges,
                            &mut state.edges,
                            from_id,
                            "read",
                            &wr_target_id,
                        );
                    }
                }
            }
            continue;
        }

        // Pure read (no write flag).
        let label = edge_label_of_ref(r);
        let from_ids = read_origins(
            resolved.value(),
            r.identifier.span().offset.0,
            r.from.value(),
            ctx,
        );
        if !r.owners.is_empty() {
            for owner_id in &r.owners {
                if owner_id.value() == resolved.value() {
                    continue;
                }
                let target_id = retarget_owner_target(
                    owner_target_id(
                        owner_id.value(),
                        r.identifier.span().offset.0,
                        &ctx.write_ops_by_variable,
                    ),
                    owner_id.value(),
                    r.identifier.span().offset.0,
                    &state.result_proxy_by_var,
                    &state.result_proxy_arm_span,
                    &state.result_proxy_by_write_op,
                    &state.result_proxy_write_op_arm_span,
                );
                for from_id in &from_ids {
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        from_id,
                        label,
                        &target_id,
                    );
                }
            }
        } else {
            // A value read in a *bare ternary statement* (`cond ? a : b;`)
            // is a discarded output of the ternary: no consumer, so it
            // emits no edge — rather than a spurious edge to the module
            // sink. The receiver of an arm's callback call is an input to
            // that call (its arm hosts a CallProxy), so it is kept and
            // routed to the proxy below. A ternary nested in a consumer
            // (`foo(cond ? a : b);`) carries the consumer's container, not
            // a ternary statement's, so its arm values still reach the
            // consumer.
            let off = r.identifier.span().offset.0;
            let is_ternary_stmt = r
                .expression_statement_container
                .as_ref()
                .is_some_and(|c| ternary_stmt_starts.contains(&c.start_span.offset.0));
            let in_callback_arm = state
                .ternary_callback_arm_spans
                .iter()
                .any(|&(s, e)| off >= s && off < e);
            if is_ternary_stmt && !in_callback_arm {
                continue;
            }
            let enclosing_fn_var_id = enclosing_function_var_borrowed(
                r.from.value(),
                &ctx.scope_map,
                &ctx.subgraph_owner_var,
            );
            let host = find_host_subgraph(r, enclosing_fn_var_id, &ctx.scope_map, state);
            // Only the owner-var-less path needs the host scope id as
            // a subgraph key; computing it is an extra scope walk, so
            // it stays gated behind `enclosing_fn_var_id.is_none()`
            // and never burdens the hot owner-var path.
            let enclosing_fn_scope_id = if enclosing_fn_var_id.is_none() {
                find_host_scope_id(r, &ctx.scope_map, state)
            } else {
                None
            };
            let target_container = match host {
                Some(sg) => Container::Subgraph(sg),
                None => Container::Root,
            };
            let expr_stmt_id = ensure_expression_statement_node(
                arena,
                state,
                r,
                &ctx.source_index,
                target_container,
            );
            let target_id = resolve_read_target_id(
                arena,
                state,
                ctx,
                expr_stmt_id.as_deref(),
                enclosing_fn_var_id,
                enclosing_fn_scope_id,
                r,
            );
            for from_id in &from_ids {
                push_edge(
                    &mut state.emitted_edges,
                    &mut state.edges,
                    from_id,
                    label,
                    &target_id,
                );
            }
        }
    }
}

/// Redirect a call's init-time owner edge from the binding node it lands
/// on -- the result variable's own node for `const xs = arr.map(cb)`, or
/// the reassignment write-op node for `y = arr.map(cb)` -- to the bound
/// CallProxy, when one exists. This is what makes the call's inputs read
/// `input → the call` rather than pointing straight at the binding.
fn retarget_owner_target(
    target_id: String,
    owner_var_id: &str,
    ref_offset: u32,
    result_proxy_by_var: &HashMap<String, String>,
    result_proxy_arm_span: &HashMap<String, (u32, u32)>,
    result_proxy_by_write_op: &HashMap<String, String>,
    result_proxy_write_op_arm_span: &HashMap<String, (u32, u32)>,
) -> String {
    if target_id == node_id(owner_var_id) {
        if let Some(proxy) = result_proxy_by_var.get(owner_var_id) {
            // A ternary-arm proxy claims only reads inside the arm that
            // hosts the call (`result_proxy_arm_span`); the sibling arm's
            // value keeps its direct edge to the binding. Ordinary
            // bindings record no span and redirect unconditionally.
            if read_belongs_to_arm(result_proxy_arm_span.get(owner_var_id), ref_offset) {
                return proxy.clone();
            }
        }
    }
    if let Some(proxy) = result_proxy_by_write_op.get(&target_id) {
        // Same arm gating for a reassignment's write-op proxy.
        if read_belongs_to_arm(result_proxy_write_op_arm_span.get(&target_id), ref_offset) {
            return proxy.clone();
        }
    }
    target_id
}

/// Whether a read at `ref_offset` is inside the ternary arm that hosts a
/// gated CallProxy. `None` (no recorded arm — an ordinary, non-ternary
/// binding) means the proxy claims every read unconditionally.
fn read_belongs_to_arm(arm_span: Option<&(u32, u32)>, ref_offset: u32) -> bool {
    match arm_span {
        Some(&(start, end)) => ref_offset >= start && ref_offset < end,
        None => true,
    }
}
