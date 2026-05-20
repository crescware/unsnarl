//! Declare each formal parameter as a `Parameter` binding inside the
//! function scope.
//!
//! Mirrors `declareFunctionParams` in
//! `ts/src/boundary/eslint-scope/declare-function-params.ts`. The TS
//! port iterates `node["params"]` and unwraps `RestElement` via
//! `p["argument"]`; the Rust port iterates the typed
//! `FormalParameters.items` and handles `FormalParameters.rest`
//! separately, since oxc keeps the rest parameter off the `items`
//! vector.
//!
//! Every parameter binding shares the same `def_node` (the surrounding
//! function node), matching the TS shape.

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
        // TS parameter property (`constructor(public x: number)` etc.):
        // npm `oxc-parser` ESTree-fies this as a `TSParameterProperty`
        // wrapper and the inner identifier classifies as a plain read
        // reference (resolving as an implicit global), not a binding.
        // Mirror that: skip declaring the parameter binding when oxc
        // records an accessibility / readonly / override flag.
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
