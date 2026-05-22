//! Hoist each binding inside a `var` / `let` / `const` declaration.
//!
//!
//! Behaviour:
//!
//! 1. Bail out for anything other than `var` / `let` / `const`. The
//!    TS port silently ignores `using` / `await using`; the Rust port
//!    matches that by falling through the `_` arm.
//! 2. For `var`, emit a `var-detected` diagnostic so downstream
//!    consumers can render the offending site as a node-only entry
//!    (no edges) — the same UX choice the TS layer made.
//! 3. The hoisting target depends on the kind: `var` hoists to the
//!    enclosing function / module / global scope
//!    (`scope.variable_scope`), `let` / `const` bind in the lexical
//!    scope itself.
//! 4. Each declarator's `id` (a `BindingPattern`) is flattened into
//!    individual identifiers and each one is declared as
//!    `DefinitionType::Variable` with the declarator as `def_node`
//!    and the surrounding declaration as `parent`.

use oxc_ast::ast::{VariableDeclaration, VariableDeclarationKind};

use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::span_from_offset;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind as IrVariableDeclarationKind};

use crate::declare::collect_binding_identifiers::collect_binding_identifiers;
use crate::materialise::ast_node_of_expression;
use crate::state::{declare_variable_with_extras, DefinitionExtras, ScopeBuilderState};

pub(crate) fn handle_variable_declaration(
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
    let declaration_kind = ir_variable_declaration_kind(var_decl.kind);
    for declarator in &var_decl.declarations {
        let declarator_node = AstNode {
            r#type: AstType::VariableDeclarator,
            span: declarator.span,
        };
        let init = declarator.init.as_ref().map(ast_node_of_expression);
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

fn ir_variable_declaration_kind(
    kind: VariableDeclarationKind,
) -> Option<IrVariableDeclarationKind> {
    match kind {
        VariableDeclarationKind::Var => Some(IrVariableDeclarationKind::Var),
        VariableDeclarationKind::Let => Some(IrVariableDeclarationKind::Let),
        VariableDeclarationKind::Const => Some(IrVariableDeclarationKind::Const),
        // `using` / `await using` are accepted by oxc but not by the
        // serializer; the surrounding caller already rejected them.
        _ => None,
    }
}

#[cfg(test)]
#[path = "handle_variable_declaration_test.rs"]
mod handle_variable_declaration_test;
