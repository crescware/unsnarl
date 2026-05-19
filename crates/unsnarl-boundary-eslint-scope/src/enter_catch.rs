//! Push a `Catch` scope, declare the caught parameter, and hoist
//! the catch body's declarations.
//!
//! Mirrors `enterCatch` in
//! `ts/src/boundary/eslint-scope/enter-catch.ts`. The TS port reads
//! `node["param"]` and walks its bindings; the Rust port reads
//! `CatchClause.param: Option<CatchParameter>` and walks
//! `CatchParameter.pattern: BindingPattern<'_>`. The body of the
//! catch is `Box<BlockStatement>` in oxc (vs. plain `BlockStatement`
//! in TS), so we hoist `body.body`.

use oxc_ast::ast::CatchClause;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::declare::collect_binding_identifiers::collect_binding_identifiers;
use crate::hoisting::hoist_declarations::hoist_declarations;
use crate::state::{declare_variable, push_scope, ScopeBuilderState};

pub(crate) fn enter_catch(
    state: &mut ScopeBuilderState,
    catch: &CatchClause<'_>,
    raw: &str,
) -> ScopeId {
    let catch_node = AstNode {
        r#type: AstType::CatchClause,
        span: catch.span,
    };
    let scope_id = push_scope(state, ScopeType::Catch, catch_node.clone());
    if let Some(param) = catch.param.as_ref() {
        for ident in collect_binding_identifiers(&param.pattern) {
            declare_variable(
                state,
                scope_id,
                ident,
                DefinitionType::CatchClause,
                catch_node.clone(),
                None,
            );
        }
    }
    hoist_declarations(state, &catch.body.body, scope_id, raw);
    scope_id
}

#[cfg(test)]
#[path = "enter_catch_test.rs"]
mod enter_catch_test;
