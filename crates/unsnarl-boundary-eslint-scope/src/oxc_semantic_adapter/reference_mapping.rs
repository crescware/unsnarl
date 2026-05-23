//! `oxc_semantic::Scoping` references → `IrArena.references`.
//!
//! Walks every reference materialised by `SemanticBuilder` —
//! `Scoping::symbol_ids()` ⇒ `get_resolved_reference_ids(sid)` for
//! the resolved set, plus `Scoping::root_unresolved_references()`
//! for the unresolved set — and emits one [`unsnarl_ir::ReferenceData`]
//! per reference. The pass also populates the cross-link sites that
//! the hand-rolled walker maintains:
//!
//! * `ScopeData::references` — appended on every reference at its
//!   creating scope.
//! * `VariableData::references` — appended when a reference resolves
//!   (to a real binding, an adapter-synthesised `arguments`, or an
//!   implicit-global Variable).
//! * `ScopeData::through` — only along the implicit-global path. The
//!   hand-rolled `resolve.rs` walks from the reference's scope up to
//!   and *including* the global scope, pushing the reference id on
//!   each scope's `through`; this pass mirrors that exactly.
//!
//! ## `arguments` synthesis
//!
//! `oxc_semantic` does not bind `arguments` (pinned by
//! `oxc_semantic_probe_test::arguments_is_or_is_not_a_symbol_inside_a_function`),
//! so an `arguments` identifier inside a function body surfaces as a
//! root-unresolved reference. `variable_mapping` has already inserted
//! a synthetic `arguments` `VariableData` into every non-arrow
//! function scope; this pass walks the scope chain from the
//! reference's scope upward, and if the first ancestor containing an
//! `arguments` binding is one of those synthetic rows, resolves the
//! reference to it. References to `arguments` outside any function
//! (e.g. module-level) fall through to the implicit-global path,
//! matching the hand-rolled `resolve_in_scope_chain` shape.
//!
//! ## Implicit globals
//!
//! Unresolved references that don't match the `arguments` case land
//! on a per-name `ImplicitGlobalVariable` on the root scope. The
//! first occurrence creates the `VariableData` (with one identifier
//! entry recording the reference's identifier span) plus the
//! `DefinitionData` row; subsequent occurrences for the same name
//! reuse the same `VariableId`. This mirrors the hand-rolled
//! `declare_implicit_global` exactly.
//!
//! ## `init` flag
//!
//! `oxc_semantic` does not emit a reference for the binding side of
//! `var x = 0` (pinned by
//! `oxc_semantic_probe_test::with_body_identifier_resolves_to_outer_binding_diverging_from_eslint_scope`'s
//! observation that the declaration carries the init directly). The
//! hand-rolled walker, in contrast, synthesises a write reference
//! with `init = true` at each binding inside a `VariableDeclarator`
//! that has an `init` expression. This pass walks `VariableDeclarator`
//! AST nodes after the regular reference loop and synthesises those
//! `init = true` writes, looking each binding identifier's symbol up
//! via `symbol_to_variable` so the synthesised reference resolves to
//! the right `VariableData`. Destructuring patterns are flattened to
//! their constituent binding identifiers (one `init` write per leaf).
//!
//! ## Known divergences (deferred to follow-up commits)
//!
//! 1. **`with` body resolution**: `oxc_semantic` resolves identifiers
//!    inside a `with (o) { ... }` block against the outer binding
//!    (pinned by the probe test linked above). Eslint-scope
//!    deliberately leaves them unresolved because the `with` extends
//!    the scope chain at runtime. This pass leaves `oxc_semantic`'s
//!    resolution intact; post-processing references inside `with`
//!    bodies is gated on the parity-harness signal.
//! 2. **JSX tag references**: lowercase JSX intrinsics (`<div />`)
//!    are not references on either side; uppercase ones (`<MyComp />`)
//!    are represented as `IdentifierReference` by the parser already,
//!    so they flow through this pass unchanged. No special-casing
//!    needed here.

use oxc_ast::ast::{BindingIdentifier, BindingPattern, FormalParameter};
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::GetSpan;
use oxc_syntax::reference::ReferenceFlags as OxcReferenceFlags;
use oxc_syntax::scope::ScopeId as OxcScopeId;
use oxc_syntax::symbol::SymbolId;

use std::collections::{HashMap, HashSet};

use unsnarl_ir::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::reference::reference_flags::{ReferenceFlagBits, ReferenceFlags};
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

/// Walk `semantic.scoping()`'s reference table and produce the
/// `unsnarl_ir` arena's `references` rows, populating cross-links on
/// `scopes` / `variables` / `definitions` along the way.
///
/// `symbol_to_variable` is the `SymbolId → VariableId` projection
/// produced by [`super::variable_mapping::build_variables`]; it lets
/// this pass translate `oxc_semantic`'s resolved-reference symbol
/// references into the matching `VariableId` without re-walking the
/// scope tree.
pub(crate) fn build_references(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    symbol_to_variable: &IndexVec<SymbolId, Option<VariableId>>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    synthetic_unresolved: &HashSet<SymbolId>,
) -> IndexVec<ReferenceId, ReferenceData> {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    let mut references: IndexVec<ReferenceId, ReferenceData> = IndexVec::new();
    let root = ScopeId::from_usize(0);
    let mut implicit_globals: HashMap<String, VariableId> = HashMap::new();

    for sid in scoping.symbol_ids() {
        if let Some(var_id) = symbol_to_variable[sid] {
            for &oxc_ref_id in scoping.get_resolved_reference_ids(sid) {
                let oxc_ref = scoping.get_reference(oxc_ref_id);
                let Some(from) = translation[oxc_ref.scope_id()] else {
                    continue;
                };
                let identifier = build_identifier(nodes.kind(oxc_ref.node_id()));
                let flags = convert_flags(oxc_ref.flags());
                let new_id = references.push(ReferenceData {
                    identifier,
                    from,
                    resolved: Some(var_id),
                    init: false,
                    flags,
                });
                scopes[from].references.push(new_id);
                variables[var_id].references.push(new_id);
            }
            continue;
        }
        if synthetic_unresolved.contains(&sid) {
            // The boundary's hand-rolled walker never allocates a
            // `VariableData` for a named function-expression self-name
            // (see `variable_mapping`'s module header). References
            // `oxc_semantic` resolved against this symbol must be
            // re-emitted through the implicit-global path so they end
            // up matching the parity baseline (an unresolved read
            // resolving to a root-scope implicit global).
            let name = scoping.symbol_name(sid).to_string();
            for &oxc_ref_id in scoping.get_resolved_reference_ids(sid) {
                let oxc_ref = scoping.get_reference(oxc_ref_id);
                let Some(from) = translation[oxc_ref.scope_id()] else {
                    continue;
                };
                let identifier = build_identifier(nodes.kind(oxc_ref.node_id()));
                let flags = convert_flags(oxc_ref.flags());
                let var_id = ensure_implicit_global(
                    scopes,
                    variables,
                    definitions,
                    &mut implicit_globals,
                    root,
                    &name,
                    &identifier,
                );
                let new_id = references.push(ReferenceData {
                    identifier,
                    from,
                    resolved: Some(var_id),
                    init: false,
                    flags,
                });
                scopes[from].references.push(new_id);
                variables[var_id].references.push(new_id);
                push_through_chain(scopes, from, root, new_id);
            }
            continue;
        }
        // Otherwise this symbol lives in a filtered (TypeScript
        // type-only) scope; its references aren't part of the runtime
        // IR either.
    }

    synthesise_init_references(
        semantic,
        scopes,
        variables,
        &mut references,
        symbol_to_variable,
        translation,
    );

    // `Scoping::root_unresolved_references` is keyed on a
    // `hashbrown::HashMap`, so its iteration order is arbitrary. The
    // hand-rolled walker encounters identifiers in source order, so
    // implicit globals appear in source order too. Sort here by the
    // first reference's identifier span so the IR `variables` /
    // implicit-globals ordering matches the parity baseline.
    let mut unresolved: Vec<_> = scoping.root_unresolved_references().iter().collect();
    unresolved.sort_by_key(|(_name, ref_ids)| {
        ref_ids
            .iter()
            .map(|&id| nodes.kind(scoping.get_reference(id).node_id()).span().start)
            .min()
            .unwrap_or(u32::MAX)
    });
    for (name_ident, ref_ids) in unresolved {
        let name = name_ident.as_str().to_string();
        for &oxc_ref_id in ref_ids.iter() {
            let oxc_ref = scoping.get_reference(oxc_ref_id);
            let Some(from) = translation[oxc_ref.scope_id()] else {
                continue;
            };
            let identifier = build_identifier(nodes.kind(oxc_ref.node_id()));
            let flags = convert_flags(oxc_ref.flags());

            let synth_args = if name == "arguments" {
                resolve_synthetic_arguments(scopes, from)
            } else {
                None
            };

            if let Some(var_id) = synth_args {
                let new_id = references.push(ReferenceData {
                    identifier,
                    from,
                    resolved: Some(var_id),
                    init: false,
                    flags,
                });
                scopes[from].references.push(new_id);
                variables[var_id].references.push(new_id);
            } else {
                let var_id = ensure_implicit_global(
                    scopes,
                    variables,
                    definitions,
                    &mut implicit_globals,
                    root,
                    &name,
                    &identifier,
                );
                let new_id = references.push(ReferenceData {
                    identifier,
                    from,
                    resolved: Some(var_id),
                    init: false,
                    flags,
                });
                scopes[from].references.push(new_id);
                variables[var_id].references.push(new_id);
                push_through_chain(scopes, from, root, new_id);
            }
        }
    }

    synthesise_parameter_property_references(
        semantic,
        scopes,
        variables,
        &mut references,
        definitions,
        &mut implicit_globals,
        translation,
        root,
    );

    mark_variable_declarator_init_reads(semantic, &mut references);

    sort_reference_lists_by_source_order(scopes, variables, &references);

    references
}

/// Sort each scope's `references` / `through` list and each variable's
/// `references` list by the underlying identifier's source offset.
///
/// The hand-rolled walker pushes references to these lists in source
/// order because it traverses the AST once and emits each reference at
/// the moment of encounter. This pass instead walks `Scoping`'s
/// symbol-keyed reference tables first, then performs separate
/// AST-walking synthesis passes (`synthesise_init_references`,
/// `synthesise_parameter_property_references`) and a sorted unresolved
/// loop afterwards, so per-scope and per-variable lists end up
/// interleaved by category rather than by source position. The IR
/// emitter [`unsnarl_emitter_ir::serializer::flat`] renumbers
/// references by source offset before serialization but preserves
/// these lists' order, so without this final sort the serialized
/// output's `scope.references` / `scope.through` / `variable.references`
/// lists would emit out-of-order ids relative to the parity baseline.
fn sort_reference_lists_by_source_order(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &IndexVec<ReferenceId, ReferenceData>,
) {
    let key = |r: &ReferenceId| references[*r].identifier.span.start;
    for scope in scopes.iter_mut() {
        scope.references.sort_by_key(key);
        scope.through.sort_by_key(key);
    }
    for var in variables.iter_mut() {
        var.references.sort_by_key(key);
    }
}

/// Mirror `classify_ordinary_reference`'s `init = true` flag for read
/// references that sit directly in the `init` slot of a
/// [`oxc_ast::ast::VariableDeclarator`].
///
/// The hand-rolled walker stamps `init = true` on any identifier whose
/// parent is `VariableDeclarator` and whose slot key is `"init"` (see
/// `crate::classify::classify_ordinary_reference`). The matching
/// references in this adapter come from [`oxc_semantic::Scoping`]'s
/// resolved-reference table (the regular Loop 1 above) or from the
/// `unresolved` loop, both of which default `init` to `false`. Walk
/// every `VariableDeclarator` in the program, look up the identifier
/// at the immediate `init` position, and stamp the matching
/// `ReferenceData::init` to `true`.
///
/// Only the immediate-child identifier case is handled: identifiers
/// nested inside a wrapping expression (`const x = a + b`) keep
/// `init = false` because `classify_ordinary_reference` sees a parent
/// like `BinaryExpression` rather than `VariableDeclarator` for them,
/// matching the hand-rolled walker's behaviour.
fn mark_variable_declarator_init_reads(
    semantic: &oxc_semantic::Semantic<'_>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
) {
    use oxc_ast::ast::Expression;

    let mut init_spans: HashSet<(u32, u32)> = HashSet::new();
    for node in semantic.nodes().iter() {
        let AstKind::VariableDeclarator(vd) = node.kind() else {
            continue;
        };
        let Some(init) = &vd.init else {
            continue;
        };
        if let Expression::Identifier(id) = init {
            init_spans.insert((id.span.start, id.span.end));
        }
    }
    if init_spans.is_empty() {
        return;
    }
    for r in references.iter_mut() {
        if init_spans.contains(&(r.identifier.span.start, r.identifier.span.end)) {
            r.init = true;
        }
    }
}

fn build_identifier(kind: AstKind<'_>) -> AstIdentifier {
    match kind {
        AstKind::IdentifierReference(ident) => AstIdentifier::new(
            AstType::Identifier,
            ident.name.as_str().to_string(),
            ident.span,
        ),
        AstKind::JSXIdentifier(ident) => AstIdentifier::new(
            AstType::JSXIdentifier,
            ident.name.as_str().to_string(),
            ident.span,
        ),
        other => panic!(
            "reference_mapping: expected IdentifierReference or JSXIdentifier at reference node, \
             got {other:?}",
        ),
    }
}

fn convert_flags(flags: OxcReferenceFlags) -> ReferenceFlagBits {
    let mut out = ReferenceFlags::NONE;
    if flags.is_read() {
        out |= ReferenceFlags::READ;
    }
    if flags.is_write() {
        out |= ReferenceFlags::WRITE;
    }
    out
}

fn resolve_synthetic_arguments(
    scopes: &IndexVec<ScopeId, ScopeData>,
    from: ScopeId,
) -> Option<VariableId> {
    let mut cur = Some(from);
    while let Some(s) = cur {
        if let Some(&id) = scopes[s].set().get("arguments") {
            return Some(id);
        }
        cur = scopes[s].upper;
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn ensure_implicit_global(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    implicit_globals: &mut HashMap<String, VariableId>,
    root: ScopeId,
    name: &str,
    first_occurrence: &AstIdentifier,
) -> VariableId {
    if let Some(&id) = implicit_globals.get(name) {
        return id;
    }
    let var_id = variables.push(VariableData::new(
        name.to_string(),
        root,
        vec![first_occurrence.clone()],
        Vec::new(),
        Vec::new(),
    ));
    scopes[root].insert_into_set(name.to_string(), var_id);
    scopes[root].variables.push(var_id);
    let node = AstNode {
        r#type: first_occurrence.r#type.clone(),
        span: first_occurrence.span,
    };
    let def_id = definitions.push(DefinitionData {
        r#type: DefinitionType::ImplicitGlobalVariable,
        name: first_occurrence.clone(),
        node,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    });
    variables[var_id].defs.push(def_id);
    implicit_globals.insert(name.to_string(), var_id);
    var_id
}

fn push_through_chain(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    from: ScopeId,
    root: ScopeId,
    ref_id: ReferenceId,
) {
    let mut cur = Some(from);
    while let Some(s) = cur {
        if s == root {
            break;
        }
        scopes[s].through.push(ref_id);
        cur = scopes[s].upper;
    }
    scopes[root].through.push(ref_id);
}

/// Walk every `VariableDeclarator` node and emit a write reference
/// with `init = true` for each declarator whose `id` slot is itself a
/// `BindingIdentifier` and whose declarator has an `init` expression.
/// Mirrors the `classify_identifier` → `WRITE + init = true` path in
/// the hand-rolled walker for the immediate `VariableDeclarator.id`
/// slot.
///
/// Destructuring patterns (`var [a, b] = ...`, `var { a } = ...`,
/// `var [{ c }] = ...`, …) are deliberately skipped: the hand-rolled
/// walker's `classify_identifier` returns `ClassifyResult::Binding`
/// for every `BindingIdentifier` reached through a pattern step, so
/// no reference row is created for nested binding identifiers — the
/// parity baseline therefore carries no synthetic init write for the
/// pattern's leaf bindings.
fn synthesise_init_references(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
    symbol_to_variable: &IndexVec<SymbolId, Option<VariableId>>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
) {
    let nodes = semantic.nodes();
    for node in nodes.iter() {
        let AstKind::VariableDeclarator(vd) = node.kind() else {
            continue;
        };
        if vd.init.is_none() {
            continue;
        }
        let BindingPattern::BindingIdentifier(binding) = &vd.id else {
            continue;
        };
        let Some(symbol_id) = binding.symbol_id.get() else {
            continue;
        };
        let Some(var_id) = symbol_to_variable[symbol_id] else {
            continue;
        };
        let Some(from) = translation[node.scope_id()] else {
            continue;
        };
        let identifier = AstIdentifier::new(
            AstType::Identifier,
            binding.name.as_str().to_string(),
            binding.span,
        );
        let new_id = references.push(ReferenceData {
            identifier,
            from,
            resolved: Some(var_id),
            init: true,
            flags: ReferenceFlags::WRITE,
        });
        scopes[from].references.push(new_id);
        variables[var_id].references.push(new_id);
    }
}

/// Walk every TypeScript parameter property and emit a Read
/// reference at each binding identifier inside its pattern, resolving
/// to a root-scope implicit global. The hand-rolled walker reaches
/// this slot via `classify_ordinary_reference` because
/// `scope_build_visitor::visit_formal_parameter` omits the
/// `"pattern"` key when `accessibility` / `readonly` / `override` is
/// set; `oxc_semantic` produces no `Reference` at the binding
/// identifier position, so the adapter must synthesise it.
#[allow(clippy::too_many_arguments)]
fn synthesise_parameter_property_references(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    implicit_globals: &mut HashMap<String, VariableId>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    root: ScopeId,
) {
    let nodes = semantic.nodes();
    for node in nodes.iter() {
        let AstKind::FormalParameter(fp) = node.kind() else {
            continue;
        };
        if !is_parameter_property(fp) {
            continue;
        }
        let Some(from) = translation[node.scope_id()] else {
            continue;
        };
        let mut bindings: Vec<&BindingIdentifier<'_>> = Vec::new();
        collect_binding_idents(&fp.pattern, &mut bindings);
        for binding in bindings {
            let identifier = AstIdentifier::new(
                AstType::Identifier,
                binding.name.as_str().to_string(),
                binding.span,
            );
            let var_id = ensure_implicit_global(
                scopes,
                variables,
                definitions,
                implicit_globals,
                root,
                binding.name.as_str(),
                &identifier,
            );
            let new_id = references.push(ReferenceData {
                identifier,
                from,
                resolved: Some(var_id),
                init: false,
                flags: ReferenceFlags::READ,
            });
            scopes[from].references.push(new_id);
            variables[var_id].references.push(new_id);
            push_through_chain(scopes, from, root, new_id);
        }
    }
}

fn is_parameter_property(fp: &FormalParameter<'_>) -> bool {
    fp.accessibility.is_some() || fp.readonly || fp.r#override
}

fn collect_binding_idents<'a, 'b>(
    pattern: &'b BindingPattern<'a>,
    out: &mut Vec<&'b BindingIdentifier<'a>>,
) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => out.push(id),
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_binding_idents(&prop.value, out);
            }
            if let Some(rest) = obj.rest.as_deref() {
                collect_binding_idents(&rest.argument, out);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() {
                collect_binding_idents(el, out);
            }
            if let Some(rest) = arr.rest.as_deref() {
                collect_binding_idents(&rest.argument, out);
            }
        }
        BindingPattern::AssignmentPattern(asn) => {
            collect_binding_idents(&asn.left, out);
        }
    }
}

#[cfg(test)]
#[path = "reference_mapping_test.rs"]
mod reference_mapping_test;
