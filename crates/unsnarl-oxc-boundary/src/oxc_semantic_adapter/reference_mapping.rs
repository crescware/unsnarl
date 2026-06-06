//! `oxc_semantic::Scoping` references → `IrArena.references`.
//!
//! Walks every reference materialised by `SemanticBuilder` —
//! `Scoping::symbol_ids()` ⇒ `get_resolved_reference_ids(sid)` for
//! the resolved set, plus `Scoping::root_unresolved_references()`
//! for the unresolved set — and emits one [`unsnarl_ir::ReferenceData`]
//! per reference. The pass also populates these cross-link sites:
//!
//! * `ScopeData::references` — appended on every reference at its
//!   creating scope.
//! * `VariableData::references` — appended when a reference resolves
//!   (to a real binding, an adapter-synthesised `arguments`, or an
//!   implicit-global Variable).
//! * `ScopeData::through` — only along the implicit-global path. The
//!   walk runs from the reference's scope up to and *including* the
//!   global scope, pushing the reference id on each scope's `through`.
//!
//! ## `arguments` synthesis
//!
//! `oxc_semantic` does not bind `arguments`, so an `arguments`
//! identifier inside a function body surfaces as a root-unresolved
//! reference. `variable_mapping` has already inserted a synthetic
//! `arguments` `VariableData` into every non-arrow function scope;
//! this pass walks the scope chain from the reference's scope upward,
//! and if the first ancestor containing an `arguments` binding is one
//! of those synthetic rows, resolves the reference to it. References
//! to `arguments` outside any function (e.g. module-level) fall
//! through to the implicit-global path.
//!
//! ## Implicit globals
//!
//! Unresolved references that don't match the `arguments` case land
//! on a per-name `ImplicitGlobalVariable` on the root scope. The
//! first occurrence creates the `VariableData` (with one identifier
//! entry recording the reference's identifier span) plus the
//! `DefinitionData` row; subsequent occurrences for the same name
//! reuse the same `VariableId`.
//!
//! ## `init` flag
//!
//! `oxc_semantic` emits no reference for the binding side of `var x = 0`
//! — the declaration carries the init directly — so a post-pass
//! synthesises a write reference with `init = true` at each plain
//! `VariableDeclarator` binding that has an `init` expression (see
//! `synthesise_init_references`), and stamps the read sitting at the
//! init-expression position (see `mark_variable_declarator_init_reads`).

use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::{GetSpan, Span};
use oxc_syntax::scope::ScopeId as OxcScopeId;
use oxc_syntax::symbol::SymbolId;

use std::collections::{HashMap, HashSet};

use unsnarl_ir::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};

mod build_identifier;
mod convert_flags;
mod implicit_global;
mod mark_variable_declarator_init_reads;
mod rebind_inner_class_name_references;
mod reference_is_skip_slot;
mod reparent_decorator_references;
mod reparent_to_switch_case;
mod resolve_synthetic_arguments;
mod sort_reference_lists_by_source_order;
mod synthesise_identifier_name_references;
mod synthesise_init_references;
mod synthesise_parameter_property_references;

use build_identifier::build_identifier;
use convert_flags::{adjust_flags_for_parent, convert_flags};
use implicit_global::{ensure_implicit_global, push_through_chain};
use mark_variable_declarator_init_reads::mark_variable_declarator_init_reads;
use rebind_inner_class_name_references::rebind_inner_class_name_references;
use reference_is_skip_slot::reference_is_skip_slot;
use reparent_decorator_references::reparent_decorator_references;
use reparent_to_switch_case::reparent_to_switch_case;
use resolve_synthetic_arguments::resolve_synthetic_arguments;
use sort_reference_lists_by_source_order::sort_reference_lists_by_source_order;
use synthesise_identifier_name_references::synthesise_identifier_name_references;
use synthesise_init_references::synthesise_init_references;
use synthesise_parameter_property_references::synthesise_parameter_property_references;

/// Walk `semantic.scoping()`'s reference table and produce the
/// `unsnarl_ir` arena's `references` rows, populating cross-links on
/// `scopes` / `variables` / `definitions` along the way.
///
/// `symbol_to_variable` is the `SymbolId → VariableId` projection
/// produced by [`super::variable_mapping::build_variables`]; it lets
/// this pass translate `oxc_semantic`'s resolved-reference symbol
/// references into the matching `VariableId` without re-walking the
/// scope tree.
#[allow(clippy::too_many_arguments)]
pub(crate) fn build_references(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    symbol_to_variable: &IndexVec<SymbolId, Option<VariableId>>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    synthetic_unresolved: &HashSet<SymbolId>,
    switch_cases: &HashMap<ScopeId, Vec<(Span, ScopeId)>>,
    inner_class_names: &[super::variable_mapping::InnerClassName],
) -> IndexVec<ReferenceId, ReferenceData> {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    let mut references: IndexVec<ReferenceId, ReferenceData> = IndexVec::new();
    let root = ScopeId::from_usize(0);
    let mut implicit_globals: HashMap<String, VariableId> = HashMap::new();

    {
        let _span = unsnarl_instrumentation::span!("refs_resolved_loop");
        for sid in scoping.symbol_ids() {
            if let Some(var_id) = symbol_to_variable[sid] {
                for &oxc_ref_id in scoping.get_resolved_reference_ids(sid) {
                    let oxc_ref = scoping.get_reference(oxc_ref_id);
                    if oxc_ref.flags().is_type() {
                        continue;
                    }
                    if reference_is_skip_slot(nodes, oxc_ref.node_id()) {
                        continue;
                    }
                    let Some(from) = translation[oxc_ref.scope_id()] else {
                        continue;
                    };
                    let identifier = build_identifier(nodes, oxc_ref.node_id());
                    let from = reparent_to_switch_case(from, identifier.span, scopes, switch_cases);
                    let flags = adjust_flags_for_parent(
                        convert_flags(oxc_ref.flags()),
                        nodes,
                        oxc_ref.node_id(),
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
                }
                continue;
            }
            if synthetic_unresolved.contains(&sid) {
                // The adapter does not allocate a `VariableData` for a
                // named function-expression self-name (see
                // `variable_mapping`'s module header). References
                // `oxc_semantic` resolved against this symbol must be
                // re-emitted through the implicit-global path so they end
                // up matching the parity baseline (an unresolved read
                // resolving to a root-scope implicit global).
                let name = scoping.symbol_name(sid).to_string();
                for &oxc_ref_id in scoping.get_resolved_reference_ids(sid) {
                    let oxc_ref = scoping.get_reference(oxc_ref_id);
                    if oxc_ref.flags().is_type() {
                        continue;
                    }
                    if reference_is_skip_slot(nodes, oxc_ref.node_id()) {
                        continue;
                    }
                    let Some(from) = translation[oxc_ref.scope_id()] else {
                        continue;
                    };
                    let identifier = build_identifier(nodes, oxc_ref.node_id());
                    let from = reparent_to_switch_case(from, identifier.span, scopes, switch_cases);
                    let flags = convert_flags(oxc_ref.flags());
                    let lookup = ensure_implicit_global(
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
                        resolved: Some(lookup.var_id),
                        init: false,
                        flags,
                    });
                    scopes[from].references.push(new_id);
                    variables[lookup.var_id].references.push(new_id);
                    if lookup.newly_created {
                        push_through_chain(scopes, from, root, new_id);
                    }
                }
                continue;
            }
            // Otherwise this symbol lives in a filtered (TypeScript
            // type-only) scope; its references aren't part of the runtime
            // IR either.
        }
    }

    {
        let _span = unsnarl_instrumentation::span!("synth_init");
        synthesise_init_references(
            semantic,
            scopes,
            variables,
            &mut references,
            symbol_to_variable,
            translation,
            switch_cases,
        );
    }

    {
        let _span = unsnarl_instrumentation::span!("refs_unresolved_loop");
        // `Scoping::root_unresolved_references` is keyed on a
        // `hashbrown::HashMap`, so its iteration order is arbitrary. The
        // parity baseline visits identifiers in source order, so implicit
        // globals appear in source order too. Sort here by the first
        // reference's identifier span so the IR `variables` /
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
                if oxc_ref.flags().is_type() {
                    continue;
                }
                if reference_is_skip_slot(nodes, oxc_ref.node_id()) {
                    continue;
                }
                let Some(from) = translation[oxc_ref.scope_id()] else {
                    continue;
                };
                let identifier = build_identifier(nodes, oxc_ref.node_id());
                let from = reparent_to_switch_case(from, identifier.span, scopes, switch_cases);
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
                    let lookup = ensure_implicit_global(
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
                        resolved: Some(lookup.var_id),
                        init: false,
                        flags,
                    });
                    scopes[from].references.push(new_id);
                    variables[lookup.var_id].references.push(new_id);
                    if lookup.newly_created {
                        push_through_chain(scopes, from, root, new_id);
                    }
                }
            }
        }
    }

    {
        let _span = unsnarl_instrumentation::span!("synth_param_props");
        synthesise_parameter_property_references(
            semantic,
            scopes,
            variables,
            &mut references,
            definitions,
            &mut implicit_globals,
            translation,
            root,
            switch_cases,
        );
    }

    {
        let _span = unsnarl_instrumentation::span!("synth_identifier_names");
        synthesise_identifier_name_references(
            semantic,
            scopes,
            variables,
            &mut references,
            definitions,
            &mut implicit_globals,
            translation,
            root,
            switch_cases,
        );
    }

    {
        let _span = unsnarl_instrumentation::span!("mark_init_reads");
        mark_variable_declarator_init_reads(semantic, &mut references);
    }

    {
        let _span = unsnarl_instrumentation::span!("reparent_decorators");
        reparent_decorator_references(semantic, scopes, variables, &mut references, translation);
    }

    {
        let _span = unsnarl_instrumentation::span!("rebind_inner_class_names");
        rebind_inner_class_name_references(scopes, variables, &mut references, inner_class_names);
    }

    {
        let _span = unsnarl_instrumentation::span!("sort_ref_lists");
        sort_reference_lists_by_source_order(scopes, variables, &references);
    }

    references
}

#[cfg(test)]
#[path = "reference_mapping_test.rs"]
mod reference_mapping_test;
