//! Mirrors `ts/src/visual-graph/builder/set-predecessor-of.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::is_ancestor_scope::is_ancestor_scope;
use super::node_id::node_id;
use super::write_op::WriteOp;
use super::write_op_node_id::write_op_node_id;

pub fn set_predecessor_of(
    op: &WriteOp,
    ops: &[WriteOp],
    scope_map: &HashMap<&str, &SerializedScope>,
) -> String {
    let Some(i) = ops.iter().position(|c| c.ref_id == op.ref_id) else {
        return node_id(&op.var_id);
    };
    for j in (0..i).rev() {
        let candidate = &ops[j];
        if is_ancestor_scope(&candidate.scope_id, &op.scope_id, scope_map) {
            return write_op_node_id(&candidate.ref_id);
        }
    }
    node_id(&op.var_id)
}

#[cfg(test)]
#[path = "set_predecessor_of_test.rs"]
mod set_predecessor_of_test;
