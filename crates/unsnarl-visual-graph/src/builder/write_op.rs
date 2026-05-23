//! Per-Write reference record that the builder threads through
//! the various edge-emitting helpers. `offset` is the identifier's
//! UTF-16 offset; `scope_id` is the scope the reference lives in
//! (`SerializedReference.from`), used by [`state_at`] /
//! [`last_write_op_in_scope_before`] / [`set_predecessor_of`] to
//! decide which write reaches a given read.

#[derive(Clone)]
pub struct WriteOp {
    pub ref_id: String,
    pub var_id: String,
    pub var_name: String,
    pub line: u32,
    pub offset: u32,
    pub scope_id: String,
}

/// Returns the prefix of `ops` whose `offset` is strictly less than
/// `boundary`. The caller is responsible for ensuring `ops` is sorted
/// by `offset` ascending; in practice every `Vec<WriteOp>` in
/// `BuilderContext::write_ops_by_variable` (and every filtered subset
/// derived from it) is, because they're built from a reference list
/// pre-sorted on `identifier.span().offset` in `build_visual_graph`.
#[inline]
pub fn ops_before(ops: &[WriteOp], boundary: u32) -> &[WriteOp] {
    &ops[..ops.partition_point(|op| op.offset < boundary)]
}
