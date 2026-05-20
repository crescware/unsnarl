//! Predicate: which [`NodeKind`](crate::node_kind::NodeKind) values are
//! eligible as `-r` root candidates.
//!
//! Mirrors `ts/src/visual-graph/prune/root-candidate-kinds.ts`. The
//! TS version exposes this as a `ReadonlySet<NodeKind>` literal; the
//! Rust port exposes the predicate as a free function so callers do
//! not pay for a static `HashSet` allocation per process and the
//! match doubles as an exhaustiveness check.

use crate::node_kind::NodeKind;

pub fn is_root_candidate_kind(kind: NodeKind) -> bool {
    matches!(
        kind,
        NodeKind::VarBinding
            | NodeKind::ConstBinding
            | NodeKind::LetBinding
            | NodeKind::FunctionDeclaration
            | NodeKind::ClassDeclaration
            | NodeKind::FormalParameter
            | NodeKind::CatchParameter
            | NodeKind::NamedImportBinding
            | NodeKind::DefaultImportBinding
            | NodeKind::NamespaceImportBinding
            | NodeKind::SyntheticImplicitGlobal
            | NodeKind::ReturnArgumentReference
            | NodeKind::ThrowArgumentReference
            | NodeKind::WriteReference
            | NodeKind::SyntheticExpressionStatement
    )
}
