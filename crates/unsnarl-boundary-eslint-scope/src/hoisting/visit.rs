//! Per-statement dispatcher for the hoist pass.
//!
//! Mirrors `visit` in `ts/src/boundary/eslint-scope/hoisting/visit.ts`.
//! TS switches on a `NodeLike.type` string; the Rust port pattern-matches
//! on the typed `Statement` enum and the typed `Declaration` enum.
//!
//! TS recurses through `ExportNamedDeclaration.declaration` and
//! `ExportDefaultDeclaration.declaration` with the same `visit`
//! function. Rust splits that into [`visit_statement`] and
//! [`visit_declaration`] because oxc_ast types each as a distinct
//! enum: `Statement` for the top-level form, `Declaration` for the
//! body of `ExportNamedDeclaration`, and `ExportDefaultDeclarationKind`
//! for `ExportDefaultDeclaration`. Each helper dispatches to the same
//! per-shape `handle_*_declaration`, so the externally observable
//! behaviour is identical.

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
