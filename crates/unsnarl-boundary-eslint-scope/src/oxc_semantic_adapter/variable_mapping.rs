//! `oxc_semantic::Scoping` symbols → `IrArena.variables`.
//!
//! Walks every `(scope_id, symbol_id)` pair produced by
//! `Scoping::iter_bindings_in` and emits a [`unsnarl_ir::VariableData`]
//! per symbol: name, declaring scope, identifier occurrences, and
//! empty `references` / `defs` slots that the reference- and
//! definition-passes fill later.
//!
//! ## Named function-expression self-name handling
//!
//! `oxc_semantic` records the id of a named function expression
//! (`const f = function inner() { ... }`) as a binding inside the
//! enclosing `Function` scope. The boundary's hand-rolled walker, by
//! contrast, classifies that identifier as a direct binding via
//! `classify::is_direct_binding` (`Function.id` slot) but does *not*
//! allocate a `VariableData` for it — the parity baseline therefore
//! has no `inner` row anywhere, and references to `inner` inside the
//! body fall through to the implicit-global path on the root scope.
//!
//! Mirror that behaviour here: skip emitting a `VariableData` for the
//! function-expression self-name symbol and record its `SymbolId` in
//! [`VariableMappingResult::synthetic_unresolved`]. [`super::reference_mapping`]
//! reads that set and redirects any `Reference`s `oxc_semantic`
//! resolved against the skipped symbol into the implicit-global
//! synthesis path, matching the parity baseline.
//!
//! `identifiers` carries one entry per binding-identifier occurrence,
//! matching what the hand-rolled walker pushes on each
//! `declare_variable` call. `oxc_semantic` collapses re-declarations
//! into a single `SymbolId`; the per-occurrence spans are recovered
//! from `Scoping::symbol_redeclarations` when present, otherwise from
//! `Scoping::symbol_span` (a single-occurrence binding).
//!
//! Additionally, this pass synthesises the implicit per-function
//! `arguments` binding. `oxc_semantic` deliberately omits it (pinned by
//! `oxc_semantic_probe_test::arguments_is_or_is_not_a_symbol_inside_a_function`),
//! while eslint-scope inserts an `arguments` `Variable` with no defs
//! and no identifiers into every non-arrow function's local scope.
//!
//! ## Ordering caveat
//!
//! `Scoping::iter_bindings_in` returns bindings by iterating the
//! underlying `hashbrown::HashMap`, which does not preserve insertion
//! order. The hand-rolled walker, in contrast, appends to each scope's
//! `variables` array in declaration order (post-hoisting). The two
//! orderings therefore diverge for any scope with more than one
//! binding; downstream consumers that index `variables` positionally
//! will observe the difference. Aligning order is gated on the
//! parity harness signal (Phase 2 step 5).

use std::collections::HashSet;

use oxc_ast::ast::FunctionType;
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::{Scoping, Semantic};
use oxc_syntax::scope::ScopeId as OxcScopeId;
use oxc_syntax::symbol::SymbolId;

use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_ir::scope::{ScopeData, VariableData};
use unsnarl_oxc_parity::AstType;

/// Output of [`build_variables`]: the IR variable arena, the
/// `SymbolId → VariableId` projection, and the set of symbols that
/// were intentionally not emitted (named function-expression
/// self-names) and whose `oxc_semantic`-resolved references must
/// re-route through the implicit-global synthesis path in
/// [`super::reference_mapping`].
pub(crate) struct VariableMappingResult {
    pub(crate) variables: IndexVec<VariableId, VariableData>,
    pub(crate) symbol_to_variable: IndexVec<SymbolId, Option<VariableId>>,
    /// Symbols whose `VariableData` is intentionally absent so that
    /// resolved references to them can be re-emitted as implicit
    /// globals (matching the boundary's hand-rolled walker, which
    /// classifies named function-expression `id` slots as direct
    /// bindings but never allocates a `VariableData` for them — see
    /// the module header).
    pub(crate) synthetic_unresolved: HashSet<SymbolId>,
}

/// Walk `semantic.scoping()`'s symbol table and produce the
/// `unsnarl_ir` arena's `variables` rows, while populating each
/// scope's `variables` / `set` indexes in `scopes`.
///
/// `scopes` is taken `&mut` because eslint-scope's
/// `ScopeData::variables` is a positional list of declared bindings
/// and `ScopeData::set` is the name → id index; both are populated
/// here rather than reconstructed later.
///
/// `symbol_to_variable` in the result is the `SymbolId → VariableId`
/// projection produced as a side effect: every `iter_bindings_in`
/// symbol that gets an IR row records its id at the symbol's slot so
/// [`super::reference_mapping`] can translate `oxc_semantic`'s
/// `Reference::symbol_id` into the matching `VariableId`. Slots stay
/// `None` for symbols that either live inside a filtered TypeScript
/// type-only subtree (their IR scope is `None`) or are the self-name
/// of a named function expression (skipped per the module-header
/// rationale).
pub(crate) fn build_variables(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
) -> VariableMappingResult {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    let mut variables: IndexVec<VariableId, VariableData> = IndexVec::new();
    let mut symbol_to_variable: IndexVec<SymbolId, Option<VariableId>> =
        std::iter::repeat_with(|| None)
            .take(scoping.symbols_len())
            .collect();
    let mut synthetic_unresolved: HashSet<SymbolId> = HashSet::new();

    for oxc_scope_id in scoping.scope_descendants_from_root() {
        let Some(ir_scope) = translation[oxc_scope_id] else {
            continue;
        };
        let node_id = scoping.get_node_id(oxc_scope_id);
        let anchor = nodes.kind(node_id);

        let named_fe_self_name = named_function_expression_self_name(&anchor);

        if matches!(anchor, AstKind::Function(_)) {
            push_implicit_arguments(scopes, &mut variables, ir_scope);
        }

        for symbol_id in scoping.iter_bindings_in(oxc_scope_id) {
            let name = scoping.symbol_name(symbol_id).to_string();
            if matches!(named_fe_self_name, Some(self_name) if self_name == name) {
                synthetic_unresolved.insert(symbol_id);
                continue;
            }
            let identifiers = build_identifiers(scoping, symbol_id, &name);
            let var_id = variables.push(VariableData::new(
                name.clone(),
                ir_scope,
                identifiers,
                Vec::new(),
                Vec::new(),
            ));
            scopes[ir_scope].insert_into_set(name, var_id);
            scopes[ir_scope].variables.push(var_id);
            symbol_to_variable[symbol_id] = Some(var_id);
        }
    }

    VariableMappingResult {
        variables,
        symbol_to_variable,
        synthetic_unresolved,
    }
}

/// If `anchor` is the `Function` node of a named function expression,
/// return its self-name. Used by [`build_variables`] to detect the
/// binding that must be skipped per the hand-rolled walker's
/// behaviour (see the module header).
fn named_function_expression_self_name<'a>(anchor: &'a AstKind<'_>) -> Option<&'a str> {
    let AstKind::Function(func) = anchor else {
        return None;
    };
    if !matches!(
        func.r#type,
        FunctionType::FunctionExpression | FunctionType::TSEmptyBodyFunctionExpression
    ) {
        return None;
    }
    func.id.as_ref().map(|id| id.name.as_str())
}

fn push_implicit_arguments(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    scope: ScopeId,
) {
    let name = "arguments";
    if scopes[scope].set().contains_key(name) {
        return;
    }
    let var_id = variables.push(VariableData::new(
        name.to_string(),
        scope,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    scopes[scope].insert_into_set(name.to_string(), var_id);
    scopes[scope].variables.push(var_id);
}

fn build_identifiers(scoping: &Scoping, symbol_id: SymbolId, name: &str) -> Vec<AstIdentifier> {
    let redeclarations = scoping.symbol_redeclarations(symbol_id);
    if redeclarations.is_empty() {
        vec![AstIdentifier::new(
            AstType::Identifier,
            name.to_string(),
            scoping.symbol_span(symbol_id),
        )]
    } else {
        redeclarations
            .iter()
            .map(|r| AstIdentifier::new(AstType::Identifier, name.to_string(), r.span))
            .collect()
    }
}

#[cfg(test)]
#[path = "variable_mapping_test.rs"]
mod variable_mapping_test;
