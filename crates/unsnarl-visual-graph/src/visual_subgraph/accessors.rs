//! Accessors for the [`VisualSubgraph`] enum: uniform readers that
//! project both underlying shapes onto a single interface.

use crate::direction::Direction;
use crate::visual_element::VisualElement;

use super::{ControlExtras, ControlSubgraphKind, OwnedExtras, OwnedSubgraphKind, VisualSubgraph};

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
                OwnedSubgraphKind::ConditionalContainer => SubgraphKind::ConditionalContainer,
                OwnedSubgraphKind::Module => SubgraphKind::Module,
            },
            Self::Control(s) => match s.kind {
                ControlSubgraphKind::Case => SubgraphKind::Case,
                ControlSubgraphKind::Switch => SubgraphKind::Switch,
                ControlSubgraphKind::If => SubgraphKind::If,
                ControlSubgraphKind::Else => SubgraphKind::Else,
                ControlSubgraphKind::Consequent => SubgraphKind::Consequent,
                ControlSubgraphKind::Alternate => SubgraphKind::Alternate,
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
    /// function is named, `None` for anonymous functions). Every other
    /// kind, including `CallProxy`, carries no owner and returns `None`.
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
                OwnedExtras::CallProxy { call_name, .. } => Some(call_name.as_str()),
                _ => None,
            },
            Self::Control(_) => None,
        }
    }

    /// `moduleSource` (JSON field), present only on `Module`
    /// subgraphs. The raw import specifier text the mermaid emitter
    /// renders as the `module <source>` header.
    pub fn module_source(&self) -> Option<&str> {
        match self {
            Self::Owned(s) => match &s.extras {
                OwnedExtras::Module { module_source } => Some(module_source.as_str()),
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
