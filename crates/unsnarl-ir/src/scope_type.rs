//! Scope categorization. Ports `ts/src/analyzer/scope-type.ts`.
//!
//! Lives in `unsnarl-ir` rather than `unsnarl-analyzer` because the IR
//! contract types reference it and `unsnarl-ir` is below
//! `unsnarl-analyzer` in the dependency graph.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScopeType {
    Block,
    Catch,
    Class,
    ClassFieldInitializer,
    ClassStaticBlock,
    For,
    Function,
    FunctionExpressionName,
    Global,
    Module,
    Switch,
    With,
}
