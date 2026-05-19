//! Hoist `var` / `let` / `const` bindings declared in the head of a
//! `for` / `for-in` / `for-of` statement.
//!
//! Mirrors `declareForLeft` in
//! `ts/src/boundary/eslint-scope/declare-for-left.ts`. The TS port
//! pulls `node["init"]` and `node["left"]`; the Rust port takes an
//! explicit `&VariableDeclaration<'_>` since the caller already knows
//! which slot it came from (`ForStatement.init` or
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
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::declare::collect_binding_identifiers::collect_binding_identifiers;
use crate::span_util::span_from_offset;
use crate::state::{declare_variable, ScopeBuilderState};

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
    for declarator in &var_decl.declarations {
        let declarator_node = AstNode {
            r#type: AstType::VariableDeclarator,
            span: declarator.span,
        };
        for ident in collect_binding_identifiers(&declarator.id) {
            declare_variable(
                state,
                target,
                ident,
                DefinitionType::Variable,
                declarator_node.clone(),
                Some(var_decl_node.clone()),
            );
        }
    }
}
