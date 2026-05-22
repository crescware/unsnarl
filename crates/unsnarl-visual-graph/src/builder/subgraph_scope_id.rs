use super::sanitize::sanitize;
use unsnarl_ir::serialized::SerializedScope;

pub fn subgraph_scope_id(scope: &SerializedScope) -> String {
    format!("s_{}", sanitize(scope.id.value()))
}

#[cfg(test)]
#[path = "subgraph_scope_id_test.rs"]
mod subgraph_scope_id_test;
