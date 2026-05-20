//! `VisualSubgraph`: a container element in the visual graph.
//!
//! Mirrors `ts/src/visual-graph/visual-subgraph.ts` together with
//! the construction sites under `ts/src/visual-graph/builder/`. As
//! with [`VisualNode`](crate::visual_node::VisualNode), the on-disk
//! JSON field order varies by construction site:
//!
//! - `describeSubgraph` ("owned") path for Function / Class /
//!   Return / Throw subgraphs and `build-children`'s manually-built
//!   IfElseContainer write `{ type, id, kind, line, endLine,
//!   direction, ...extras, elements }`.
//! - `describeSubgraph` ("control" common-spread) path for Case /
//!   Switch / If / Else / Try / Catch / Finally / For / While /
//!   DoWhile / Block writes `{ type, id, line, endLine, direction,
//!   elements, kind, ...extras }`.
//!
//! Two struct shapes mirror that split. They are wrapped in a
//! single untagged [`VisualSubgraph`] enum so the rest of the
//! builder can hold a uniform element kind.

use serde::Serialize;

use crate::direction::Direction;
use crate::visual_element::VisualElement;
use crate::visual_element_type::SubgraphTypeTag;

/// Kind values that take the "owned" field order (kind early,
/// elements last).
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OwnedSubgraphKind {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "return")]
    Return,
    #[serde(rename = "throw")]
    Throw,
    #[serde(rename = "if-else-container")]
    IfElseContainer,
}

/// Tail fields the "owned" shape carries *before* `elements`.
/// Function/Class/IfElseContainer each have their own. Return /
/// Throw carry none.
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum OwnedExtras {
    None {},
    Function {
        #[serde(rename = "ownerNodeId")]
        owner_node_id: Option<String>,
        #[serde(rename = "ownerName")]
        owner_name: String,
    },
    Class {
        #[serde(rename = "className")]
        class_name: Option<String>,
    },
    IfElseContainer {
        #[serde(rename = "hasElse")]
        has_else: bool,
    },
}

/// "Owned" subgraph: kind appears right after id, extras sit just
/// before `elements`.
#[derive(Clone, Serialize)]
pub struct OwnedVisualSubgraph {
    #[serde(rename = "type")]
    pub r#type: SubgraphTypeTag,
    pub id: String,
    pub kind: OwnedSubgraphKind,
    pub line: u32,
    #[serde(rename = "endLine")]
    pub end_line: Option<u32>,
    pub direction: Direction,
    #[serde(flatten)]
    pub extras: OwnedExtras,
    pub elements: Vec<VisualElement>,
}

/// Kind values that take the "control" field order (elements
/// before kind).
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ControlSubgraphKind {
    #[serde(rename = "case")]
    Case,
    #[serde(rename = "switch")]
    Switch,
    #[serde(rename = "if")]
    If,
    #[serde(rename = "else")]
    Else,
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
    #[serde(rename = "block")]
    Block,
}

/// Tail fields the "control" shape carries *after* `kind`. Only
/// `Case` carries one.
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum ControlExtras {
    None {},
    Case {
        #[serde(rename = "caseTest")]
        case_test: Option<String>,
    },
}

/// "Control" subgraph: elements appears in the middle, kind near
/// the end.
#[derive(Clone, Serialize)]
pub struct ControlVisualSubgraph {
    #[serde(rename = "type")]
    pub r#type: SubgraphTypeTag,
    pub id: String,
    pub line: u32,
    #[serde(rename = "endLine")]
    pub end_line: Option<u32>,
    pub direction: Direction,
    pub elements: Vec<VisualElement>,
    pub kind: ControlSubgraphKind,
    #[serde(flatten)]
    pub extras: ControlExtras,
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum VisualSubgraph {
    Owned(OwnedVisualSubgraph),
    Control(ControlVisualSubgraph),
}

impl VisualSubgraph {
    pub fn id(&self) -> &str {
        match self {
            Self::Owned(s) => &s.id,
            Self::Control(s) => &s.id,
        }
    }

    pub fn elements_mut(&mut self) -> &mut Vec<VisualElement> {
        match self {
            Self::Owned(s) => &mut s.elements,
            Self::Control(s) => &mut s.elements,
        }
    }

    pub fn elements(&self) -> &Vec<VisualElement> {
        match self {
            Self::Owned(s) => &s.elements,
            Self::Control(s) => &s.elements,
        }
    }

    pub fn line(&self) -> u32 {
        match self {
            Self::Owned(s) => s.line,
            Self::Control(s) => s.line,
        }
    }

    pub fn end_line(&self) -> Option<u32> {
        match self {
            Self::Owned(s) => s.end_line,
            Self::Control(s) => s.end_line,
        }
    }

    pub fn set_end_line(&mut self, end_line: Option<u32>) {
        match self {
            Self::Owned(s) => s.end_line = end_line,
            Self::Control(s) => s.end_line = end_line,
        }
    }
}

#[cfg(test)]
#[path = "visual_subgraph_test.rs"]
mod visual_subgraph_test;
