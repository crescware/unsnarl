//! Declare-side helpers shared across `enter_*` and `hoisting/*`.
//!
//! Mirrors `ts/src/boundary/eslint-scope/declare/`. The TS directory
//! groups `collectBindingIdentifiers`, `declareVariable`,
//! `declareImplicitArguments`, and `isAstNode`; the Rust port pulls
//! `declareVariable` into [`crate::state`] (it directly mutates
//! `ScopeBuilderState`) and keeps the rest here so the imports stay
//! aligned with the TS source.

pub(crate) mod collect_binding_identifiers;
