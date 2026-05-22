//! `VariableDeclarationKind`: `var` / `let` / `const`.
//!
//! Placed in `unsnarl-oxc-parity`, not `unsnarl-ir`, because the
//! values are read directly off `oxc_ast`'s
//! `VariableDeclaration.kind` field. The serializer compares incoming
//! oxc strings value-for-value against these variants, so the
//! membership and spelling of the set are owned by oxc, not by the
//! IR contract. The set is ECMA-bounded today (3 keywords) but
//! extensible on the oxc side (`using` / `await using` are landing
//! for ES2024 explicit resource management); the same parity
//! argument that pulled `AstType` out of the IR contract applies
//! here, so the change driver — an oxc upgrade — lives in this
//! crate rather than `unsnarl-ir`.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}
