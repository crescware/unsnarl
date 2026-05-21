//! Mirrors `ts/src/visual-graph/builder/control-subgraph-kind-of.ts`.

use unsnarl_ir::scope::block_context::BlockContext;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use crate::visual_subgraph::ControlSubgraphKind;

pub fn control_subgraph_kind_of(scope: &SerializedScope) -> Option<ControlSubgraphKind> {
    match scope.r#type {
        ScopeType::Catch => Some(ControlSubgraphKind::Catch),
        ScopeType::For => Some(ControlSubgraphKind::For),
        ScopeType::Switch => Some(ControlSubgraphKind::Switch),
        ScopeType::Block => kind_from_block_context(scope.block_context.as_ref()),
        _ => None,
    }
}

fn kind_from_block_context(ctx: Option<&BlockContext>) -> Option<ControlSubgraphKind> {
    let Some(ctx) = ctx else {
        // A Block scope without a recognised parent context still
        // renders as a generic block (mirrors the TS fallthrough).
        return Some(ControlSubgraphKind::Block);
    };
    let key = ctx.key();
    match ctx.parent_type() {
        AstType::TryStatement => match key {
            "block" => Some(ControlSubgraphKind::Try),
            "finalizer" => Some(ControlSubgraphKind::Finally),
            _ => Some(ControlSubgraphKind::Block),
        },
        AstType::IfStatement => match key {
            "consequent" => Some(ControlSubgraphKind::If),
            "alternate" => Some(ControlSubgraphKind::Else),
            _ => Some(ControlSubgraphKind::Block),
        },
        AstType::SwitchStatement if key == "cases" => Some(ControlSubgraphKind::Case),
        AstType::WhileStatement if key == "body" => Some(ControlSubgraphKind::While),
        AstType::DoWhileStatement if key == "body" => Some(ControlSubgraphKind::DoWhile),
        _ => Some(ControlSubgraphKind::Block),
    }
}

#[cfg(test)]
#[path = "control_subgraph_kind_of_test.rs"]
mod control_subgraph_kind_of_test;
