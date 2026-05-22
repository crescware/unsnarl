//! `VisualSubgraph`: a container element in the visual graph.
//!
//! As with [`VisualNode`](crate::visual_node::VisualNode), the
//! on-disk JSON field order varies by construction site:
//!
//! - `describe_subgraph` ("owned") path for Function / Class /
//!   Return / Throw subgraphs and `build_children`'s manually-built
//!   IfElseContainer writes
//!   `{ type, id, kind, line, end_line, direction, ...extras, elements }`.
//! - `describe_subgraph` ("control" common-spread) path for
//!   Case / Switch / If / Else / Try / Catch / Finally / For /
//!   While / DoWhile / Block writes
//!   `{ type, id, line, end_line, direction, elements, kind, ...extras }`.
//!
//! Two struct shapes match the split, wrapped in a single untagged
//! [`VisualSubgraph`] enum so the rest of the builder can hold a
//! uniform element kind.

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

    /// Returns the logical [`SubgraphKind`] discriminator.
    ///
    /// The two underlying shapes (owned / control) carry their own
    /// subset enums to preserve JSON field order; this projects them
    /// back onto a single flat [`SubgraphKind`] so consumers can
    /// match in one switch.
    pub fn kind(&self) -> crate::subgraph_kind::SubgraphKind {
        use crate::subgraph_kind::SubgraphKind;
        match self {
            Self::Owned(s) => match s.kind {
                OwnedSubgraphKind::Function => SubgraphKind::Function,
                OwnedSubgraphKind::Class => SubgraphKind::Class,
                OwnedSubgraphKind::Return => SubgraphKind::Return,
                OwnedSubgraphKind::Throw => SubgraphKind::Throw,
                OwnedSubgraphKind::IfElseContainer => SubgraphKind::IfElseContainer,
            },
            Self::Control(s) => match s.kind {
                ControlSubgraphKind::Case => SubgraphKind::Case,
                ControlSubgraphKind::Switch => SubgraphKind::Switch,
                ControlSubgraphKind::If => SubgraphKind::If,
                ControlSubgraphKind::Else => SubgraphKind::Else,
                ControlSubgraphKind::Try => SubgraphKind::Try,
                ControlSubgraphKind::Catch => SubgraphKind::Catch,
                ControlSubgraphKind::Finally => SubgraphKind::Finally,
                ControlSubgraphKind::For => SubgraphKind::For,
                ControlSubgraphKind::While => SubgraphKind::While,
                ControlSubgraphKind::DoWhile => SubgraphKind::DoWhile,
                ControlSubgraphKind::Block => SubgraphKind::Block,
            },
        }
    }

    pub fn direction(&self) -> Direction {
        match self {
            Self::Owned(s) => s.direction,
            Self::Control(s) => s.direction,
        }
    }

    /// `ownerNodeId` (JSON field), present only on `Function`
    /// subgraphs (the FunctionDeclaration's node id when the
    /// function is named, `None` for anonymous functions).
    pub fn owner_node_id(&self) -> Option<&str> {
        match self {
            Self::Owned(s) => match &s.extras {
                OwnedExtras::Function { owner_node_id, .. } => owner_node_id.as_deref(),
                _ => None,
            },
            Self::Control(_) => None,
        }
    }

    /// `ownerName` (JSON field), present only on `Function`
    /// subgraphs. The field is serialised as the empty string when
    /// the owner is anonymous; that is preserved here verbatim so
    /// the mermaid emitter's fallback to `node_map` lookup matches.
    pub fn owner_name(&self) -> Option<&str> {
        match self {
            Self::Owned(s) => match &s.extras {
                OwnedExtras::Function { owner_name, .. } => Some(owner_name.as_str()),
                _ => None,
            },
            Self::Control(_) => None,
        }
    }

    /// `className` (JSON field), present only on `Class` subgraphs.
    pub fn class_name(&self) -> Option<&str> {
        match self {
            Self::Owned(s) => match &s.extras {
                OwnedExtras::Class { class_name } => class_name.as_deref(),
                _ => None,
            },
            Self::Control(_) => None,
        }
    }

    /// `caseTest` (JSON field), present only on `Case` subgraphs.
    /// `None` here corresponds to the default case (serialised as
    /// `null`).
    pub fn case_test(&self) -> Option<&str> {
        match self {
            Self::Control(s) => match &s.extras {
                ControlExtras::Case { case_test } => case_test.as_deref(),
                _ => None,
            },
            Self::Owned(_) => None,
        }
    }

    /// `hasElse` (JSON field), present only on `IfElseContainer`
    /// subgraphs.
    pub fn has_else(&self) -> bool {
        match self {
            Self::Owned(s) => match &s.extras {
                OwnedExtras::IfElseContainer { has_else } => *has_else,
                _ => false,
            },
            Self::Control(_) => false,
        }
    }
}

#[cfg(test)]
#[path = "visual_subgraph_test.rs"]
mod visual_subgraph_test;
