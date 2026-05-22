//! Hoist `var` / `let` / `const` bindings declared in the head of a
//! `for` / `for-in` / `for-of` statement.
//!
//! Takes an explicit `&VariableDeclaration<'_>` since the caller
//! already knows which slot it came from (`ForStatement.init` or
//! `ForInStatement.left` / `ForOfStatement.left`).
//!
//! For `var` we emit the same `var-detected` diagnostic that
//! `handle_variable_declaration` emits and route the binding out to
//! the enclosing function / module / global scope; for `let` /
//! `const` we bind in the supplied `scope` (which is the For scope
//! itself).

use oxc_ast::ast::{VariableDeclaration, VariableDeclarationKind};

use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::span_from_offset;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind as IrVariableDeclarationKind};

use crate::declare::collect_binding_identifiers::collect_binding_identifiers;
use crate::state::{declare_variable_with_extras, DefinitionExtras, ScopeBuilderState};

pub(crate) fn declare_for_left(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    var_decl: &VariableDeclaration<'_>,
    raw: &str,
) {
    let is_target_kind = matches!(
        var_decl.kind,
        VariableDeclarationKind::Var
            | VariableDeclarationKind::Let
            | VariableDeclarationKind::Const,
    );
    if !is_target_kind {
        return;
    }
    let is_var = matches!(var_decl.kind, VariableDeclarationKind::Var);
    if is_var {
        state.diagnostics.add(
            DiagnosticKind::VarDetected,
            "var declaration detected; rendered as node only (no edges).".to_string(),
            span_from_offset(raw, var_decl.span.start as usize),
        );
    }
    let target = if is_var {
        state.arena.scopes[scope].variable_scope
    } else {
        scope
    };
    let var_decl_node = AstNode {
        r#type: AstType::VariableDeclaration,
        span: var_decl.span,
    };
    let declaration_kind = match var_decl.kind {
        VariableDeclarationKind::Var => Some(IrVariableDeclarationKind::Var),
        VariableDeclarationKind::Let => Some(IrVariableDeclarationKind::Let),
        VariableDeclarationKind::Const => Some(IrVariableDeclarationKind::Const),
        _ => None,
    };
    for declarator in &var_decl.declarations {
        let declarator_node = AstNode {
            r#type: AstType::VariableDeclarator,
            span: declarator.span,
        };
        let init = declarator
            .init
            .as_ref()
            .map(crate::materialise::ast_node_of_expression);
        for ident in collect_binding_identifiers(&declarator.id) {
            declare_variable_with_extras(
                state,
                target,
                ident,
                DefinitionType::Variable,
                declarator_node.clone(),
                Some(var_decl_node.clone()),
                DefinitionExtras {
                    init: init.clone(),
                    declaration_kind: declaration_kind.clone(),
                    ..DefinitionExtras::default()
                },
            );
        }
    }
}

#[cfg(test)]
#[path = "declare_for_left_test.rs"]
mod declare_for_left_test;
