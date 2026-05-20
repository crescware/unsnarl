//! Mirrors `ts/src/visual-graph/builder/is-control-subgraph.ts`.

use unsnarl_ir::serialized::SerializedScope;

use super::control_subgraph_kind_of::control_subgraph_kind_of;

pub fn is_control_subgraph(scope: &SerializedScope) -> bool {
    control_subgraph_kind_of(scope).is_some()
}
