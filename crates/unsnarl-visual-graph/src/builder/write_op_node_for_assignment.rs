//! The write-op node id of a reassignment host's plain-identifier
//! target.

use unsnarl_ir::serialized::SerializedCallbackHost;

use super::context::BuilderContext;
use super::write_op_node_id::write_op_node_id;

/// The write-op node id of a reassignment host's target, when the
/// left-hand side is a plain identifier. The host carries the target
/// identifier's offset (`target_span`); the matching Write `WriteOp`
/// shares that offset (both are the write reference's identifier
/// offset), so its `ref_id` yields the `wr_<ref_id>` node. `None` for
/// member / destructuring targets (no `target_span`) or if no write op
/// is found at that offset.
pub fn write_op_node_for_assignment(
    host: &SerializedCallbackHost,
    ctx: &BuilderContext<'_>,
) -> Option<String> {
    let target_offset = host.target_span.as_ref()?.offset.0;
    ctx.write_op_by_ref
        .values()
        .find(|op| op.offset == target_offset)
        .map(|op| write_op_node_id(&op.ref_id))
}
