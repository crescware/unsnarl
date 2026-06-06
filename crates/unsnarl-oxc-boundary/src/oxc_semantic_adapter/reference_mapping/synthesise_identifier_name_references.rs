//! Synthesise implicit-global Read references at `IdentifierName` /
//! JSX-tag positions that the parity baseline treats as references but
//! for which `oxc_semantic` emits no `Reference` row.

use std::collections::{HashMap, HashSet};

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

/// Synthesise implicit-global Read references at identifier positions
/// that the parity baseline treats as references but for which
/// `oxc_semantic` does not emit `Reference` rows.
///
/// `oxc_semantic` does not emit `Reference` rows for `IdentifierName`
/// nodes, nor for `JSXIdentifier` nodes at JSX-tag positions whose
/// name is a lowercase intrinsic. The parity baseline does, treating
/// both shapes as implicit-global Reads with the appropriate
/// `AstType` on the resulting `Reference` / `ImplicitGlobalVariable`
/// rows. Walk every relevant AST node and synthesise the matching
/// references so the IR carries the same `scope#0:new@<offset>` /
/// `scope#0:target@<offset>` / `scope#0:span@<offset>` implicit-global
/// variables the parity baseline shows.
///
/// Currently covered:
///
/// * [`AstKind::MetaProperty`] — both `meta` and `property`
///   `IdentifierName` slots (`new.target`, `import.meta`).
/// * [`AstKind::ImportAttribute`] when the key is an `Identifier`
///   variant (the `type` in `import x from "y" with { type: "json" }`).
/// * [`AstKind::JSXIdentifier`] — every JSX-tag / attribute / member
///   identifier whose span isn't already covered by an existing
///   reference from the resolved- or unresolved-loop passes.
#[allow(clippy::too_many_arguments)]
pub(super) fn synthesise_identifier_name_references(
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
    use oxc_ast::ast::{ImportAttributeKey, ModuleExportName};

    let existing_spans: HashSet<(u32, u32)> = references
        .iter()
        .map(|r| (r.identifier.span.start, r.identifier.span.end))
        .collect();
    let nodes = semantic.nodes();
    let mut sites: Vec<(ScopeId, &str, Span, AstType)> = Vec::new();
    for node in nodes.iter() {
        let Some(from) = translation[node.scope_id()] else {
            continue;
        };
        match node.kind() {
            AstKind::MetaProperty(mp) => {
                sites.push((
                    from,
                    mp.meta.name.as_str(),
                    mp.meta.span,
                    AstType::Identifier,
                ));
                sites.push((
                    from,
                    mp.property.name.as_str(),
                    mp.property.span,
                    AstType::Identifier,
                ));
            }
            AstKind::ImportAttribute(ia) => {
                if let ImportAttributeKey::Identifier(id) = &ia.key {
                    sites.push((from, id.name.as_str(), id.span, AstType::Identifier));
                }
            }
            AstKind::ExportSpecifier(es) => {
                if let ModuleExportName::IdentifierName(id) = &es.local {
                    sites.push((from, id.name.as_str(), id.span, AstType::Identifier));
                }
            }
            AstKind::ExportAllDeclaration(ead) => {
                if let Some(ModuleExportName::IdentifierName(id)) = &ead.exported {
                    sites.push((from, id.name.as_str(), id.span, AstType::Identifier));
                }
            }
            AstKind::JSXIdentifier(id) => {
                if jsx_identifier_is_skip_slot(nodes, node.id(), id.span) {
                    continue;
                }
                sites.push((from, id.name.as_str(), id.span, AstType::JSXIdentifier));
            }
            _ => {}
        }
    }
    // Drop any site whose span already has a Reference row from the
    // resolved- or unresolved-loop passes (an `<MyComp/>` tag where
    // `MyComp` is in scope was already handled there).
    sites.retain(|(_, _, span, _)| !existing_spans.contains(&(span.start, span.end)));
    // Emit in source order so first-occurrence implicit-global
    // synthesis matches the parity baseline.
    sites.sort_by_key(|(_, _, span, _)| span.start);
    for (from, name, span, ast_type) in sites {
        let identifier = AstIdentifier::new(ast_type, name.to_string(), span);
        let from = reparent_to_switch_case(from, span, scopes, switch_cases);
        let lookup = ensure_implicit_global(
            scopes,
            variables,
            definitions,
            implicit_globals,
            root,
            name,
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

/// Decide whether a `JSXIdentifier` node sits in a purely structural
/// slot that should not produce a reference row.
///
/// Skip slots:
///
/// * `JSXAttribute.name` when the name is a `JSXIdentifier` directly.
/// * `JSXMemberExpression.property` (the `.b` in `<a.b />`).
/// * Anything beneath a `JSXClosingElement` (the closing-tag identifier
///   would otherwise duplicate the opening tag's reference).
fn jsx_identifier_is_skip_slot(
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_semantic::NodeId,
    span: Span,
) -> bool {
    let parent_kind = nodes.parent_kind(node_id);
    match parent_kind {
        AstKind::JSXAttribute(attr) => {
            // JSXAttribute.name can be JSXIdentifier or JSXNamespacedName;
            // skip only when this JSXIdentifier IS the JSXAttribute's
            // direct `.name` slot (i.e. spans match).
            use oxc_ast::ast::JSXAttributeName;
            matches!(&attr.name, JSXAttributeName::Identifier(id) if id.span == span)
        }
        AstKind::JSXMemberExpression(mp) => mp.property.span == span,
        AstKind::JSXClosingElement(_) => true,
        _ => false,
    }
}
