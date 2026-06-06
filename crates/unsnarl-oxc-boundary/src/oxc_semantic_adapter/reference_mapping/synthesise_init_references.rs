//! Synthesise the `init = true` write reference at each plain
//! `VariableDeclarator` binding that carries an `init` expression.

use std::collections::HashMap;

use oxc_ast::ast::BindingPattern;
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::scope::ScopeId as OxcScopeId;
use oxc_syntax::symbol::SymbolId;

use unsnarl_ir::ids::{ReferenceId, ScopeId, VariableId};
use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{ScopeData, VariableData};
use unsnarl_oxc_parity::AstType;

use super::reparent_to_switch_case::reparent_to_switch_case;

/// Emit a write reference with `init = true` for each
/// `VariableDeclarator` whose `id` slot is itself a `BindingIdentifier`
/// and whose declarator has an `init` expression.
///
/// Destructuring patterns are deliberately skipped: the parity baseline
/// records no reference row for binding identifiers reached through a
/// pattern step, so no synthetic init write is emitted for the pattern's
/// leaf bindings either.
#[allow(clippy::too_many_arguments)]
pub(super) fn synthesise_init_references(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
    symbol_to_variable: &IndexVec<SymbolId, Option<VariableId>>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
    switch_cases: &HashMap<ScopeId, Vec<(Span, ScopeId)>>,
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
        let from = reparent_to_switch_case(from, binding.span, scopes, switch_cases);
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
