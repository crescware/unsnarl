//! Declaration sites → `IrArena.definitions`.
//!
//! Walks every `(SymbolId, NodeId)` declaration pair recovered from
//! `Scoping::symbol_declaration` (plus `Scoping::symbol_redeclarations`
//! for `var x; var x;`-style redeclarations) and emits one
//! [`unsnarl_ir::scope::DefinitionData`] per declaration site,
//! cross-linked onto the corresponding [`unsnarl_ir::scope::VariableData::defs`].
//!
//! `oxc_semantic`'s `symbol_declaration` returns the AST node that
//! *contains* a `BindingIdentifier` / `BindingPattern` (`VariableDeclarator`,
//! `Function`, `Class`, `FormalParameter`, `CatchParameter`,
//! `ImportSpecifier`, `ImportDefaultSpecifier`, `ImportNamespaceSpecifier`),
//! not the identifier node itself. This module classifies each
//! anchor into one of six `DefinitionType` variants (the seventh,
//! `ImplicitGlobalVariable`, is emitted by `reference_mapping`) and
//! reconstructs the four extras the serializer reads
//! (`init` / `declaration_kind` / `import_source` / `imported_name`)
//! by walking the AST around the anchor:
//!
//! * `Variable` (kind = `VariableDeclarator`): parent is the
//!   enclosing `VariableDeclaration`, `init` is materialised from
//!   `declarator.init`, `declaration_kind` is `declaration.kind`.
//! * `FunctionName` (kind = `Function`): no parent; `def_node` uses
//!   `FunctionDeclaration` / `FunctionExpression` per `Function::r#type`.
//! * `ClassName` (kind = `Class`): no parent; `def_node` uses
//!   `ClassDeclaration` / `ClassExpression` per `Class::r#type`.
//! * `Parameter` (kind = `FormalParameter` or `FormalParameterRest`):
//!   walks ancestors to the surrounding `Function` /
//!   `ArrowFunctionExpression` and uses that as `def_node`.
//! * `CatchClause` (kind = `CatchParameter`): walks ancestors to the
//!   surrounding `CatchClause` and uses that as `def_node`.
//! * `ImportBinding` (kind = `ImportSpecifier` /
//!   `ImportDefaultSpecifier` / `ImportNamespaceSpecifier`): parent
//!   is the enclosing `ImportDeclaration`, `import_source` is its
//!   `source.value`, `imported_name` is the original imported symbol
//!   name for `ImportSpecifier` only.
//!
//! `name.span` for each `DefinitionData` is the identifier's own
//! span (`Scoping::symbol_span` for the original declaration, or each
//! redeclaration's `Redeclaration::span` for subsequent sites) — not
//! the anchor's span. Constructed directly from the parsed
//! `BindingIdentifier`.
//!
//! ## Bindings that produce no `DefinitionData` here
//!
//! Some symbols are dropped entirely by `variable_mapping` (TS
//! parameter properties, named function-expression self-names,
//! type-only declarations) and therefore have no `VariableData` to
//! cross-link a def into. The inner-class-self-name and
//! implicit-`arguments` synthetic variables get their defs
//! synthesised in `variable_mapping` directly. This pass only emits
//! definitions for the symbols that survive into the IR via the
//! main `iter_bindings_in` walk.

use oxc_ast::ast::ModuleExportName;
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::symbol::SymbolId;

use unsnarl_ir::ids::{DefinitionId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::{DefinitionData, VariableData};
use unsnarl_oxc_parity::AstType;

mod build_catch_clause_def;
mod build_class_name_def;
mod build_function_name_def;
mod build_import_def;
mod build_parameter_def;
mod build_variable_def;
mod function_ast_type;

use build_catch_clause_def::build_catch_clause_def;
use build_class_name_def::build_class_name_def;
use build_function_name_def::build_function_name_def;
use build_import_def::build_import_def;
use build_parameter_def::build_parameter_def;
use build_variable_def::build_variable_def;

/// Walk every symbol's declaration sites and emit one
/// `DefinitionData` per site. Cross-links the new def ids onto the
/// matching `VariableData::defs` via `symbol_to_variable`.
pub(crate) fn build_definitions(
    semantic: &Semantic<'_>,
    variables: &mut IndexVec<VariableId, VariableData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    symbol_to_variable: &IndexVec<SymbolId, Option<VariableId>>,
) {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    for sid in scoping.symbol_ids() {
        let Some(var_id) = symbol_to_variable[sid] else {
            continue;
        };
        let name = scoping.symbol_name(sid).to_string();
        let redecls = scoping.symbol_redeclarations(sid);
        if redecls.is_empty() {
            let span = scoping.symbol_span(sid);
            let node_id = scoping.symbol_declaration(sid);
            if let Some(def) = build_definition(nodes, &name, span, node_id) {
                let def_id = definitions.push(def);
                variables[var_id].defs.push(def_id);
            }
        } else {
            for r in redecls {
                if let Some(def) = build_definition(nodes, &name, r.span, r.declaration) {
                    let def_id = definitions.push(def);
                    variables[var_id].defs.push(def_id);
                }
            }
        }
    }
}

fn build_definition(
    nodes: &oxc_semantic::AstNodes<'_>,
    name: &str,
    ident_span: Span,
    node_id: oxc_syntax::node::NodeId,
) -> Option<DefinitionData> {
    let identifier = AstIdentifier::new(AstType::Identifier, name.to_string(), ident_span);
    let kind = nodes.kind(node_id);
    match kind {
        AstKind::VariableDeclarator(vd) => Some(build_variable_def(nodes, identifier, node_id, vd)),
        AstKind::Function(f) => Some(build_function_name_def(identifier, f)),
        AstKind::Class(c) => Some(build_class_name_def(identifier, c)),
        AstKind::FormalParameter(_) | AstKind::FormalParameterRest(_) => {
            build_parameter_def(nodes, identifier, node_id)
        }
        AstKind::CatchParameter(_) => build_catch_clause_def(nodes, identifier, node_id),
        AstKind::ImportSpecifier(s) => Some(build_import_def(
            nodes,
            identifier,
            node_id,
            AstNode::new(AstType::ImportSpecifier, s.span),
            Some(module_export_name(&s.imported)),
        )),
        AstKind::ImportDefaultSpecifier(s) => Some(build_import_def(
            nodes,
            identifier,
            node_id,
            AstNode::new(AstType::ImportDefaultSpecifier, s.span),
            None,
        )),
        AstKind::ImportNamespaceSpecifier(s) => Some(build_import_def(
            nodes,
            identifier,
            node_id,
            AstNode::new(AstType::ImportNamespaceSpecifier, s.span),
            None,
        )),
        _ => None,
    }
}

fn module_export_name(name: &ModuleExportName<'_>) -> String {
    match name {
        ModuleExportName::IdentifierName(n) => n.name.as_str().to_string(),
        ModuleExportName::IdentifierReference(n) => n.name.as_str().to_string(),
        ModuleExportName::StringLiteral(n) => n.value.as_str().to_string(),
    }
}

#[cfg(test)]
#[path = "definition_mapping_test.rs"]
mod definition_mapping_test;
