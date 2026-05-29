//! Constructs the `VisualSubgraph` value for a given scope.
//! Function / Class take the "owned" path (kind early, elements
//! last); Return / Throw share that shape but are created by the
//! `ensure_*` helpers instead. The control branch (Case / Switch /
//! If / Else / Try / Catch / Finally / For / While / DoWhile /
//! Block) takes the "control" common-spread layout where elements
//! sits in the middle and kind appears near the end.

use std::collections::HashMap;

use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::scope::block_context::BlockContext;
use unsnarl_ir::serialized::{SerializedScope, SerializedVariable};

use crate::direction::Direction;
use crate::visual_subgraph::{
    ControlExtras, ControlSubgraphKind, ControlVisualSubgraph, FunctionCallbackArg,
    OwnedVisualSubgraph, VisualSubgraph,
};

use super::control_subgraph_kind_of::control_subgraph_kind_of;
use super::is_class_subgraph::is_class_subgraph;
use super::is_function_subgraph::is_function_subgraph;
use super::node_id::node_id;
use super::render_head_expression::render_head_expression;
use super::subgraph_scope_id::subgraph_scope_id;

pub fn describe_subgraph(
    scope: &SerializedScope,
    subgraph_owner_var: &HashMap<String, String>,
    variable_map: &HashMap<&str, &SerializedVariable>,
    source_index: &SourceIndex<'_>,
) -> VisualSubgraph {
    let id = subgraph_scope_id(scope);
    let end_line = Some(scope.block.end_span.line.0);

    if is_function_subgraph(scope) {
        let owner_var_id = subgraph_owner_var.get(scope.id.value()).cloned();
        let owner_var = owner_var_id
            .as_ref()
            .and_then(|id| variable_map.get(id.as_str()).copied());
        let start_line = owner_var
            .and_then(|v| v.identifiers.first())
            .map(|s| s.line.0)
            .unwrap_or_else(|| scope.block.span.line.0);
        let owner_node_id = owner_var_id.as_ref().map(|v| node_id(v));
        let owner_name = owner_var.map(|v| v.name().to_string()).unwrap_or_default();
        let mut sg = OwnedVisualSubgraph::function(
            id,
            start_line,
            owner_node_id,
            owner_name,
            Vec::new(),
            Direction::RL,
        );
        sg.end_line = end_line;
        // When the scope is the i-th argument of a call (per the
        // analyzer's `CallbackArgument` annotation), attach
        // `callbackArg` so the mermaid emitter can label the subgraph
        // as `<callee>(args[<i>])`. The annotation now carries the
        // call's `callee` head subtree directly, so the label renders
        // for any call-argument function -- variable-bound, returned,
        // or nested -- not just `ExpressionStatement`-level ones.
        if let Some(cb) = scope.callback_argument.as_ref() {
            sg.callback_arg = Some(FunctionCallbackArg {
                callee: render_head_expression(&cb.callee, source_index),
                arg_index: cb.arg_index,
            });
        }
        return sg.into();
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
        let mut sg = OwnedVisualSubgraph::class(
            id,
            scope.block.span.line.0,
            inner_name,
            Vec::new(),
            Direction::RL,
        );
        sg.end_line = end_line;
        return sg.into();
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

    let mut sg = ControlVisualSubgraph {
        extras,
        ..ControlVisualSubgraph::block(id, scope.block.span.line.0, Vec::new(), Direction::RL)
    };
    sg.kind = kind;
    sg.end_line = end_line;
    sg.into()
}

#[cfg(test)]
#[path = "describe_subgraph_test.rs"]
mod describe_subgraph_test;
