//! Hoist a function declaration into the enclosing scope.
//!
//! Pattern-matches on `Function.id: Option<BindingIdentifier>` —
//! when the id is absent (anonymous `FunctionExpression`), no
//! hoisting happens.

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
    // `declare function f(): void;` is a type-only declaration; no
    // variable should be hoisted for it.
    if matches!(
        function.r#type,
        oxc_ast::ast::FunctionType::TSDeclareFunction
    ) {
        return;
    }
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
