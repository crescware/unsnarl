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

use oxc_ast::ast::{ClassType, FunctionType, ModuleExportName, VariableDeclarationKind};
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::symbol::SymbolId;

use unsnarl_ir::ids::{DefinitionId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::{DefinitionData, VariableData};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind as IrVariableDeclarationKind};

use crate::materialise::ast_node_of_expression;

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
            AstNode {
                r#type: AstType::ImportSpecifier,
                span: s.span,
            },
            Some(module_export_name(&s.imported)),
        )),
        AstKind::ImportDefaultSpecifier(s) => Some(build_import_def(
            nodes,
            identifier,
            node_id,
            AstNode {
                r#type: AstType::ImportDefaultSpecifier,
                span: s.span,
            },
            None,
        )),
        AstKind::ImportNamespaceSpecifier(s) => Some(build_import_def(
            nodes,
            identifier,
            node_id,
            AstNode {
                r#type: AstType::ImportNamespaceSpecifier,
                span: s.span,
            },
            None,
        )),
        _ => None,
    }
}

fn build_variable_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
    vd: &oxc_ast::ast::VariableDeclarator<'_>,
) -> DefinitionData {
    let declarator_node = AstNode {
        r#type: AstType::VariableDeclarator,
        span: vd.span,
    };
    let init = vd.init.as_ref().map(ast_node_of_expression);
    let (parent, declaration_kind) = match nodes.parent_kind(node_id) {
        AstKind::VariableDeclaration(decl) => (
            Some(AstNode {
                r#type: AstType::VariableDeclaration,
                span: decl.span,
            }),
            ir_variable_declaration_kind(decl.kind),
        ),
        _ => (None, None),
    };
    DefinitionData {
        r#type: DefinitionType::Variable,
        name: identifier,
        node: declarator_node,
        parent,
        init,
        declaration_kind,
        import_source: None,
        imported_name: None,
    }
}

fn build_function_name_def(
    identifier: AstIdentifier,
    f: &oxc_ast::ast::Function<'_>,
) -> DefinitionData {
    DefinitionData {
        r#type: DefinitionType::FunctionName,
        name: identifier,
        node: AstNode {
            r#type: function_ast_type(f),
            span: f.span,
        },
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    }
}

fn build_class_name_def(identifier: AstIdentifier, c: &oxc_ast::ast::Class<'_>) -> DefinitionData {
    let ty = match c.r#type {
        ClassType::ClassDeclaration => AstType::ClassDeclaration,
        ClassType::ClassExpression => AstType::ClassExpression,
    };
    DefinitionData {
        r#type: DefinitionType::ClassName,
        name: identifier,
        node: AstNode {
            r#type: ty,
            span: c.span,
        },
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    }
}

fn build_parameter_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
) -> Option<DefinitionData> {
    let owner = enclosing_function_node(nodes, node_id)?;
    Some(DefinitionData {
        r#type: DefinitionType::Parameter,
        name: identifier,
        node: owner,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    })
}

fn build_catch_clause_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
) -> Option<DefinitionData> {
    let owner = enclosing_catch_clause_node(nodes, node_id)?;
    Some(DefinitionData {
        r#type: DefinitionType::CatchClause,
        name: identifier,
        node: owner,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    })
}

fn build_import_def(
    nodes: &oxc_semantic::AstNodes<'_>,
    identifier: AstIdentifier,
    node_id: oxc_syntax::node::NodeId,
    spec_node: AstNode,
    imported_name: Option<String>,
) -> DefinitionData {
    let (parent, import_source) = match nodes.parent_kind(node_id) {
        AstKind::ImportDeclaration(decl) => (
            Some(AstNode {
                r#type: AstType::ImportDeclaration,
                span: decl.span,
            }),
            Some(decl.source.value.as_str().to_string()),
        ),
        _ => (None, None),
    };
    DefinitionData {
        r#type: DefinitionType::ImportBinding,
        name: identifier,
        node: spec_node,
        parent,
        init: None,
        declaration_kind: None,
        import_source,
        imported_name,
    }
}

fn enclosing_function_node(
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_syntax::node::NodeId,
) -> Option<AstNode> {
    for ancestor in nodes.ancestor_kinds(node_id) {
        match ancestor {
            AstKind::Function(f) => {
                return Some(AstNode {
                    r#type: function_ast_type(f),
                    span: f.span,
                });
            }
            AstKind::ArrowFunctionExpression(arrow) => {
                return Some(AstNode {
                    r#type: AstType::ArrowFunctionExpression,
                    span: arrow.span,
                });
            }
            _ => {}
        }
    }
    None
}

fn enclosing_catch_clause_node(
    nodes: &oxc_semantic::AstNodes<'_>,
    node_id: oxc_syntax::node::NodeId,
) -> Option<AstNode> {
    for ancestor in nodes.ancestor_kinds(node_id) {
        if let AstKind::CatchClause(c) = ancestor {
            return Some(AstNode {
                r#type: AstType::CatchClause,
                span: c.span,
            });
        }
    }
    None
}

fn function_ast_type(f: &oxc_ast::ast::Function<'_>) -> AstType {
    match f.r#type {
        FunctionType::FunctionExpression | FunctionType::TSEmptyBodyFunctionExpression => {
            AstType::FunctionExpression
        }
        FunctionType::FunctionDeclaration | FunctionType::TSDeclareFunction => {
            AstType::FunctionDeclaration
        }
    }
}

fn ir_variable_declaration_kind(
    kind: VariableDeclarationKind,
) -> Option<IrVariableDeclarationKind> {
    match kind {
        VariableDeclarationKind::Var => Some(IrVariableDeclarationKind::Var),
        VariableDeclarationKind::Let => Some(IrVariableDeclarationKind::Let),
        VariableDeclarationKind::Const => Some(IrVariableDeclarationKind::Const),
        VariableDeclarationKind::Using | VariableDeclarationKind::AwaitUsing => None,
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
