//! Synthesise the implicit-global Read reference at each binding
//! identifier of a TypeScript parameter property.

use std::collections::HashMap;

use oxc_ast::ast::{BindingIdentifier, BindingPattern, FormalParameter};
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::scope::ScopeId as OxcScopeId;

use unsnarl_ir::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_oxc_parity::AstType;

use super::implicit_global::{ensure_implicit_global, push_through_chain};
use super::reparent_to_switch_case::reparent_to_switch_case;

/// Walk every TypeScript parameter property and emit a Read
/// reference at each binding identifier inside its pattern, resolving
/// to a root-scope implicit global. The parity baseline emits this
/// reference because the binding-identifier slot is treated as a
/// plain reference when `accessibility` / `readonly` / `override` is
/// set on the parameter; `oxc_semantic` produces no `Reference` at
/// the binding identifier position, so the adapter must synthesise
/// it.
#[allow(clippy::too_many_arguments)]
pub(super) fn synthesise_parameter_property_references(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    implicit_globals: &mut HashMap<String, VariableId>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    root: ScopeId,
    switch_cases: &HashMap<ScopeId, Vec<(Span, ScopeId)>>,
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
            let from = reparent_to_switch_case(from, binding.span, scopes, switch_cases);
            let lookup = ensure_implicit_global(
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
                resolved: Some(lookup.var_id),
                init: false,
                flags: ReferenceFlags::READ,
            });
            scopes[from].references.push(new_id);
            variables[lookup.var_id].references.push(new_id);
            if lookup.newly_created {
                push_through_chain(scopes, from, root, new_id);
            }
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
