//! Mirrors `ts/src/visual-graph/builder/state-ref-id.ts`.
//!
//! Resolve the *state* of `varId` at the moment ref `refId` reads
//! it: a Write op's node id when one exists for the ref itself,
//! the most recent preceding write otherwise, or the variable
//! declaration node when no write has happened yet.

use super::context::BuilderContext;
use super::node_id::node_id;
use super::state_at::state_at;
use super::write_op_node_id::write_op_node_id;

pub fn state_ref_id(ref_id: &str, var_id: &str, ctx: &BuilderContext<'_>) -> String {
    if let Some(op) = ctx.write_op_by_ref.get(ref_id) {
        return write_op_node_id(&op.ref_id);
    }
    let Some(r) = ctx.ir.references.iter().find(|r| r.id.value() == ref_id) else {
        return node_id(var_id);
    };
    let state_ref = state_at(
        var_id,
        r.identifier.span().offset.0,
        &ctx.write_ops_by_variable,
    );
    if state_ref == var_id {
        node_id(var_id)
    } else {
        write_op_node_id(&state_ref)
    }
}
