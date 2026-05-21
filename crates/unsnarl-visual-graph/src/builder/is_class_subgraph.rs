//! Mirrors `ts/src/visual-graph/builder/is-class-subgraph.ts`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;

pub fn is_class_subgraph(scope: &SerializedScope) -> bool {
    matches!(scope.r#type, ScopeType::Class)
}

#[cfg(test)]
#[path = "is_class_subgraph_test.rs"]
mod is_class_subgraph_test;
