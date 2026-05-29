//! `VariableDeclarationKind`: `var` / `let` / `const` / `using` /
//! `await using`.
//!
//! Placed in `unsnarl-oxc-parity`, not `unsnarl-ir`, because the
//! values are read directly off `oxc_ast`'s
//! `VariableDeclaration.kind` field. The serializer compares incoming
//! oxc strings value-for-value against these variants, so the
//! membership and spelling of the set are owned by oxc, not by the
//! IR contract. The set covers the five ECMAScript declaration
//! keywords oxc emits — the three classic forms plus the ES2024
//! explicit-resource-management forms (`using` / `await using`); the
//! same parity argument that pulled `AstType` out of the IR contract
//! applies here, so the change driver — an oxc upgrade — lives in
//! this crate rather than `unsnarl-ir`.
//!
//! The serialized spellings mirror `oxc_ast`'s
//! `VariableDeclarationKind::as_str` exactly: `await using` keeps its
//! internal space, so the `AwaitUsing` variant is renamed rather than
//! lowercased to `awaitusing`.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
    Using,
    #[serde(rename = "await using")]
    AwaitUsing,
}
