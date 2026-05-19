//! Hoist a class declaration into the enclosing scope.
//!
//! Mirrors `handleClassDeclaration` in
//! `ts/src/boundary/eslint-scope/hoisting/handle-class-declaration.ts`.
//! TS reads `node["id"]` and skips when it is not an identifier; the
//! Rust port pattern-matches on `Class.id: Option<BindingIdentifier>`,
//! which encodes the same skip condition at the type level.

use oxc_ast::ast::Class;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::state::{declare_variable, ScopeBuilderState};

pub(crate) fn handle_class_declaration(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    class: &Class<'_>,
) {
    let Some(id) = class.id.as_ref() else {
        return;
    };
    let identifier = AstIdentifier::new(AstType::Identifier, id.name.as_str().to_string(), id.span);
    let class_node = AstNode {
        r#type: AstType::ClassDeclaration,
        span: class.span,
    };
    declare_variable(
        state,
        scope,
        identifier,
        DefinitionType::ClassName,
        class_node,
        None,
    );
}
