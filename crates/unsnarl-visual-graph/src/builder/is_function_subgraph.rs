//! Mirrors `ts/src/visual-graph/builder/is-function-subgraph.ts`.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;

pub fn is_function_subgraph(scope: &SerializedScope) -> bool {
    matches!(scope.r#type, ScopeType::Function) && !scope.function_expression_scope
}
