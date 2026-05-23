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
//! fill them. `function_expression_scope` stays `false`: the
//! boundary's hand-rolled walker never allocates a separate
//! `FunctionExpressionName` scope for a named function expression's
//! self-name (it classifies the `Function.id` slot as a direct
//! binding but does not push a scope), so the adapter mirrors that
//! shape and emits only the `Function` scope. The self-name binding
//! itself is skipped in `variable_mapping`, and resolved references
//! to it are redirected to root-scope implicit globals in
//! `reference_mapping`.
//!
//! ## OxcScopeId → IrScopeId translation
//!
//! The walk is no longer a 1:1 `usize` cast: scopes the eslint-scope
//! model collapses into a single row (currently the `CatchClause` +
//! catch-body `BlockStatement` merge) skip emitting their own IR row
//! and instead route to the parent's, while TypeScript type-only
//! scopes are dropped entirely (the hand-rolled walker never enters
//! them via [`unsnarl_oxc_parity::is_type_only_subtree`]). The
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
//! cases share their enclosing `SwitchStatement`'s scope. eslint-scope,
//! by contrast, creates a separate `Block` scope per case (anchored
//! to the `SwitchCase` AST node) so block-scoped declarations stay
//! contained within their case. This module synthesises those `Block`
//! scopes immediately after each `SwitchStatement` IR row is emitted:
//! the cases occupy the next `cases.len()` IR ids. Any oxc-derived
//! scope whose `Scoping` parent is the `SwitchStatement` is re-routed
//! to the case whose span encloses its anchor (computed in
//! [`upper_for`]), keeping each case's nested scopes parented to the
//! synthetic case row instead of the bare switch.
//!
//! ## Known divergences (deferred to follow-up commits)
//!
//! No remaining scope-shape divergences; both the
//! `FunctionExpressionName` and `ClassFieldInitializer` cases noted
//! in earlier drafts turn out to be no-ops for parity (see above and
//! below).
//!
//! `ClassFieldInitializer` is absent from both sides intentionally:
//! the npm `eslint-scope` package would create one per
//! `PropertyDefinition` initialiser, but the boundary's hand-rolled
//! walker does not emit one (the parity fixtures' `expected.ir.json`
//! files confirm this — class scopes hold field-initialiser references
//! directly, with no nested `class-field-initializer` row). The
//! adapter follows suit and emits no such scope either.
//!
//! Remaining divergences are gated on a parity-harness signal (Phase
//! 2 step 5); the comment is kept in code so reviewers see the exact
//! scope of this commit's coverage rather than discovering it from
//! test output.

use std::collections::HashMap;

use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::{AstNodes, Scoping, Semantic};
use oxc_span::{GetSpan, Span};
use oxc_syntax::scope::{ScopeFlags, ScopeId as OxcScopeId};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope::ScopeData;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;
use unsnarl_oxc_parity::AstType;

use crate::materialise::ast_node_of;
use crate::parser::SourceType;

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
                    AstNode {
                        r#type: AstType::SwitchCase,
                        span: case.span,
                    },
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

/// Build the `AstNode` recorded on a scope's `block` field, applying
/// the TypeScript-only `Program` span normalisation when the scope's
/// anchor is the root `Program`.
///
/// Background: npm `oxc-parser` exposes `program.start = 0` for
/// `lang: "js" / "jsx"`, but for `lang: "ts" / "tsx"` it advances
/// past any leading hashbang and leading line / block comments so
/// `program.start` lands on the first directive / body statement.
/// The Rust `oxc_parser` crate emits `Program.span.start = 0`
/// unconditionally, so the boundary's hand-rolled walker normalises
/// the start in [`crate::analyze::analyze`] before pushing the root
/// scope. Mirror that normalisation here so the adapter's root
/// `block.span` matches the parity baseline for TypeScript inputs
/// whose source begins with comments / a hashbang (e.g.
/// cytoscape.min.js).
fn build_anchor_node(kind: &AstKind<'_>, language: Language) -> AstNode {
    let mut node = ast_node_of(kind);
    if matches!(kind, AstKind::Program(_)) && matches!(language, Language::Ts | Language::Tsx) {
        if let AstKind::Program(program) = kind {
            let normalised_start = program
                .directives
                .first()
                .map(|d| d.span.start)
                .or_else(|| program.body.first().map(|s| s.span().start))
                .or_else(|| program.hashbang.as_ref().map(|h| h.span.end))
                .unwrap_or(program.span.start);
            node.span = Span::new(normalised_start, program.span.end);
        }
    }
    node
}

/// Compute the IR `upper` for a non-merged, non-filtered oxc scope.
///
/// For most scopes the upper is the parent's translated IR id. When
/// the parent's anchor is a `SwitchStatement`, the upper is rewired
/// to the synthetic case `Block` scope whose span encloses this
/// scope's anchor (see [`build_scopes`]'s `SwitchCase` synthesis).
fn upper_for(
    oxc_id: OxcScopeId,
    scoping: &Scoping,
    nodes: &AstNodes<'_>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    switch_info: &HashMap<OxcScopeId, SwitchInfo>,
) -> Option<ScopeId> {
    let parent_oxc = scoping.scope_parent_id(oxc_id)?;
    let parent_ir = translation[parent_oxc]?;
    if let Some(info) = switch_info.get(&parent_oxc) {
        let anchor_span = nodes.kind(scoping.get_node_id(oxc_id)).span();
        for (case_span, case_ir) in &info.cases {
            if case_span.start <= anchor_span.start && anchor_span.end <= case_span.end {
                return Some(*case_ir);
            }
        }
    }
    Some(parent_ir)
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

/// Predicate: does this oxc scope have no IR row of its own, instead
/// merging into the parent's row?
///
/// Two merge cases:
///
/// * Catch body `BlockStatement`: `oxc_semantic` emits a separate
///   `BlockStatement` scope for `catch (e) { ... }`'s body block,
///   while eslint-scope folds both the catch parameter and the body's
///   declarations into a single `Catch` scope. The body block is
///   detected by its parent in [`Scoping::scope_parent_id`] being a
///   `CatchClause` scope.
/// * `WithStatement`: `oxc_semantic` allocates a dedicated With scope
///   (`ScopeFlags::With`), but the hand-rolled walker has no
///   `visit_with_statement` override and lets the default walk descend
///   straight into the body — so the body's `BlockStatement` ends up
///   parented directly under the enclosing scope, no separate With
///   scope row exists in the parity baseline. Mirror that by treating
///   the `WithStatement` scope as merged into its parent; its body
///   block scope (a regular `BlockStatement`) keeps its own IR row
///   with the With's parent as its `upper`.
fn is_merged_into_parent(oxc_id: OxcScopeId, scoping: &Scoping, nodes: &AstNodes<'_>) -> bool {
    let kind = nodes.kind(scoping.get_node_id(oxc_id));
    if matches!(kind, AstKind::WithStatement(_)) {
        return true;
    }
    let Some(parent) = scoping.scope_parent_id(oxc_id) else {
        return false;
    };
    let parent_kind = nodes.kind(scoping.get_node_id(parent));
    matches!(kind, AstKind::BlockStatement(_)) && matches!(parent_kind, AstKind::CatchClause(_))
}

/// Predicate: should this scope (and its entire subtree) be omitted
/// from the IR scope tree?
///
/// `oxc_semantic` allocates a scope for several TypeScript type-only
/// constructs (`type X = ...`, `interface X { ... }`, `namespace X
/// { ... }`, mapped / conditional types). The hand-rolled walker
/// recognises the surrounding subtree via
/// [`unsnarl_oxc_parity::is_type_only_subtree`] and never enters
/// scope-creation for them; mirror that behaviour by dropping the
/// scope's IR row outright. Filtering propagates to descendants in
/// the calling loop via the inherited-filter check.
fn is_filtered_out(kind: &AstKind<'_>) -> bool {
    matches!(
        kind,
        AstKind::TSModuleDeclaration(_)
            | AstKind::TSTypeAliasDeclaration(_)
            | AstKind::TSInterfaceDeclaration(_)
            | AstKind::TSConditionalType(_)
            | AstKind::TSMappedType(_)
    )
}

fn is_var_creating(flags: ScopeFlags) -> bool {
    flags.is_var()
}

#[cfg(test)]
#[path = "scope_mapping_test.rs"]
mod scope_mapping_test;
