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
//! enclosing `Function` scope. The parity baseline has no `inner` row
//! anywhere; references to `inner` inside the body fall through to
//! the implicit-global path on the root scope.
//!
//! Skip emitting a `VariableData` for the function-expression
//! self-name symbol and record its `SymbolId` in
//! [`VariableMappingResult::synthetic_unresolved`].
//! [`super::reference_mapping`] reads that set and redirects any
//! `Reference`s `oxc_semantic` resolved against the skipped symbol
//! into the implicit-global synthesis path.
//!
//! ## TypeScript parameter properties
//!
//! `constructor(public x: number, private y)` declares `x` / `y` as
//! class fields rather than function parameters. The parity baseline
//! does not record them as parameter bindings; instead, the binding
//! identifier classifies as an ordinary reference (resolving as an
//! implicit global).
//!
//! When a symbol's declaration node is a `FormalParameter` carrying
//! `accessibility` / `readonly` / `override`, skip emitting the
//! `VariableData` and record the symbol in `synthetic_unresolved` so
//! [`super::reference_mapping`] re-routes the
//! `oxc_semantic`-resolved references against the parameter to the
//! implicit-global path.
//!
//! ## Inner `ClassName` for class declarations
//!
//! For a class *declaration* (`class C { ... }`), `oxc_semantic`
//! binds `C` only in the enclosing scope. The parity baseline adds a
//! *second* binding for `C` inside the `Class` scope so references to
//! `C` from inside method bodies resolve to the inner row instead of
//! the outer one. Class *expressions* (`const C = class D { ... }`)
//! already get their inner-name binding from `oxc_semantic`, so no
//! synthesis is needed for them.
//!
//! Synthesise the inner `ClassName` row here for class declarations
//! only, emitting both the `VariableData` and the corresponding
//! `DefinitionData` (`DefinitionType::ClassName`).
//!
//! `identifiers` carries one entry per binding-identifier occurrence.
//! `oxc_semantic` collapses re-declarations into a single `SymbolId`;
//! the per-occurrence spans are recovered from
//! `Scoping::symbol_redeclarations` when present, otherwise from
//! `Scoping::symbol_span` (a single-occurrence binding).
//!
//! Additionally, this pass synthesises the implicit per-function
//! `arguments` binding. `oxc_semantic` does not bind `arguments`,
//! while the parity baseline inserts an `arguments` `Variable` with
//! no defs and no identifiers into every non-arrow function's local
//! scope.
//!
//! ## Binding ordering
//!
//! `Scoping::iter_bindings_in` yields bindings in `hashbrown::HashMap`
//! order, which is not source order. Each scope's emitted `variables`
//! must instead match the parity baseline's declaration-site
//! (post-hoisting source) order, because downstream consumers index
//! `variables` positionally; the synthesised `arguments` binding leads
//! its function scope's list. This ordering invariant is locked by the
//! ordering tests in `variable_mapping_test.rs`, not by this prose.

use std::collections::{HashMap, HashSet};

use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::scope::ScopeId as OxcScopeId;
use oxc_syntax::symbol::SymbolId;

use unsnarl_ir::ids::{DefinitionId, ScopeId, VariableId};
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};

mod build_identifiers;
mod inner_class_declaration_name;
mod is_type_only_function_declaration;
mod is_typescript_parameter_property;
mod is_under_type_only_declaration;
mod named_function_expression_self_name;
mod push_implicit_arguments;
mod push_inner_class_name;
mod reparent_binding_to_switch_case;

use build_identifiers::build_identifiers;
use inner_class_declaration_name::inner_class_declaration_name;
use is_type_only_function_declaration::is_type_only_function_declaration;
use is_typescript_parameter_property::is_typescript_parameter_property;
use is_under_type_only_declaration::is_under_type_only_declaration;
use named_function_expression_self_name::named_function_expression_self_name;
use push_implicit_arguments::push_implicit_arguments;
use push_inner_class_name::push_inner_class_name;
use reparent_binding_to_switch_case::reparent_binding_to_switch_case;

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
    /// globals (matching the parity baseline, which never allocates a
    /// `VariableData` for named function-expression `id` slots — see
    /// the module header).
    pub(crate) synthetic_unresolved: HashSet<SymbolId>,
    /// For each class *declaration* that received a synthesised inner
    /// `ClassName` binding, the outer `VariableId` (the binding in the
    /// class scope's parent that `oxc_semantic` knows about) and the
    /// inner `VariableId` (synthesised inside the class scope so
    /// references from method bodies resolve to it instead of the
    /// outer one). The class scope's `block.span` is read off
    /// `scopes[class_scope].block.span` at use time.
    pub(crate) inner_class_names: Vec<InnerClassName>,
}

/// One synthesised inner `ClassName` binding, recorded so the
/// reference-mapping pass can re-resolve references inside the class
/// scope from the outer binding to the inner one.
pub(crate) struct InnerClassName {
    /// IR id of the class scope that owns the inner binding.
    pub(crate) class_scope: ScopeId,
    /// IR id of the inner `ClassName` variable.
    pub(crate) inner: VariableId,
}

/// Walk `semantic.scoping()`'s symbol table and produce the
/// `unsnarl_ir` arena's `variables` rows, while populating each
/// scope's `variables` / `set` indexes in `scopes`.
///
/// `scopes` is taken `&mut` because `ScopeData::variables` is a
/// positional list of declared bindings and `ScopeData::set` is the
/// name → id index; both are populated here rather than
/// reconstructed later.
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
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    switch_cases: &HashMap<ScopeId, Vec<(Span, ScopeId)>>,
) -> VariableMappingResult {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    let mut variables: IndexVec<VariableId, VariableData> = IndexVec::new();
    let mut symbol_to_variable: IndexVec<SymbolId, Option<VariableId>> =
        std::iter::repeat_with(|| None)
            .take(scoping.symbols_len())
            .collect();
    let mut synthetic_unresolved: HashSet<SymbolId> = HashSet::new();
    let mut inner_class_names: Vec<InnerClassName> = Vec::new();

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
        if let Some((name, span, class_span)) = inner_class_declaration_name(&anchor) {
            let inner = push_inner_class_name(
                scopes,
                &mut variables,
                definitions,
                ir_scope,
                name,
                span,
                class_span,
            );
            inner_class_names.push(InnerClassName {
                class_scope: ir_scope,
                inner,
            });
        }

        let mut bindings: Vec<SymbolId> = scoping.iter_bindings_in(oxc_scope_id).collect();
        // Emit in source order: `iter_bindings_in` is HashMap-ordered,
        // so sort by declaration span to match the parity baseline.
        // The resulting per-scope order is locked by the sibling test.
        bindings.sort_by_key(|sid| scoping.symbol_span(*sid).start);
        for symbol_id in bindings {
            let name = scoping.symbol_name(symbol_id).to_string();
            if matches!(named_fe_self_name, Some(self_name) if self_name == name) {
                synthetic_unresolved.insert(symbol_id);
                continue;
            }
            if is_typescript_parameter_property(scoping, nodes, symbol_id) {
                synthetic_unresolved.insert(symbol_id);
                continue;
            }
            if is_type_only_function_declaration(scoping, nodes, symbol_id) {
                synthetic_unresolved.insert(symbol_id);
                continue;
            }
            if is_under_type_only_declaration(scoping, nodes, symbol_id) {
                synthetic_unresolved.insert(symbol_id);
                continue;
            }
            let identifiers = build_identifiers(scoping, symbol_id, &name);
            // `oxc_semantic` keeps every per-`SwitchCase` binding on
            // the enclosing `SwitchStatement` scope. The parity
            // baseline — mirrored by `super::scope_mapping`'s
            // synthetic case `Block` scopes — pulls them down to the
            // case scope. Route the binding to the case scope whose
            // span encloses the symbol's declaration site.
            let target_scope = reparent_binding_to_switch_case(
                ir_scope,
                scoping.symbol_span(symbol_id),
                scopes,
                switch_cases,
            );
            let var_id = variables.push(VariableData::new(
                name.clone(),
                target_scope,
                identifiers,
                Vec::new(),
                Vec::new(),
            ));
            scopes[target_scope].insert_into_set(name, var_id);
            scopes[target_scope].variables.push(var_id);
            symbol_to_variable[symbol_id] = Some(var_id);
        }
    }

    VariableMappingResult {
        variables,
        symbol_to_variable,
        synthetic_unresolved,
        inner_class_names,
    }
}

#[cfg(test)]
#[path = "variable_mapping_test.rs"]
mod variable_mapping_test;
