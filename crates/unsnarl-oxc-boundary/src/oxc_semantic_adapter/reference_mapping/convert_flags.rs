//! Reference-flag conversion helpers shared by the reference-mapping
//! passes: translate `oxc_semantic`'s `ReferenceFlags` into the IR's
//! `ReferenceFlagBits`, and patch the slots where oxc's flag disagrees
//! with the parity baseline.

use oxc_ast::AstKind;
use oxc_syntax::reference::ReferenceFlags as OxcReferenceFlags;

use unsnarl_ir::reference::reference_flags::{ReferenceFlagBits, ReferenceFlags};

pub(super) fn convert_flags(flags: OxcReferenceFlags) -> ReferenceFlagBits {
    let mut out = ReferenceFlags::NONE;
    if flags.is_read() {
        out |= ReferenceFlags::READ;
    }
    if flags.is_write() {
        out |= ReferenceFlags::WRITE;
    }
    out
}

/// Adjust reference flags for slots where `oxc_semantic`'s flag does
/// not match the parity baseline.
///
/// Currently handled: `ForInStatement.left` and `ForOfStatement.left`.
/// `oxc_semantic` marks the loop variable as `Write` (each iteration
/// assigns to it), but the parity baseline records these as plain
/// `READ`. Force the flags to `READ` when the reference's parent is
/// one of those `for` shapes.
pub(super) fn adjust_flags_for_parent(
    flags: ReferenceFlagBits,
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_semantic::NodeId,
) -> ReferenceFlagBits {
    let parent_kind = nodes.parent_kind(node_id);
    match parent_kind {
        AstKind::ForOfStatement(_) | AstKind::ForInStatement(_) => ReferenceFlags::READ,
        _ => flags,
    }
}
