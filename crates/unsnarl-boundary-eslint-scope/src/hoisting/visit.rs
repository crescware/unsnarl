//! Per-statement dispatcher for the hoist pass.
//!
//! Pattern-matches on the typed `Statement` enum and the typed
//! `Declaration` enum.
//!
//! [`visit_statement`] handles the top-level `Statement` form,
//! [`visit_declaration`] handles `ExportNamedDeclaration.declaration`
//! (`Declaration`) and the
//! `ExportDefaultDeclaration.declaration` (`ExportDefaultDeclarationKind`)
//! body. Each helper dispatches to the same per-shape
//! `handle_*_declaration`.

use oxc_ast::ast::{Declaration, ExportDefaultDeclarationKind, Statement};

use unsnarl_ir::ids::ScopeId;

use crate::hoisting::handle_class_declaration::handle_class_declaration;
use crate::hoisting::handle_function_declaration::handle_function_declaration;
use crate::hoisting::handle_import_declaration::handle_import_declaration;
use crate::hoisting::handle_variable_declaration::handle_variable_declaration;
use crate::state::ScopeBuilderState;

pub(crate) fn visit_statement(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    stmt: &Statement<'_>,
    raw: &str,
) {
    match stmt {
        Statement::VariableDeclaration(v) => handle_variable_declaration(state, scope, v, raw),
        Statement::FunctionDeclaration(f) => handle_function_declaration(state, scope, f),
        Statement::ClassDeclaration(c) => handle_class_declaration(state, scope, c),
        Statement::ImportDeclaration(i) => handle_import_declaration(state, scope, i),
        Statement::ExportNamedDeclaration(e) => {
            if let Some(decl) = e.declaration.as_ref() {
                visit_declaration(state, scope, decl, raw);
            }
        }
        Statement::ExportDefaultDeclaration(e) => match &e.declaration {
            ExportDefaultDeclarationKind::FunctionDeclaration(f) => {
                handle_function_declaration(state, scope, f);
            }
            ExportDefaultDeclarationKind::ClassDeclaration(c) => {
                handle_class_declaration(state, scope, c);
            }
            _ => {}
        },
        _ => {}
    }
}

fn visit_declaration(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    decl: &Declaration<'_>,
    raw: &str,
) {
    match decl {
        Declaration::VariableDeclaration(v) => handle_variable_declaration(state, scope, v, raw),
        Declaration::FunctionDeclaration(f) => handle_function_declaration(state, scope, f),
        Declaration::ClassDeclaration(c) => handle_class_declaration(state, scope, c),
        _ => {}
    }
}

#[cfg(test)]
#[path = "visit_test.rs"]
mod visit_test;
