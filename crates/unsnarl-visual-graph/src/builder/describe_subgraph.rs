//! Mirrors `ts/src/visual-graph/builder/describe-subgraph.ts`.
//!
//! Constructs the `VisualSubgraph` value for a given scope.
//! Function / Class take the "owned" path (kind early, elements
//! last); Return / Throw share that shape but are created by the
//! `ensure_*` helpers instead. The control branch (Case / Switch /
//! If / Else / Try / Catch / Finally / For / While / DoWhile /
//! Block) takes the "control" common-spread layout where elements
//! sits in the middle and kind appears near the end.

use std::collections::HashMap;

use unsnarl_ir::scope::block_context::BlockContext;
use unsnarl_ir::serialized::{SerializedScope, SerializedVariable};

use crate::direction::Direction;
use crate::visual_element_type::SubgraphTypeTag;
use crate::visual_subgraph::{
    ControlExtras, ControlSubgraphKind, ControlVisualSubgraph, OwnedExtras, OwnedSubgraphKind,
    OwnedVisualSubgraph, VisualSubgraph,
};

use super::control_subgraph_kind_of::control_subgraph_kind_of;
use super::is_class_subgraph::is_class_subgraph;
use super::is_function_subgraph::is_function_subgraph;
use super::node_id::node_id;
use super::subgraph_scope_id::subgraph_scope_id;

pub fn describe_subgraph(
    scope: &SerializedScope,
    subgraph_owner_var: &HashMap<String, String>,
    variable_map: &HashMap<String, &SerializedVariable>,
) -> VisualSubgraph {
    let id = subgraph_scope_id(scope);
    let end_line = Some(scope.block.end_span.line.0);

    if is_function_subgraph(scope) {
        let owner_var_id = subgraph_owner_var.get(scope.id.value()).cloned();
        let owner_var = owner_var_id
            .as_ref()
            .and_then(|id| variable_map.get(id).copied());
        let start_line = owner_var
            .and_then(|v| v.identifiers.first())
            .map(|s| s.line.0)
            .unwrap_or_else(|| scope.block.span.line.0);
        let owner_node_id = owner_var_id.as_ref().map(|v| node_id(v));
        let owner_name = owner_var.map(|v| v.name().to_string()).unwrap_or_default();
        return VisualSubgraph::Owned(OwnedVisualSubgraph {
            r#type: SubgraphTypeTag::Subgraph,
            id,
            kind: OwnedSubgraphKind::Function,
            line: start_line,
            end_line,
            direction: Direction::RL,
            extras: OwnedExtras::Function {
                owner_node_id,
                owner_name,
            },
            elements: Vec::new(),
        });
    }

    if is_class_subgraph(scope) {
        // The class's own identifier (`Foo` in `class Foo {}`) lives
        // inside the class scope as the inner ClassName binding.
        // Anonymous `ClassExpression` has no such binding, so the
        // variables list is empty.
        let inner_name = scope
            .variables
            .first()
            .and_then(|id| variable_map.get(id.value()).copied())
            .map(|v| v.name().to_string());
        return VisualSubgraph::Owned(OwnedVisualSubgraph {
            r#type: SubgraphTypeTag::Subgraph,
            id,
            kind: OwnedSubgraphKind::Class,
            line: scope.block.span.line.0,
            end_line,
            direction: Direction::RL,
            extras: OwnedExtras::Class {
                class_name: inner_name,
            },
            elements: Vec::new(),
        });
    }

    let kind = control_subgraph_kind_of(scope)
        .expect("describe_subgraph: scope is neither function, class, nor a control subgraph");

    let extras = match kind {
        ControlSubgraphKind::Case => {
            let case_test = scope.block_context.as_ref().and_then(|ctx| match ctx {
                BlockContext::CaseClause(c) => c.case_test().map(|s| s.to_string()),
                _ => None,
            });
            ControlExtras::Case { case_test }
        }
        _ => ControlExtras::None {},
    };

    VisualSubgraph::Control(ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id,
        line: scope.block.span.line.0,
        end_line,
        direction: Direction::RL,
        elements: Vec::new(),
        kind,
        extras,
    })
}
