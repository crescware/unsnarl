//! `oxc_semantic::Scoping` symbols → `IrArena.variables`.
//!
//! Phase 2 stub. The real mapping walks every `SymbolId` in
//! `Scoping::symbol_ids`, builds a [`unsnarl_ir::VariableData`]
//! (name, scope, identifiers, refs-empty-until-reference-pass, defs-
//! empty-until-definition-pass), and additionally synthesises the
//! per-function implicit `arguments` binding that `oxc_semantic` does
//! NOT create but eslint-scope does (pinned by
//! `oxc_semantic_probe_test::arguments_is_or_is_not_a_symbol_inside_a_function`).
//!
//! TODO(phase-2): fill in the actual mapping.
