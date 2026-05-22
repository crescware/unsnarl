//! Declare each formal parameter as a `Parameter` binding inside the
//! function scope.
//!
//! Iterates the typed `FormalParameters.items` and handles
//! `FormalParameters.rest` separately, since oxc keeps the rest
//! parameter off the `items` vector.
//!
//! Every parameter binding shares the same `def_node`: the
//! surrounding function node.

use oxc_ast::ast::FormalParameters;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::DefinitionType;

use crate::declare::collect_binding_identifiers::collect_binding_identifiers;
use crate::state::{declare_variable, ScopeBuilderState};

pub(crate) fn declare_function_params(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    func_node: AstNode,
    params: &FormalParameters<'_>,
) {
    for p in &params.items {
        // TypeScript parameter property (`constructor(public x: number)`
        // etc.): the inner identifier classifies as a plain read
        // reference (resolving as an implicit global), not a binding.
        // Skip declaring the parameter binding when oxc records an
        // accessibility / readonly / override flag.
        if p.accessibility.is_some() || p.readonly || p.r#override {
            continue;
        }
        for ident in collect_binding_identifiers(&p.pattern) {
            declare_variable(
                state,
                scope,
                ident,
                DefinitionType::Parameter,
                func_node.clone(),
                None,
            );
        }
    }
    if let Some(rest) = params.rest.as_deref() {
        for ident in collect_binding_identifiers(&rest.rest.argument) {
            declare_variable(
                state,
                scope,
                ident,
                DefinitionType::Parameter,
                func_node.clone(),
                None,
            );
        }
    }
}

#[cfg(test)]
#[path = "declare_function_params_test.rs"]
mod declare_function_params_test;
