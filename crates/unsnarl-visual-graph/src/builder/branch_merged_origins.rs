//! Given an outer branch (an if/case/try arm), collect every
//! reachable last-write op by recursing into branch containers
//! nested inside it. Without this recursion, a `case 0` whose body
//! is a fully-covering inner if/else would flatten its branches
//! into a single textually-last write and drop the inner siblings.

use super::branch_container_key::branch_container_key;
use super::context::BuilderContext;
use super::is_ancestor_scope::is_ancestor_scope;
use super::outermost_branch_under::outermost_branch_under;
use super::write_op::{ops_before, WriteOp};
use super::write_op_node_id::write_op_node_id;

pub fn branch_merged_origins(
    branch_id: &str,
    prev: &[WriteOp],
    ctx: &BuilderContext<'_>,
) -> Vec<String> {
    let inside_ops: Vec<&WriteOp> = prev
        .iter()
        .filter(|op| {
            op.scope_id == branch_id || is_ancestor_scope(branch_id, &op.scope_id, &ctx.scope_map)
        })
        .collect();
    if inside_ops.is_empty() {
        return Vec::new();
    }
    let last = inside_ops[inside_ops.len() - 1];
    let Some(inner_branch_id) = outermost_branch_under(branch_id, &last.scope_id, &ctx.scope_map)
    else {
        return vec![write_op_node_id(&last.ref_id)];
    };
    let Some(inner_scope) = ctx.scope_map.get(inner_branch_id.as_str()).copied() else {
        return vec![write_op_node_id(&last.ref_id)];
    };
    let Some(inner_key) = branch_container_key(inner_scope) else {
        return vec![write_op_node_id(&last.ref_id)];
    };

    // `branch_scopes_by_container` is a pre-built index keyed by the
    // container key (e.g. `if:<offset>`, `switch:<offset>`,
    // `try:<offset>`), so the scopes that share `inner_key` can be
    // looked up directly. The previous shape walked every scope in
    // the IR (~36k on `mermaid.js`) once per call, which dominated
    // wall time inside `read_origins`.
    let inner_siblings: Vec<_> = ctx
        .branch_scopes_by_container
        .get(&inner_key)
        .map(|scopes| {
            scopes
                .iter()
                .copied()
                .filter(|s| is_ancestor_scope(branch_id, s.id.value(), &ctx.scope_map))
                .collect()
        })
        .unwrap_or_default();

    let is_switch = inner_key.starts_with("switch:");
    let sorted_cases = if is_switch {
        ctx.sorted_cases_by_container.get(&inner_key)
    } else {
        None
    };

    // The recursive call wants `prev: &[WriteOp]` but `inside_ops`
    // is a `Vec<&WriteOp>`. Clone the filtered ops into a flat
    // owned vec so the inner call can take a slice over them.
    let inside_owned: Vec<WriteOp> = inside_ops.iter().map(|op| (*op).clone()).collect();

    let mut merged: Vec<String> = Vec::new();
    for sib in inner_siblings.iter() {
        if is_switch {
            if let Some(cases) = sorted_cases {
                if sib.falls_through {
                    let idx = cases.iter().position(|c| c.id.value() == sib.id.value());
                    if let Some(i) = idx {
                        if i < cases.len() - 1 {
                            continue;
                        }
                    }
                }
            }
            if sib.exits_function {
                continue;
            }
        }
        let sub = branch_merged_origins(sib.id.value(), &inside_owned, ctx);
        merged.extend(sub);
    }

    // When the inner container can be skipped (an if without
    // alternate, or a try without catch), the last write that ran
    // *before* the inner container remains a possible last writer
    // for the outer branch and must be kept.
    let lacks_fallback = (inner_key.starts_with("if:")
        && !inner_siblings
            .iter()
            .any(|v| v.block_context.as_ref().map(|c| c.key()) == Some("alternate")))
        || (inner_key.starts_with("try:")
            && !inner_siblings
                .iter()
                .any(|v| v.block_context.as_ref().map(|c| c.key()) == Some("handler")));
    if lacks_fallback {
        let inner_offset = inner_scope
            .block_context
            .as_ref()
            .map(|c| c.parent_span_offset().0)
            .unwrap_or(0);
        if let Some(last_before) = ops_before(&inside_owned, inner_offset).last() {
            merged.push(write_op_node_id(&last_before.ref_id));
        }
    }

    if merged.is_empty() {
        return vec![write_op_node_id(&last.ref_id)];
    }
    merged
}

#[cfg(test)]
#[path = "branch_merged_origins_test.rs"]
mod branch_merged_origins_test;
