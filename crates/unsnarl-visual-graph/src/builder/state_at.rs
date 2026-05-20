//! Mirrors `ts/src/visual-graph/builder/state-at.ts`.
//!
//! Find the latest write reference id whose offset precedes the
//! supplied `offset`. Returns the variable id itself when no
//! preceding write exists.

use std::collections::HashMap;

use super::write_op::WriteOp;

pub fn state_at(
    var_id: &str,
    offset: u32,
    write_ops_by_variable: &HashMap<String, Vec<WriteOp>>,
) -> String {
    let ops = write_ops_by_variable
        .get(var_id)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let mut last: Option<&WriteOp> = None;
    for op in ops {
        if op.offset >= offset {
            break;
        }
        last = Some(op);
    }
    last.map(|o| o.ref_id.clone())
        .unwrap_or_else(|| var_id.to_string())
}
