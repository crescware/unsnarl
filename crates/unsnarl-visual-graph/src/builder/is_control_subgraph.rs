use unsnarl_ir::serialized::SerializedScope;

use super::control_subgraph_kind_of::control_subgraph_kind_of;

pub fn is_control_subgraph(scope: &SerializedScope) -> bool {
    control_subgraph_kind_of(scope).is_some()
}

#[cfg(test)]
#[path = "is_control_subgraph_test.rs"]
mod is_control_subgraph_test;
