//! Push a `Class` scope and declare the inner class name.
//!
//! The TS port reads
//! `node["id"]` defensively because `NodeLike` does not pin the
//! shape; the Rust port matches on `Class.id: Option<BindingIdentifier>`
//! directly.
//!
//! The TS comment on the original notes that the *outer* class name
//! for `ClassDeclaration` is already added by the hoisting pass
//! (`hoisting/handle_class_declaration.rs`) before the walker
//! reaches the class node. This helper only declares the *inner*
//! `ClassName` binding that lives inside the new class scope.

use oxc_ast::ast::{Class, ClassType};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::state::{declare_variable, push_scope, ScopeBuilderState};

pub(crate) fn enter_class(state: &mut ScopeBuilderState, class: &Class<'_>) -> ScopeId {
    let class_ast_type = match class.r#type {
        ClassType::ClassDeclaration => AstType::ClassDeclaration,
        ClassType::ClassExpression => AstType::ClassExpression,
    };
    let class_node = AstNode {
        r#type: class_ast_type,
        span: class.span,
    };
    let scope_id = push_scope(state, ScopeType::Class, class_node.clone());
    if let Some(id) = class.id.as_ref() {
        let identifier =
            AstIdentifier::new(AstType::Identifier, id.name.as_str().to_string(), id.span);
        declare_variable(
            state,
            scope_id,
            identifier,
            DefinitionType::ClassName,
            class_node,
            None,
        );
    }
    scope_id
}

#[cfg(test)]
#[path = "enter_class_test.rs"]
mod enter_class_test;
