//! Discriminator for `HeadExpression` variants.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpressionStatementHeadKind {
    Identifier,
    Member,
    Call,
    New,
    Await,
    Assign,
    Update,
    /// Marker for an operand whose AST shape isn't reducible to the
    /// head vocabulary (literal, computed member, arrow, template
    /// literal, etc.). Rendered as "..." so the surrounding structure
    /// still reads as an assignment / update.
    Elided,
    Raw,
}
