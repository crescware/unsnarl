//! Emit the `set` / `fallthrough` let-chain edges that thread a
//! variable's successive write ops together in scope order.

use super::branch_container_key::branch_container_key;
use super::context::BuilderContext;
use super::is_ancestor_scope::is_ancestor_scope;
use super::last_write_op_in_scope_before::last_write_op_in_scope_before;
use super::node_id::node_id;
use super::previous_fallthrough_case::previous_fallthrough_case;
use super::push_edge::push_edge;
use super::state::BuildState;
use super::write_op_node_id::write_op_node_id;

pub fn emit_let_chain_edges(state: &mut BuildState, ctx: &BuilderContext<'_>) {
    // `write_ops_by_variable` is seeded in `ir.variables` source
    // order. `HashMap` iteration order is not stable; walking
    // `ir.variables` here keeps the rendered edge order stable
    // against the IR parity baselines.
    for v in &ctx.ir.variables {
        let Some(ops) = ctx.write_ops_by_variable.get(v.id.value()) else {
            continue;
        };
        if ops.is_empty() {
            continue;
        }
        for i in 0..ops.len() {
            let op = &ops[i];
            let mut prev_id = node_id(&op.var_id);
            let op_scope = ctx.scope_map.get(op.scope_id.as_str()).copied();
            let op_branch_key = op_scope.and_then(branch_container_key);
            let is_first_in_case = op_scope.is_some()
                && op_branch_key
                    .as_deref()
                    .is_some_and(|k| k.starts_with("switch:"))
                && !ops[..i]
                    .iter()
                    .any(|prev_op| prev_op.scope_id == op.scope_id);
            if is_first_in_case {
                if let Some(scope) = op_scope {
                    if let Some(prev_case) =
                        previous_fallthrough_case(scope, &ctx.sorted_cases_by_container)
                    {
                        if let Some(prev_case_last) = last_write_op_in_scope_before(
                            &op.var_id,
                            prev_case.id.value(),
                            op.offset,
                            &ctx.write_ops_by_variable,
                            &ctx.scope_map,
                        ) {
                            prev_id = write_op_node_id(&prev_case_last.ref_id);
                        }
                    }
                }
            } else {
                for j in (0..i).rev() {
                    let candidate = &ops[j];
                    if is_ancestor_scope(&candidate.scope_id, &op.scope_id, &ctx.scope_map) {
                        prev_id = write_op_node_id(&candidate.ref_id);
                        break;
                    }
                }
            }
            let edge_kind = if is_first_in_case
                && op_scope
                    .and_then(|s| previous_fallthrough_case(s, &ctx.sorted_cases_by_container))
                    .is_some()
            {
                "fallthrough"
            } else {
                "set"
            };
            push_edge(
                &mut state.emitted_edges,
                &mut state.edges,
                &prev_id,
                edge_kind,
                &write_op_node_id(&op.ref_id),
            );
        }
    }
}
