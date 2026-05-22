//! Predicate: which [`NodeKind`](crate::node_kind::NodeKind) values are
//! excluded from bare-name `-r` queries.
//!
//! Exposed as a free function (rather than a static set) — see the
//! `root_candidate_kinds` note for the rationale.

use crate::node_kind::NodeKind;

pub fn is_name_query_excluded(kind: NodeKind) -> bool {
    matches!(
        kind,
        NodeKind::WriteReference
            | NodeKind::ReturnArgumentReference
            | NodeKind::ThrowArgumentReference
    )
}
