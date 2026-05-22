//! `oxc_semantic::Scoping` scope tree → `IrArena.scopes`.
//!
//! Phase 2 stub. The real mapping reads `Scoping::get_node_id(scope_id)`
//! to recover each scope's anchor AST node, then materialises an
//! [`unsnarl_ir::ScopeData`] via [`crate::materialise::ast_node_of`].
//! `ScopeType` is derived from `ScopeFlags` (Function / Block / Catch /
//! ClassStaticBlock / TS-specific kinds), `is_strict` from the root
//! flag inheritance, and `function_expression_scope` from the parent
//! being a `FunctionExpression` whose binding identifier sits in the
//! scope.
//!
//! `child_scopes` / `variables` / `references` / `through` get filled
//! by the orchestrator (`build.rs`) once all entity passes have run,
//! so this module returns the raw `ScopeData` rows in `oxc_semantic`
//! ScopeId order; ID translation to `unsnarl_ir::ScopeId` is a 1:1
//! `usize` cast handled in `build.rs`.
//!
//! TODO(phase-2): fill in the actual mapping.
