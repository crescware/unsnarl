//! Categorises a scope by the `NestingKind` it counts against —
//! the bucket the depth control machinery uses to decide whether
//! the scope collapses past the depth threshold.

use unsnarl_ir::nesting_kind::NestingKind;
use unsnarl_ir::scope::block_context::BlockContext;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

pub fn nesting_kind_of(scope: &SerializedScope) -> Option<NestingKind> {
    match scope.r#type {
        ScopeType::Function => {
            if scope.function_expression_scope {
                None
            } else {
                Some(NestingKind::Function)
            }
        }
        ScopeType::For => Some(NestingKind::For),
        ScopeType::Switch => Some(NestingKind::Switch),
        ScopeType::Catch => Some(NestingKind::TryCatchFinally),
        ScopeType::Block => {
            let Some(ctx) = scope.block_context.as_ref() else {
                return Some(NestingKind::Block);
            };
            Some(nesting_kind_from_block_context(ctx))
        }
        _ => None,
    }
}

fn nesting_kind_from_block_context(ctx: &BlockContext) -> NestingKind {
    match ctx.parent_type() {
        AstType::IfStatement => NestingKind::If,
        AstType::ForStatement | AstType::ForInStatement | AstType::ForOfStatement => {
            NestingKind::For
        }
        AstType::WhileStatement | AstType::DoWhileStatement => NestingKind::While,
        AstType::TryStatement | AstType::CatchClause => NestingKind::TryCatchFinally,
        AstType::SwitchStatement => NestingKind::Switch,
        _ => NestingKind::Block,
    }
}

#[cfg(test)]
#[path = "nesting_kind_of_test.rs"]
mod nesting_kind_of_test;
