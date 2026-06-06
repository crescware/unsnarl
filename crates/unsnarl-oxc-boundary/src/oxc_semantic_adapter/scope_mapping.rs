//! `oxc_semantic::Scoping` scope tree → `IrArena.scopes`.
//!
//! Reads `Scoping::get_node_id(scope_id)` to recover each scope's
//! anchor AST node, then materialises an [`unsnarl_ir::scope::ScopeData`]
//! via [`crate::materialise::ast_node_of`]. [`ScopeType`] is derived
//! from the anchor `AstKind` for cases where the parity baseline's
//! scope categorisation diverges from `ScopeFlags` (e.g.
//! `BlockStatement`, `SwitchStatement`, `Class`); the bare flags are
//! used only as a tiebreaker (Function vs Arrow are both
//! `ScopeType::Function`, so the arrow distinction is dropped here).
//!
//! `is_strict` cannot read off [`ScopeFlags::StrictMode`] directly:
//! [`crate::parser::OxcParser`] always parses with
//! `oxc_span::SourceType::with_module(true)`, so the root scope's
//! [`ScopeFlags::StrictMode`] is set for every input regardless of the
//! boundary's analysis-level source type. It is instead derived from
//! the analysis-level source type at the root and propagated by parent
//! inheritance (see `scope_mapping_test`).
//!
//! `variable_scope` is computed inline: `Scoping::scope_descendants_from_root`
//! iterates scopes in DFS order, so each parent's row is already in
//! place when a child is pushed and the child can resolve its
//! `variable_scope` from the parent's (see `scope_mapping_test`).
//! `child_scopes` is then filled in a second pass once every scope's
//! `upper` is set.
//!
//! `variables` / `references` / `through` stay empty here; the
//! respective entity passes (`variable_mapping`, `reference_mapping`)
//! fill them. `function_expression_scope` stays `false`: the parity
//! baseline does not allocate a separate `FunctionExpressionName`
//! scope for a named function expression's self-name, so the adapter
//! emits only the `Function` scope. The self-name binding itself is
//! skipped in `variable_mapping`, and resolved references to it are
//! redirected to root-scope implicit globals in `reference_mapping`.
//!
//! ## OxcScopeId → IrScopeId translation
//!
//! The walk is not a 1:1 `usize` cast. Scopes that the parity
//! baseline collapses into a single row (currently the merge of a
//! `CatchClause` with its catch-body `BlockStatement`) skip emitting
//! their own IR row and instead route to the parent's, while
//! TypeScript type-only scopes are dropped entirely via
//! [`unsnarl_oxc_parity::is_type_only_subtree`]. The
//! [`ScopeMappingResult::translation`] table records, for every
//! `OxcScopeId`, either the IR id that should be observed for it or
//! `None` if the scope is filtered out; downstream passes
//! ([`super::variable_mapping`], [`super::reference_mapping`]) use
//! this map to resolve `scope_id` references coming out of `Scoping`
//! and skip any rows whose scope is `None`.
//!
//! ## SwitchCase synthesis
//!
//! `oxc_semantic` does not allocate a scope per `SwitchCase` — the
//! cases share their enclosing `SwitchStatement`'s scope. The parity
//! baseline instead creates a separate `Block` scope per case (anchored
//! to the `SwitchCase` AST node) so block-scoped declarations stay
//! contained within their case. This module synthesises those rows and
//! re-routes each oxc-derived child of the switch to the owning case
//! (mechanics in [`upper_for`]; behaviour pinned by `scope_mapping_test`).
//!
//! ## `ClassFieldInitializer`
//!
//! Absent intentionally: the parity fixtures' `expected.ir.json`
//! files carry no `class-field-initializer` row; class scopes hold
//! field-initialiser references directly. The adapter follows suit
//! and emits no such scope either.

use std::collections::HashMap;

use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::scope::{ScopeFlags, ScopeId as OxcScopeId};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope::ScopeData;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;
use unsnarl_oxc_parity::AstType;

use crate::parser::SourceType;

mod build_anchor_node;
mod is_filtered_out;
mod is_merged_into_parent;
mod upper_for;

use build_anchor_node::build_anchor_node;
use is_filtered_out::is_filtered_out;
use is_merged_into_parent::is_merged_into_parent;
use upper_for::upper_for;

/// Output of [`build_scopes`]: the IR scope arena plus the
/// `OxcScopeId → Option<IrScopeId>` translation that downstream passes
/// use to resolve `Scoping`-side scope references.
pub(crate) struct ScopeMappingResult {
    pub(crate) scopes: IndexVec<ScopeId, ScopeData>,
    /// For every `OxcScopeId`, the IR scope id that downstream passes
    /// must observe, or `None` if the scope is filtered out (currently
    /// any TypeScript type-only subtree). Scopes that are merged into
    /// their parent (catch body block today) carry `Some(parent_ir)`.
    pub(crate) translation: IndexVec<OxcScopeId, Option<ScopeId>>,
    /// For each IR switch scope, the synthetic per-`SwitchCase` Block
    /// scopes' spans + ir ids. Downstream passes (`reference_mapping`,
    /// `variable_mapping`) use this to re-parent any reference /
    /// binding whose `Scoping`-side `scope_id` is the switch scope but
    /// whose source position lies inside a specific `case` — eslint-
    /// scope puts those rows on the case's `Block` scope, while
    /// `oxc_semantic` keeps them on the bare switch. Cases are listed
    /// in source order.
    pub(crate) switch_cases: HashMap<ScopeId, Vec<(Span, ScopeId)>>,
}

/// Per-`SwitchStatement` record: each case's span and the synthetic
/// IR scope id allocated for it. Used by [`upper_for`] to redirect
/// oxc-derived children of the switch to the matching case.
struct SwitchInfo {
    cases: Vec<(Span, ScopeId)>,
}

/// Walk `semantic.scoping()`'s scope tree and produce the
/// `unsnarl_ir` arena rows alongside the `OxcScopeId → IrScopeId`
/// translation table. Most scopes are emitted 1:1, but a small set of
/// shapes are remapped:
///
/// - `CatchClause` + catch-body `BlockStatement`: merged into a single
///   `Catch` IR row.
/// - TypeScript type-only scopes (`TSModuleDeclaration` /
///   `TSTypeAliasDeclaration` / `TSInterfaceDeclaration` /
///   `TSConditionalType` / `TSMappedType`): dropped entirely along
///   with any descendants.
/// - `SwitchStatement`: one synthetic `Block` IR row per
///   `SwitchCase` is emitted immediately after the switch's row, and
///   nested scopes are re-parented to the case whose span contains
///   them.
pub(crate) fn build_scopes<'a>(
    semantic: &Semantic<'a>,
    source_type: SourceType,
    language: Language,
) -> ScopeMappingResult {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    let total = scoping.scopes_len();
    let mut translation: IndexVec<OxcScopeId, Option<ScopeId>> = IndexVec::with_capacity(total);
    let mut scopes: IndexVec<ScopeId, ScopeData> = IndexVec::with_capacity(total);
    let mut switch_info: HashMap<OxcScopeId, SwitchInfo> = HashMap::new();
    let root_is_strict = matches!(source_type, SourceType::Module);

    for oxc_id in scoping.scope_descendants_from_root() {
        debug_assert_eq!(translation.len(), oxc_id.index());
        let parent_routing = scoping.scope_parent_id(oxc_id).map(|p| translation[p]);
        if matches!(parent_routing, Some(None)) {
            translation.push(None);
            continue;
        }
        let kind = nodes.kind(scoping.get_node_id(oxc_id));
        if is_filtered_out(&kind) {
            translation.push(None);
            continue;
        }
        if is_merged_into_parent(oxc_id, scoping, nodes) {
            let parent_ir = parent_routing
                .flatten()
                .expect("merged scope must have an IR-visible parent");
            translation.push(Some(parent_ir));
            continue;
        }
        let new_id = ScopeId::from_usize(scopes.len());
        let block = build_anchor_node(&kind, language);
        let flags = scoping.scope_flags(oxc_id);
        let ty = derive_scope_type(flags, &kind, source_type);
        let upper = upper_for(oxc_id, scoping, nodes, &translation, &switch_info);
        let is_strict = match upper {
            Some(upper_id) => scopes[upper_id].is_strict,
            None => root_is_strict,
        };
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
        translation.push(Some(new_id));

        if let AstKind::SwitchStatement(switch) = kind {
            let switch_ir = new_id;
            let switch_is_strict = scopes[switch_ir].is_strict;
            let switch_var_scope = scopes[switch_ir].variable_scope;
            let mut cases = Vec::with_capacity(switch.cases.len());
            for case in &switch.cases {
                let case_ir = ScopeId::from_usize(scopes.len());
                scopes.push(ScopeData::new(
                    ScopeType::Block,
                    switch_is_strict,
                    Some(switch_ir),
                    Vec::new(),
                    switch_var_scope,
                    AstNode::new(AstType::SwitchCase, case.span),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    false,
                ));
                cases.push((case.span, case_ir));
            }
            switch_info.insert(oxc_id, SwitchInfo { cases });
        }
    }

    for raw_index in 0..scopes.len() {
        let ir_id = ScopeId::from_usize(raw_index);
        if let Some(upper) = scopes[ir_id].upper {
            scopes[upper].child_scopes.push(ir_id);
        }
    }

    let switch_cases: HashMap<ScopeId, Vec<(Span, ScopeId)>> = switch_info
        .into_iter()
        .filter_map(|(oxc_id, info)| translation[oxc_id].map(|ir| (ir, info.cases)))
        .collect();

    ScopeMappingResult {
        scopes,
        translation,
        switch_cases,
    }
}

/// Derive the `ScopeType` for a scope from its anchor `AstKind` and
/// `ScopeFlags`. The anchor is the source of truth for most cases —
/// `oxc_semantic` uses empty flags for `BlockStatement` /
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

#[cfg(test)]
#[path = "scope_mapping_test.rs"]
mod scope_mapping_test;
