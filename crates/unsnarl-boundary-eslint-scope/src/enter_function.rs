//! Push a `Function` scope and seed its bindings.
//!
//! Mirrors `enterFunction` in
//! `ts/src/boundary/eslint-scope/enter-function.ts`. TS handles
//! `FunctionDeclaration` / `FunctionExpression` /
//! `ArrowFunctionExpression` through one entry; in Rust they split
//! between `oxc_ast::ast::Function` (the first two) and
//! `oxc_ast::ast::ArrowFunctionExpression`, so this module exposes
//! two helpers (`enter_function` / `enter_arrow_function_expression`)
//! that share the same body apart from `declare_implicit_arguments`,
//! which arrows must not call.

use oxc_ast::ast::{ArrowFunctionExpression, Function, FunctionType};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::declare::declare_implicit_arguments::declare_implicit_arguments;
use crate::declare_function_params::declare_function_params;
use crate::hoisting::hoist_declarations::hoist_declarations;
use crate::state::{push_scope, ScopeBuilderState};

pub(crate) fn enter_function(
    state: &mut ScopeBuilderState,
    func: &Function<'_>,
    raw: &str,
) -> ScopeId {
    let func_ast_type = match func.r#type {
        FunctionType::FunctionExpression | FunctionType::TSEmptyBodyFunctionExpression => {
            AstType::FunctionExpression
        }
        FunctionType::FunctionDeclaration | FunctionType::TSDeclareFunction => {
            AstType::FunctionDeclaration
        }
    };
    let func_node = AstNode {
        r#type: func_ast_type,
        span: func.span,
    };
    let scope_id = push_scope(state, ScopeType::Function, func_node.clone());
    declare_implicit_arguments(state, scope_id);
    declare_function_params(state, scope_id, func_node, &func.params);
    if let Some(body) = func.body.as_deref() {
        hoist_declarations(state, &body.statements, scope_id, raw);
    }
    scope_id
}

pub(crate) fn enter_arrow_function_expression(
    state: &mut ScopeBuilderState,
    arrow: &ArrowFunctionExpression<'_>,
    raw: &str,
) -> ScopeId {
    let arrow_node = AstNode {
        r#type: AstType::ArrowFunctionExpression,
        span: arrow.span,
    };
    let scope_id = push_scope(state, ScopeType::Function, arrow_node.clone());
    declare_function_params(state, scope_id, arrow_node, &arrow.params);
    hoist_declarations(state, &arrow.body.statements, scope_id, raw);
    scope_id
}
