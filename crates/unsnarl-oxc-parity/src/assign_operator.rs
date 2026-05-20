//! `AssignOperator`: the 16 assignment operators ECMA defines for
//! `AssignmentExpression.operator`.
//!
//! Placed in `unsnarl-oxc-parity`, not `unsnarl-ir`, because the
//! values are read directly off oxc's `AssignmentExpression.operator`
//! field (see TS `analyzer/expression-statement-head.ts:121`, which
//! pulls `node.operator` from the AST and stores it verbatim). The
//! spellings must therefore stay value-for-value aligned with what
//! `oxc_ast` emits; the change driver is the same as
//! `VariableDeclarationKind`'s, so it belongs in the same crate.
//!
//! `#[serde(rename = "...")]` on each variant pins the on-disk JSON
//! form (`"+="`, `"=>"`, etc.) so consumers see the same strings the
//! TS port wrote, even though Rust spells the variants in PascalCase.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum AssignOperator {
    #[serde(rename = "=")]
    Assign,
    #[serde(rename = "+=")]
    AddAssign,
    #[serde(rename = "-=")]
    SubAssign,
    #[serde(rename = "*=")]
    MulAssign,
    #[serde(rename = "/=")]
    DivAssign,
    #[serde(rename = "%=")]
    RemAssign,
    #[serde(rename = "**=")]
    ExpAssign,
    #[serde(rename = "<<=")]
    ShlAssign,
    #[serde(rename = ">>=")]
    ShrAssign,
    #[serde(rename = ">>>=")]
    UnsignedShrAssign,
    #[serde(rename = "&=")]
    BitAndAssign,
    #[serde(rename = "|=")]
    BitOrAssign,
    #[serde(rename = "^=")]
    BitXorAssign,
    #[serde(rename = "&&=")]
    LogicalAndAssign,
    #[serde(rename = "||=")]
    LogicalOrAssign,
    #[serde(rename = "??=")]
    NullishAssign,
}

impl AssignOperator {
    /// Source spelling of the operator (`"="`, `"+="`, ...). Used by
    /// `render_head_expression` to format the operator inside a
    /// rendered assignment label.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Assign => "=",
            Self::AddAssign => "+=",
            Self::SubAssign => "-=",
            Self::MulAssign => "*=",
            Self::DivAssign => "/=",
            Self::RemAssign => "%=",
            Self::ExpAssign => "**=",
            Self::ShlAssign => "<<=",
            Self::ShrAssign => ">>=",
            Self::UnsignedShrAssign => ">>>=",
            Self::BitAndAssign => "&=",
            Self::BitOrAssign => "|=",
            Self::BitXorAssign => "^=",
            Self::LogicalAndAssign => "&&=",
            Self::LogicalOrAssign => "||=",
            Self::NullishAssign => "??=",
        }
    }
}
