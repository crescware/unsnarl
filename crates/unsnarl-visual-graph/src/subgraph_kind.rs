//! `SubgraphKind`: tag on `VisualSubgraph` distinguishing every shape
//! of subgraph the builder emits.
//!
//! The values double as the on-disk JSON tags (`"function"`,
//! `"if-else-container"`, ...), so they serialize to those literal
//! strings.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubgraphKind {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "call-proxy")]
    CallProxy,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "switch")]
    Switch,
    #[serde(rename = "case")]
    Case,
    #[serde(rename = "if")]
    If,
    #[serde(rename = "else")]
    Else,
    #[serde(rename = "if-else-container")]
    IfElseContainer,
    #[serde(rename = "try")]
    Try,
    #[serde(rename = "catch")]
    Catch,
    #[serde(rename = "finally")]
    Finally,
    #[serde(rename = "for")]
    For,
    #[serde(rename = "while")]
    While,
    #[serde(rename = "do-while")]
    DoWhile,
    #[serde(rename = "return")]
    Return,
    #[serde(rename = "throw")]
    Throw,
    #[serde(rename = "block")]
    Block,
}

impl SubgraphKind {
    /// The bare variant name as it appears in the on-disk JSON
    /// (`"function"`, `"if-else-container"`, ...). Used by the
    /// mermaid emitter's `--debug` mode to splice the kind name
    /// into the rendered label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::CallProxy => "call-proxy",
            Self::Class => "class",
            Self::Switch => "switch",
            Self::Case => "case",
            Self::If => "if",
            Self::Else => "else",
            Self::IfElseContainer => "if-else-container",
            Self::Try => "try",
            Self::Catch => "catch",
            Self::Finally => "finally",
            Self::For => "for",
            Self::While => "while",
            Self::DoWhile => "do-while",
            Self::Return => "return",
            Self::Throw => "throw",
            Self::Block => "block",
        }
    }
}
