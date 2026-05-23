//! Binding-pattern traversal helpers shared with `unsnarl-analyzer`.
//!
//! `unsnarl_analyzer::owner::all_binding_variables` walks a
//! `BindingPattern` to look every leaf binding identifier up in the
//! scope chain. Keeping the pattern walk in the boundary crate keeps
//! the single source of truth for which leaf positions count as
//! bindings (matching what `oxc_semantic_adapter::variable_mapping`
//! collects via `Scoping::iter_bindings_in`).

use oxc_ast::ast::BindingPattern;

use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_oxc_parity::AstType;

/// Collect every leaf `BindingIdentifier` reachable from a
/// `BindingPattern` and return the IR-shape [`AstIdentifier`] view of
/// each occurrence. Object / array / assignment-pattern shapes
/// (including their `...rest` slots) are flattened.
///
/// The traversal mirrors the leaf set
/// [`oxc_semantic::Scoping::iter_bindings_in`] records for a
/// `VariableDeclarator.id` pattern, so feeding each `AstIdentifier`'s
/// name back through [`super::resolve::resolve_in_scope_chain`] yields
/// the same `VariableId`s
/// [`super::oxc_semantic_adapter::variable_mapping`] produces.
pub fn collect_binding_identifiers(pattern: &BindingPattern<'_>) -> Vec<AstIdentifier> {
    let mut out = Vec::new();
    collect(pattern, &mut out);
    out
}

fn collect(pattern: &BindingPattern<'_>, out: &mut Vec<AstIdentifier>) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            out.push(AstIdentifier::new(
                AstType::Identifier,
                id.name.as_str().to_string(),
                id.span,
            ));
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect(&prop.value, out);
            }
            if let Some(rest) = obj.rest.as_deref() {
                collect(&rest.argument, out);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() {
                collect(el, out);
            }
            if let Some(rest) = arr.rest.as_deref() {
                collect(&rest.argument, out);
            }
        }
        BindingPattern::AssignmentPattern(asn) => {
            collect(&asn.left, out);
        }
    }
}
