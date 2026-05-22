//! Computes a string key that groups every branch of a single
//! `if` / `switch` / `try` statement under a stable hashable
//! identifier. The key blends the parent scope id with the
//! statement's `parentSpanOffset` (or, for `else if` chains, the
//! `ifChainRootOffset`) so branches that should merge into one
//! container share the same key.

use unsnarl_ir::scope::block_context::BlockContext;
use unsnarl_ir::scope::block_context_kind::BlockContextKind;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

pub fn branch_container_key(scope: &SerializedScope) -> Option<String> {
    let ctx = scope.block_context.as_ref()?;
    let upper = scope.upper.as_ref().map(|u| u.value()).unwrap_or("");

    match ctx {
        BlockContext::CaseClause(c) => {
            if matches!(c.parent_type(), AstType::SwitchStatement) && c.key() == "cases" {
                return Some(format!("switch:{upper}:{}", c.parent_span_offset().0));
            }
            None
        }
        BlockContext::Other(c) => {
            if matches!(c.parent_type(), AstType::IfStatement)
                && (c.key() == "consequent" || c.key() == "alternate")
            {
                let root = if matches!(c.kind(), BlockContextKind::Other) {
                    c.if_chain_root_offset()
                        .map(|o| o.0)
                        .unwrap_or_else(|| c.parent_span_offset().0)
                } else {
                    c.parent_span_offset().0
                };
                return Some(format!("if:{upper}:{root}"));
            }
            if matches!(c.parent_type(), AstType::TryStatement)
                && (c.key() == "block" || c.key() == "handler")
            {
                return Some(format!("try:{upper}:{}", c.parent_span_offset().0));
            }
            None
        }
    }
}

#[cfg(test)]
#[path = "branch_container_key_test.rs"]
mod branch_container_key_test;
