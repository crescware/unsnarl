//! `var` / `let` / `const`. Ports
//! `ts/src/serializer/variable-declaration-kind.ts`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}
