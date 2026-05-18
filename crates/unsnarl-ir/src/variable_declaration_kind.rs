//! `var` / `let` / `const`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}
