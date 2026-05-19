//! Hoist the top-level body of a parsed program into a target scope.
//!
//! Mirrors `hoistInto` in `ts/src/boundary/eslint-scope/hoist-into.ts`.
//! TS reads `program["body"]` and dispatches when it is an array; the
//! Rust port takes a typed `&Program<'_>` where `body: Vec<Statement>`
//! is structurally guaranteed, so no array-check is needed.

use oxc_ast::ast::Program;

use unsnarl_ir::ids::ScopeId;

use crate::hoisting::hoist_declarations::hoist_declarations;
use crate::state::ScopeBuilderState;

pub(crate) fn hoist_into(
    state: &mut ScopeBuilderState,
    program: &Program<'_>,
    scope: ScopeId,
    raw: &str,
) {
    hoist_declarations(state, &program.body, scope, raw);
}

#[cfg(test)]
#[path = "hoist_into_test.rs"]
mod hoist_into_test;
