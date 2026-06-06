//! Constructors for [`OwnedVisualSubgraph`].

use crate::direction::Direction;
use crate::visual_element::VisualElement;
use crate::visual_element_type::SubgraphTypeTag;

use super::{OwnedExtras, OwnedSubgraphKind, OwnedVisualSubgraph};

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

    /// Import-source subgraph: groups every local binding (and any
    /// renamed-import intermediate) that an `import ... from
    /// "<source>"` statement introduces. `module_source` is the raw
    /// specifier text; the id is the sanitized `sg_<source>` form,
    /// which is lossy, so the source is carried separately for the
    /// `module <source>` header.
    pub fn module(
        id: impl Into<String>,
        line: u32,
        module_source: impl Into<String>,
        elements: Vec<VisualElement>,
        direction: Direction,
    ) -> Self {
        Self::base(
            id,
            line,
            OwnedSubgraphKind::Module,
            OwnedExtras::Module {
                module_source: module_source.into(),
            },
            elements,
            direction,
        )
    }
}
