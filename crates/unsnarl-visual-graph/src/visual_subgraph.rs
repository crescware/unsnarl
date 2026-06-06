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

mod accessors;
mod control_constructors;
mod owned_constructors;

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
    #[serde(rename = "module")]
    Module,
}

/// Tail fields the "owned" shape carries *before* `elements`.
/// Function/Class/IfElseContainer each have their own. Return /
/// Throw carry none. `CallProxy` carries the rendered call head
/// text used as the subgraph's display name (e.g. `"run()"`,
/// `"console.log()"`). `Module` carries the original import source
/// specifier (e.g. `"./utils/helper"`, `"../lib/mod.js"`) so the
/// mermaid emitter can render the `module <source>` header even
/// though the subgraph's id is the sanitized `sg_<source>` form.
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
    Module {
        #[serde(rename = "moduleSource")]
        module_source: String,
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

#[cfg(test)]
#[path = "visual_subgraph_test.rs"]
mod visual_subgraph_test;
