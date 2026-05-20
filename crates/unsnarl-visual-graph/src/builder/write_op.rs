//! Mirrors `ts/src/visual-graph/builder/write-op.ts`.
//!
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
