//! Mirrors `ts/src/visual-graph/builder/owner-target-id.ts`.

use std::collections::HashMap;

use super::node_id::node_id;
use super::write_op::WriteOp;
use super::write_op_node_id::write_op_node_id;

pub fn owner_target_id(
    owner_var_id: &str,
    offset: u32,
    write_ops_by_variable: &HashMap<String, Vec<WriteOp>>,
) -> String {
    let ops = write_ops_by_variable
        .get(owner_var_id)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let mut last: Option<&WriteOp> = None;
    for op in ops {
        if op.offset > offset {
            break;
        }
        last = Some(op);
    }
    last.map(|o| write_op_node_id(&o.ref_id))
        .unwrap_or_else(|| node_id(owner_var_id))
}
