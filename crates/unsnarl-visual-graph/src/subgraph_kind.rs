//! `SubgraphKind`: tag on `VisualSubgraph` distinguishing every shape
//! of subgraph the builder emits.
//!
//! Mirrors `ts/src/visual-graph/subgraph-kind.ts`. The values double
//! as the on-disk JSON tags (`"function"`, `"if-else-container"`,
//! ...), so they serialize to those literal strings.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubgraphKind {
    #[serde(rename = "function")]
    Function,
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
