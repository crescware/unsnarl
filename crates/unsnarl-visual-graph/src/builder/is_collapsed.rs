//! Returns `true` when the supplied depth ceiling is below the
//! scope's rendered nesting count for the same kind, which is the
//! signal the depth-control pass uses to drop the scope's body from
//! the rendered graph. `None` for `ceiling` means "no ceiling" and
//! the function returns `false` for every scope.
//!
//! The `rendered` input is the per-scope depth produced by
//! `compute_rendered_nesting_depths` — depth derived from the
//! subgraph hierarchy the visual layer will actually emit, not from
//! AST node types. That is what lets synthesised subgraphs (e.g. the
//! ternary arms inserted by `synthesise_conditional_arms`) participate
//! in depth collapsing on the same footing as AST-anchored subgraphs.

use unsnarl_ir::nesting_kind::{NestingDepths, NestingKind};
use unsnarl_ir::serialized::SerializedScope;

use super::nesting_kind_of::nesting_kind_of;

pub fn is_collapsed(
    scope: &SerializedScope,
    rendered: &NestingDepths,
    ceiling: Option<&NestingDepths>,
) -> bool {
    let Some(ceiling) = ceiling else {
        return false;
    };
    let Some(kind) = nesting_kind_of(scope) else {
        return false;
    };
    nesting_depth_for(rendered, kind).0 > nesting_depth_for(ceiling, kind).0
}

fn nesting_depth_for(
    d: &NestingDepths,
    kind: NestingKind,
) -> unsnarl_ir::nesting_kind::NestingDepth {
    match kind {
        NestingKind::Function => d.function,
        NestingKind::If => d.r#if,
        NestingKind::For => d.r#for,
        NestingKind::While => d.r#while,
        NestingKind::Switch => d.switch,
        NestingKind::TryCatchFinally => d.try_catch_finally,
        NestingKind::Block => d.block,
    }
}

#[cfg(test)]
#[path = "is_collapsed_test.rs"]
mod is_collapsed_test;
