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
    #[serde(rename = "call-proxy")]
    CallProxy,
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
/// Throw carry none. `CallProxy` carries the rendered call head
/// text used as the subgraph's display name (e.g. `"run()"`,
/// `"console.log()"`).
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
    CallProxy {
        #[serde(rename = "callName")]
        call_name: String,
    },
}

/// Callback-argument annotation attached to a `Function`-kind
/// `OwnedVisualSubgraph`.
///
/// Populated by [`super::builder::describe_subgraph`] when the
/// underlying function scope carries a
/// [`unsnarl_ir::scope::CallbackArgument`] -- i.e. when the scope
/// is the `arg_index`-th argument of an `ExpressionStatement`-level
/// call. `callee` is the callee text (e.g. `"run"`,
/// `"console.log"`, `"Promise.resolve().then"`) without trailing
/// argument parens, so the mermaid emitter can synthesize the
/// self-contained header `<callee>(args[<arg_index>])<br/>L_start-end`
/// without revisiting the IR head expression at label time.
#[derive(Clone, Serialize)]
pub struct FunctionCallbackArg {
    pub callee: String,
    #[serde(rename = "argIndex")]
    pub arg_index: u32,
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
    /// Set only when `kind == Function` and the underlying scope is
    /// a direct argument of an ExpressionStatement-level call. Kept
    /// as a separate field (rather than smuggled into `extras`) so
    /// `OwnedExtras::Function` keeps its existing
    /// `{ ownerNodeId, ownerName }` shape, and so existing
    /// `OwnedVisualSubgraph::function(...)` callers do not have to
    /// change at every site -- they default to `None` via the
    /// `base()` helper.
    #[serde(rename = "callbackArg", skip_serializing_if = "Option::is_none")]
    pub callback_arg: Option<FunctionCallbackArg>,
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

impl OwnedVisualSubgraph {
    fn base(
        id: impl Into<String>,
        line: u32,
        kind: OwnedSubgraphKind,
        extras: OwnedExtras,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self {
            r#type: SubgraphTypeTag::Subgraph,
            id: id.into(),
            kind,
            line,
            end_line: None,
            direction,
            extras,
            callback_arg: None,
            elements,
        }
    }

    pub fn function(
        id: impl Into<String>,
        line: u32,
        owner_node_id: Option<String>,
        owner_name: impl Into<String>,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            OwnedSubgraphKind::Function,
            OwnedExtras::Function {
                owner_node_id,
                owner_name: owner_name.into(),
            },
            elements,
            direction,
        )
    }

    /// Synthetic call-proxy subgraph wrapping the callback bodies
    /// of an `ExpressionStatement`-level call. The subgraph's id
    /// matches the id the leaf proxy node would otherwise have
    /// (`expr_stmt_<offset>`), so edges from the call's callee /
    /// non-callback identifier arguments terminate on this
    /// subgraph's border just as they would have terminated on
    /// the leaf node.
    pub fn call_proxy(
        id: impl Into<String>,
        line: u32,
        call_name: impl Into<String>,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            OwnedSubgraphKind::CallProxy,
            OwnedExtras::CallProxy {
                call_name: call_name.into(),
            },
            elements,
            direction,
        )
    }

    pub fn class(
        id: impl Into<String>,
        line: u32,
        class_name: Option<String>,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            OwnedSubgraphKind::Class,
            OwnedExtras::Class { class_name },
            elements,
            direction,
        )
    }

    pub fn if_else_container(
        id: impl Into<String>,
        line: u32,
        has_else: bool,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            OwnedSubgraphKind::IfElseContainer,
            OwnedExtras::IfElseContainer { has_else },
            elements,
            direction,
        )
    }

    pub fn return_subgraph(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            OwnedSubgraphKind::Return,
            OwnedExtras::None {},
            elements,
            direction,
        )
    }

    pub fn throw_subgraph(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            OwnedSubgraphKind::Throw,
            OwnedExtras::None {},
            elements,
            direction,
        )
    }
}

impl ControlVisualSubgraph {
    fn base(
        id: impl Into<String>,
        line: u32,
        kind: ControlSubgraphKind,
        extras: ControlExtras,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self {
            r#type: SubgraphTypeTag::Subgraph,
            id: id.into(),
            line,
            end_line: None,
            direction,
            elements,
            kind,
            extras,
        }
    }

    pub fn case(
        id: impl Into<String>,
        line: u32,
        case_test: Option<String>,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::Case,
            ControlExtras::Case { case_test },
            elements,
            direction,
        )
    }

    pub fn if_subgraph(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::If,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn else_subgraph(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::Else,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn switch(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::Switch,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn try_subgraph(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::Try,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn catch(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::Catch,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn finally(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::Finally,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn for_subgraph(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::For,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn while_subgraph(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::While,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn do_while(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::DoWhile,
            ControlExtras::None {},
            elements,
            direction,
        )
    }

    pub fn block(
        id: impl Into<String>,
        line: u32,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            ControlSubgraphKind::Block,
            ControlExtras::None {},
            elements,
            direction,
        )
    }
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum VisualSubgraph {
    Owned(OwnedVisualSubgraph),
    Control(ControlVisualSubgraph),
}

impl From<OwnedVisualSubgraph> for VisualSubgraph {
    fn from(s: OwnedVisualSubgraph) -> Self {
        Self::Owned(s)
    }
}

impl From<ControlVisualSubgraph> for VisualSubgraph {
    fn from(s: ControlVisualSubgraph) -> Self {
        Self::Control(s)
    }
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
                OwnedSubgraphKind::CallProxy => SubgraphKind::CallProxy,
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

    /// `callName` (JSON field), present only on `CallProxy`
    /// subgraphs. The text reproduces what the leaf
    /// `expr_stmt_<offset>` node's label would otherwise have
    /// shown (`render_head_expression`'s output, e.g. `"run()"`,
    /// `"console.log()"`), so the mermaid emitter can use it as
    /// the subgraph header.
    pub fn call_name(&self) -> Option<&str> {
        match self {
            Self::Owned(s) => match &s.extras {
                OwnedExtras::CallProxy { call_name } => Some(call_name.as_str()),
                _ => None,
            },
            Self::Control(_) => None,
        }
    }

    /// `callbackArg` (JSON field), present only on `Function`
    /// subgraphs whose underlying scope is a direct argument of an
    /// `ExpressionStatement`-level call. Returns `(callee,
    /// arg_index)` for label synthesis; `None` for plain anonymous /
    /// named functions.
    pub fn callback_arg(&self) -> Option<(&str, u32)> {
        match self {
            Self::Owned(s) => s
                .callback_arg
                .as_ref()
                .map(|c| (c.callee.as_str(), c.arg_index)),
            Self::Control(_) => None,
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
