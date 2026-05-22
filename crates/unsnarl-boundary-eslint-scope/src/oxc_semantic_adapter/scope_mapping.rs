//! `oxc_semantic::Scoping` scope tree → `IrArena.scopes`.
//!
//! Reads `Scoping::get_node_id(scope_id)` to recover each scope's
//! anchor AST node, then materialises an [`unsnarl_ir::scope::ScopeData`]
//! via [`crate::materialise::ast_node_of`]. [`ScopeType`] is derived
//! from the anchor `AstKind` for cases where eslint-scope's scope
//! categorisation diverges from `ScopeFlags` (e.g. `BlockStatement`,
//! `SwitchStatement`, `Class`); the bare flags are used only as a
//! tiebreaker (Function vs Arrow are both `ScopeType::Function`, so
//! the arrow distinction is dropped here).
//!
//! `is_strict` cannot read off [`ScopeFlags::StrictMode`] directly:
//! [`crate::parser::OxcParser`] always parses with
//! `oxc_span::SourceType::with_module(true)` so module-only syntax
//! (top-level `await`, `import` / `export`) keeps parsing under
//! [`crate::parser::SourceType::Script`], and as a consequence the
//! root scope's [`ScopeFlags::StrictMode`] is set for every input
//! regardless of the boundary's analysis-level source type. The
//! hand-rolled scope-builder sidesteps the same problem by computing
//! `is_strict` purely from the analysis-level source type at the
//! root (Module ⇒ true, Script ⇒ false) and propagating it down via
//! parent inheritance, ignoring inline `"use strict"` directives and
//! class-body auto-strictness. Mirror that behaviour here so the
//! adapter's serialized output is parity-shaped.
//!
//! `variable_scope` is computed inline. `Scoping::scope_descendants_from_root`
//! iterates scopes in DFS order so each parent's row is already in
//! place when a child is pushed, letting the child either point at
//! itself (if its flags make it a `var`-creating scope) or copy the
//! parent's `variable_scope`. `child_scopes` is then filled in a
//! second pass once every scope's `upper` is set.
//!
//! `variables` / `references` / `through` stay empty here; the
//! respective entity passes (`variable_mapping`, `reference_mapping`)
//! fill them. `function_expression_scope` stays `false`: eslint-scope
//! synthesises a `FunctionExpressionName` wrapper around named
//! function expressions, but `oxc_semantic` does not create such a
//! scope. Synthesising that wrapper is a follow-up commit; for now
//! the mapping is a direct 1:1 walk of `oxc_semantic`'s scope tree.
//!
//! ## Known divergences (deferred to follow-up commits)
//!
//! 1. **Catch clause**: `oxc_semantic` emits two scopes
//!    (`ScopeFlags::CatchClause` for the parameter scope, and an
//!    empty-flagged `BlockStatement` scope for the body). Eslint-scope
//!    emits one (`Catch`) covering both. This mapping currently
//!    surfaces both; merging is a follow-up.
//! 2. **`FunctionExpressionName`**: not synthesised here (see above).
//! 3. **`SwitchCase`**: eslint-scope creates a per-`SwitchCase` Block
//!    scope; `oxc_semantic` does not. Synthesis is a follow-up.
//! 4. **`ClassFieldInitializer`**: eslint-scope creates a per-field
//!    initializer scope; `oxc_semantic` does not. Synthesis is a
//!    follow-up.
//! 5. **TypeScript-only scopes**: `oxc_semantic` adds scopes for
//!    `TSModuleDeclaration` / `TSConditionalType` / `TSMappedType`
//!    that the boundary's hand-rolled walker filters out via
//!    `is_type_only_subtree`. Filtering is a follow-up.
//!
//! Each item is gated on a parity-harness signal (Phase 2 step 5);
//! the comment is kept in code so reviewers see the exact scope of
//! this commit's coverage rather than discovering it from test output.

use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_syntax::scope::{ScopeFlags, ScopeId as OxcScopeId};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::scope::ScopeData;
use unsnarl_ir::scope_type::ScopeType;

use crate::materialise::ast_node_of;
use crate::parser::SourceType;

/// Walk `semantic.scoping()`'s scope tree and produce the
/// `unsnarl_ir` arena rows. ID translation between `oxc_semantic`'s
/// `ScopeId` and `unsnarl_ir::ids::ScopeId` is a 1:1 `usize` cast:
/// both are `NonMaxU32`-backed and `scope_descendants_from_root`
/// preserves DFS order starting at zero.
pub(crate) fn build_scopes<'a>(
    semantic: &Semantic<'a>,
    source_type: SourceType,
) -> IndexVec<ScopeId, ScopeData> {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    let total = scoping.scopes_len();
    let mut scopes: IndexVec<ScopeId, ScopeData> = IndexVec::with_capacity(total);

    let root_is_strict = matches!(source_type, SourceType::Module);
    for oxc_id in scoping.scope_descendants_from_root() {
        let node_id = scoping.get_node_id(oxc_id);
        let kind = nodes.kind(node_id);
        let block = ast_node_of(&kind);
        let flags = scoping.scope_flags(oxc_id);
        let ty = derive_scope_type(flags, &kind, source_type);
        let upper = scoping.scope_parent_id(oxc_id).map(ir_scope_id);
        let is_strict = match upper {
            Some(upper_id) => scopes[upper_id].is_strict,
            None => root_is_strict,
        };
        let new_id = ScopeId::from_usize(scopes.len());
        let variable_scope = if is_var_creating(flags) {
            new_id
        } else if let Some(upper_id) = upper {
            scopes[upper_id].variable_scope
        } else {
            new_id
        };
        scopes.push(ScopeData::new(
            ty,
            is_strict,
            upper,
            Vec::new(),
            variable_scope,
            block,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            false,
        ));
    }

    for raw_index in 0..scopes.len() {
        let ir_id = ScopeId::from_usize(raw_index);
        if let Some(upper) = scopes[ir_id].upper {
            scopes[upper].child_scopes.push(ir_id);
        }
    }

    scopes
}

/// Derive the eslint-scope `ScopeType` for a scope from its anchor
/// `AstKind` and `ScopeFlags`. The anchor is the source of truth for
/// most cases — `oxc_semantic` uses empty flags for `BlockStatement` /
/// `ForStatement` / `SwitchStatement` / `Class` etc., and the anchor
/// disambiguates them.
pub(crate) fn derive_scope_type(
    flags: ScopeFlags,
    anchor: &AstKind<'_>,
    source_type: SourceType,
) -> ScopeType {
    if flags.is_top() {
        return match source_type {
            SourceType::Module => ScopeType::Module,
            SourceType::Script => ScopeType::Global,
        };
    }
    match anchor {
        AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => ScopeType::Function,
        AstKind::Class(_) => ScopeType::Class,
        AstKind::CatchClause(_) => ScopeType::Catch,
        AstKind::WithStatement(_) => ScopeType::With,
        AstKind::ForStatement(_) | AstKind::ForInStatement(_) | AstKind::ForOfStatement(_) => {
            ScopeType::For
        }
        AstKind::SwitchStatement(_) => ScopeType::Switch,
        AstKind::StaticBlock(_) => ScopeType::ClassStaticBlock,
        // BlockStatement, SwitchCase consequent, TS-only blocks, …
        _ => ScopeType::Block,
    }
}

fn is_var_creating(flags: ScopeFlags) -> bool {
    flags.is_var()
}

fn ir_scope_id(oxc: OxcScopeId) -> ScopeId {
    ScopeId::from_usize(oxc.index())
}

#[cfg(test)]
#[path = "scope_mapping_test.rs"]
mod scope_mapping_test;
