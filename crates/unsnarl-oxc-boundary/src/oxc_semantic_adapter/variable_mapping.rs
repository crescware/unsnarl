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
//! `Scoping::iter_bindings_in` returns bindings by iterating the
//! underlying `hashbrown::HashMap`, which does not preserve insertion
//! order. The parity baseline appends each scope's `variables` array
//! in declaration-site order (post-hoisting: var / function / class
//! declarations land in source order). Sort the per-scope binding
//! list by `Scoping::symbol_span(sid).start` before emitting rows so
//! downstream consumers that index `variables` positionally see the
//! same order as the parity baseline.

use std::collections::{HashMap, HashSet};

use oxc_ast::ast::{ClassType, FunctionType};
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::{Scoping, Semantic};
use oxc_span::Span;
use oxc_syntax::scope::ScopeId as OxcScopeId;
use oxc_syntax::symbol::SymbolId;

use unsnarl_ir::ids::{DefinitionId, ScopeId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::DefinitionType;
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

    // Per-case `variables` lists must end up in declaration order;
    // re-parenting above can interleave cases. Re-sort each case's
    // binding list by symbol declaration span so the output matches
    // the parity baseline.
    for cases in switch_cases.values() {
        for (_, case_ir) in cases {
            scopes[*case_ir].variables.sort_by_key(|v| {
                variables[*v]
                    .identifiers
                    .first()
                    .map(|i| i.span.start)
                    .unwrap_or(0)
            });
        }
    }

    VariableMappingResult {
        variables,
        symbol_to_variable,
        synthetic_unresolved,
        inner_class_names,
    }
}

/// Reparent a binding to the per-case Block scope inside a switch.
///
/// Only relocate when the binding's declaring scope IS the switch
/// scope itself — i.e. `oxc_semantic` placed the binding directly on
/// the switch row (a `let` / `const` / function declaration written
/// directly under a `case` consequent with no wrapping block). In
/// that situation the parity baseline expects the binding to live on
/// the synthetic per-`SwitchCase` `Block` row, so the adapter picks
/// the case-Block whose span contains the binding (falling back to
/// the bare switch when the binding sits outside every case head).
///
/// Crucially, bindings whose `oxc_semantic` declaring scope is an
/// *ancestor* of a switch must stay put. The motivating case is
/// `function f() { switch (k) { case 1: var x; } }`: `var x` is
/// hoisted to the function scope (matching ECMAScript semantics and
/// `oxc_semantic`'s `Binder` for `VariableDeclarator`), so
/// `iter_bindings_in(function_scope)` yields the symbol with
/// `symbol_span` pointing inside the case. Reparenting it to the
/// case-Block would silently move `x` out of the function scope's
/// `set` / `variables`, breaking lookups from any code outside the
/// switch.
fn reparent_binding_to_switch_case(
    ir_scope: ScopeId,
    span: Span,
    scopes: &IndexVec<ScopeId, ScopeData>,
    switch_cases: &HashMap<ScopeId, Vec<(Span, ScopeId)>>,
) -> ScopeId {
    let Some(cases) = switch_cases.get(&ir_scope) else {
        return ir_scope;
    };
    let switch_span = scopes[ir_scope].block.span;
    if span.start < switch_span.start || span.end > switch_span.end {
        return ir_scope;
    }
    for (case_span, case_ir) in cases {
        if case_span.start <= span.start && span.end <= case_span.end {
            return *case_ir;
        }
    }
    ir_scope
}

/// If `anchor` is the `Function` node of a named function expression,
/// return its self-name. Used by [`build_variables`] to detect the
/// binding that must be skipped per the parity baseline (see the
/// module header).
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

/// Returns true if `symbol_id`'s declaration node is a TypeScript
/// parameter property — a `FormalParameter` (or `FormalParameterRest`)
/// carrying `accessibility` / `readonly` / `override`. Those slots
/// represent class fields rather than parameters, so the parity
/// baseline does not record them as parameter bindings.
fn is_typescript_parameter_property(
    scoping: &Scoping,
    nodes: &oxc_semantic::AstNodes<'_>,
    symbol_id: SymbolId,
) -> bool {
    let kind = nodes.kind(scoping.symbol_declaration(symbol_id));
    match kind {
        AstKind::FormalParameter(fp) => fp.accessibility.is_some() || fp.readonly || fp.r#override,
        _ => false,
    }
}

/// Returns true if `symbol_id`'s declaration is a TypeScript
/// type-only function (`declare function f(): void`, parsed by oxc as
/// `Function { type: TSDeclareFunction, ... }`, or an overload
/// signature parsed as `TSEmptyBodyFunctionExpression`). The parity
/// baseline drops such functions, so the binding never makes it into
/// the IR variable list.
fn is_type_only_function_declaration(
    scoping: &Scoping,
    nodes: &oxc_semantic::AstNodes<'_>,
    symbol_id: SymbolId,
) -> bool {
    let kind = nodes.kind(scoping.symbol_declaration(symbol_id));
    let AstKind::Function(func) = kind else {
        return false;
    };
    matches!(
        func.r#type,
        FunctionType::TSDeclareFunction | FunctionType::TSEmptyBodyFunctionExpression
    )
}

/// Returns true if any ancestor of `symbol_id`'s declaration node is
/// a TypeScript type-only construct that
/// [`unsnarl_oxc_parity::is_type_only_subtree`] marks as type-only.
/// The parity baseline never declares the inner binding in such
/// subtrees, so the variable is omitted from the IR variable list.
fn is_under_type_only_declaration(
    scoping: &Scoping,
    nodes: &oxc_semantic::AstNodes<'_>,
    symbol_id: SymbolId,
) -> bool {
    let mut cur = scoping.symbol_declaration(symbol_id);
    loop {
        if matches!(
            nodes.kind(cur),
            AstKind::TSImportEqualsDeclaration(_)
                | AstKind::TSExportAssignment(_)
                | AstKind::TSNamespaceExportDeclaration(_)
                | AstKind::TSInterfaceDeclaration(_)
                | AstKind::TSTypeAliasDeclaration(_)
                | AstKind::TSEnumDeclaration(_)
                | AstKind::TSModuleDeclaration(_),
        ) {
            return true;
        }
        let next = nodes.parent_id(cur);
        if next == cur {
            return false;
        }
        cur = next;
    }
}

/// If `anchor` is the `Class` node of a named class *declaration*,
/// return `(name, id_span, class_span)`. Class *expressions* already
/// receive an inner-name binding from `oxc_semantic`, so they return
/// `None` here.
fn inner_class_declaration_name<'a>(
    anchor: &'a AstKind<'_>,
) -> Option<(&'a str, oxc_span::Span, oxc_span::Span)> {
    let AstKind::Class(class) = anchor else {
        return None;
    };
    if !matches!(class.r#type, ClassType::ClassDeclaration) {
        return None;
    }
    let id = class.id.as_ref()?;
    Some((id.name.as_str(), id.span, class.span))
}

/// Synthesise the inner `ClassName` binding plus its `ClassName`
/// definition for a class declaration. Returns the new `VariableId`
/// so the caller can record it for the reference-mapping rebind pass.
fn push_inner_class_name(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    scope: ScopeId,
    name: &str,
    id_span: oxc_span::Span,
    class_span: oxc_span::Span,
) -> VariableId {
    let identifier = AstIdentifier::new(AstType::Identifier, name.to_string(), id_span);
    let var_id = variables.push(VariableData::new(
        name.to_string(),
        scope,
        vec![identifier.clone()],
        Vec::new(),
        Vec::new(),
    ));
    scopes[scope].insert_into_set(name.to_string(), var_id);
    scopes[scope].variables.push(var_id);
    let def_id = definitions.push(DefinitionData {
        r#type: DefinitionType::ClassName,
        name: identifier,
        node: AstNode {
            r#type: AstType::ClassDeclaration,
            span: class_span,
        },
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    });
    variables[var_id].defs.push(def_id);
    var_id
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
