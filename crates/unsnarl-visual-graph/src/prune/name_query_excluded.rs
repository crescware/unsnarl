//! Predicate: which [`NodeKind`](crate::node_kind::NodeKind) values are
//! excluded from bare-name `-r` queries.
//!
//! Mirrors `ts/src/visual-graph/prune/name-query-excluded.ts`. The TS
//! version exposes this as a `ReadonlySet<NodeKind>` literal; the
//! Rust port exposes the predicate as a free function (see the
//! `root_candidate_kinds` note for the rationale).

use crate::node_kind::NodeKind;

pub fn is_name_query_excluded(kind: NodeKind) -> bool {
    matches!(
        kind,
        NodeKind::WriteReference
            | NodeKind::ReturnArgumentReference
            | NodeKind::ThrowArgumentReference
    )
}
