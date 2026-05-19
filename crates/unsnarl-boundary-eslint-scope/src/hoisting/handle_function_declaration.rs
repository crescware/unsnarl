//! Hoist a function declaration into the enclosing scope.
//!
//! Mirrors `handleFunctionDeclaration` in
//! `ts/src/boundary/eslint-scope/hoisting/handle-function-declaration.ts`.
//! TS reads `node["id"]` and skips when it is not an identifier; the
//! Rust port matches on `Function.id: Option<BindingIdentifier>`,
//! which encodes the same skip condition at the type level.

use oxc_ast::ast::Function;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::state::{declare_variable, ScopeBuilderState};

pub(crate) fn handle_function_declaration(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    function: &Function<'_>,
) {
    let Some(id) = function.id.as_ref() else {
        return;
    };
    let identifier = AstIdentifier::new(AstType::Identifier, id.name.as_str().to_string(), id.span);
    let function_node = AstNode {
        r#type: AstType::FunctionDeclaration,
        span: function.span,
    };
    declare_variable(
        state,
        scope,
        identifier,
        DefinitionType::FunctionName,
        function_node,
        None,
    );
}

#[cfg(test)]
#[path = "handle_function_declaration_test.rs"]
mod handle_function_declaration_test;
