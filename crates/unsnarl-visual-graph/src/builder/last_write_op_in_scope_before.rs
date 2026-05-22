use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::is_ancestor_scope::is_ancestor_scope;
use super::write_op::WriteOp;

pub fn last_write_op_in_scope_before<'a>(
    var_id: &str,
    scope_id: &str,
    offset: u32,
    write_ops_by_variable: &'a HashMap<String, Vec<WriteOp>>,
    scope_map: &HashMap<&str, &SerializedScope>,
) -> Option<&'a WriteOp> {
    let ops = write_ops_by_variable
        .get(var_id)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let mut last: Option<&'a WriteOp> = None;
    for op in ops {
        if op.offset >= offset {
            break;
        }
        if op.scope_id == scope_id || is_ancestor_scope(scope_id, &op.scope_id, scope_map) {
            last = Some(op);
        }
    }
    last
}

#[cfg(test)]
#[path = "last_write_op_in_scope_before_test.rs"]
mod last_write_op_in_scope_before_test;
