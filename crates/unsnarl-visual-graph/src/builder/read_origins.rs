//! For a read reference, return the set of nodes from which the
//! `read` edge should be drawn — typically the last preceding
//! `Write` for the same variable, but extended through branch
//! containers (`if` / `case` / `try`) so all reachable last-writes
//! are surfaced, plus the pre-container write when the container
//! has no fallback branch (`if` without `alternate`, `try` without
//! `handler`).

use super::branch_container_key::branch_container_key;
use super::branch_merged_origins::branch_merged_origins;
use super::branch_scope_of::branch_scope_of;
use super::context::BuilderContext;
use super::is_ancestor_scope::is_ancestor_scope;
use super::node_id::node_id;
use super::timing::TimingScope;
use super::write_op::WriteOp;
use super::write_op_node_id::write_op_node_id;

pub fn read_origins(
    var_id: &str,
    ref_offset: u32,
    ref_scope_id: &str,
    ctx: &BuilderContext<'_>,
) -> Vec<String> {
    let _t = TimingScope::start("read_origins");
    let ops_slice: &[WriteOp] = ctx
        .write_ops_by_variable
        .get(var_id)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let prev: Vec<WriteOp> = ops_slice
        .iter()
        .filter(|op| op.offset < ref_offset)
        .cloned()
        .collect();
    let Some(last) = prev.last() else {
        return vec![node_id(var_id)];
    };
    if is_ancestor_scope(&last.scope_id, ref_scope_id, &ctx.scope_map) {
        return vec![write_op_node_id(&last.ref_id)];
    }
    let Some(last_branch_id) = branch_scope_of(&last.scope_id, &ctx.scope_map) else {
        return vec![write_op_node_id(&last.ref_id)];
    };
    let Some(last_branch_scope) = ctx.scope_map.get(last_branch_id.as_str()).copied() else {
        return vec![write_op_node_id(&last.ref_id)];
    };
    let Some(container_key) = branch_container_key(last_branch_scope) else {
        return vec![write_op_node_id(&last.ref_id)];
    };

    let branch_scope_ids: Vec<&str> = ctx
        .branch_scopes_by_container
        .get(&container_key)
        .map(|scopes| scopes.iter().map(|s| s.id.value()).collect())
        .unwrap_or_default();

    let is_switch = container_key.starts_with("switch:");
    let sorted_cases = if is_switch {
        ctx.sorted_cases_by_container.get(&container_key)
    } else {
        None
    };

    let mut origins: Vec<String> = Vec::new();
    for branch_id in &branch_scope_ids {
        let branch_scope = ctx.scope_map.get(*branch_id).copied();
        if let Some(scope) = branch_scope {
            if is_switch && scope.falls_through {
                if let Some(cases) = sorted_cases {
                    let idx = cases.iter().position(|c| c.id.value() == *branch_id);
                    if let Some(i) = idx {
                        if i < cases.len() - 1 {
                            continue;
                        }
                    }
                }
            }
            if is_switch && scope.exits_function {
                continue;
            }
        }
        let sub = branch_merged_origins(branch_id, &prev, ctx);
        origins.extend(sub);
    }

    if container_key.starts_with("if:") {
        let has_alternate = branch_scope_ids.iter().any(|id| {
            ctx.scope_map
                .get(*id)
                .copied()
                .and_then(|s| s.block_context.as_ref())
                .map(|c| c.key())
                == Some("alternate")
        });
        if !has_alternate {
            let if_offset = last_branch_scope
                .block_context
                .as_ref()
                .map(|c| c.parent_span_offset().0)
                .unwrap_or(0);
            let before: Vec<&WriteOp> = ops_slice
                .iter()
                .filter(|op| op.offset < if_offset)
                .collect();
            if let Some(last_before) = before.last() {
                origins.push(write_op_node_id(&last_before.ref_id));
            } else {
                origins.push(node_id(var_id));
            }
        }
    }

    if container_key.starts_with("try:") {
        let has_handler = branch_scope_ids.iter().any(|id| {
            ctx.scope_map
                .get(*id)
                .copied()
                .and_then(|s| s.block_context.as_ref())
                .map(|c| c.key())
                == Some("handler")
        });
        if !has_handler {
            let try_offset = last_branch_scope
                .block_context
                .as_ref()
                .map(|c| c.parent_span_offset().0)
                .unwrap_or(0);
            let before: Vec<&WriteOp> = ops_slice
                .iter()
                .filter(|op| op.offset < try_offset)
                .collect();
            if let Some(last_before) = before.last() {
                origins.push(write_op_node_id(&last_before.ref_id));
            } else {
                origins.push(node_id(var_id));
            }
        }
    }

    if origins.is_empty() {
        return vec![write_op_node_id(&last.ref_id)];
    }
    dedup_preserve_order(origins)
}

/// Preserves first-occurrence order while removing duplicates.
fn dedup_preserve_order(input: Vec<String>) -> Vec<String> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(input.len());
    for s in input {
        if seen.insert(s.clone()) {
            out.push(s);
        }
    }
    out
}

#[cfg(test)]
#[path = "read_origins_test.rs"]
mod read_origins_test;
