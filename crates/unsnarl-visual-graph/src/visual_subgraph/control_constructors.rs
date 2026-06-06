//! Constructors for [`ControlVisualSubgraph`].

use crate::direction::Direction;
use crate::visual_element::VisualElement;
use crate::visual_element_type::SubgraphTypeTag;

use super::{ControlExtras, ControlSubgraphKind, ControlVisualSubgraph};

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
