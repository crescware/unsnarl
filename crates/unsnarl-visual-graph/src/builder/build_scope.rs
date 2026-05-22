//! Per-scope render dispatcher: if the scope is gated below the
//! depth ceiling its subtree is recorded as collapsed and the
//! visible anchor (owner variable or a BeyondDepth stub) is
//! cached so edges crossing the boundary still have a destination;
//! otherwise the scope's own subgraph (when it should have one) is
//! constructed, variables and write ops are pushed in source order,
//! children are walked, and loop-test / switch-discriminant anchors
//! are staged for end-of-build placement.

use unsnarl_ir::serialized::SerializedDefinition;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::VariableDeclarationKind;

use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::build_children::build_children;
use super::context::BuilderContext;
use super::describe_subgraph::describe_subgraph;
use super::ensure_beyond_depth_stub::ensure_beyond_depth_stub;
use super::is_collapsed::is_collapsed;
use super::loop_test_anchor::attach_loop_test_anchor;
use super::make_variable_node::make_variable_node;
use super::node_id::node_id;
use super::should_subgraph::should_subgraph;
use super::state::BuildState;
use super::switch_discriminant_anchor::attach_switch_discriminant_anchor;
use super::visible_ancestor_subgraph::visible_ancestor_subgraph;
use super::write_op_node_id::write_op_node_id;

fn record_collapsed_descendants(
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    scope: &SerializedScope,
    root_scope_id: &str,
) {
    state
        .collapsed_root_by_scope
        .insert(scope.id.value().to_string(), root_scope_id.to_string());
    for child_id in &scope.child_scopes {
        let Some(child) = ctx.scope_map.get(child_id.value()).copied() else {
            continue;
        };
        record_collapsed_descendants(state, ctx, child, root_scope_id);
    }
}

pub fn build_scope(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    scope: &SerializedScope,
    container: Container,
) {
    if is_collapsed(scope, ctx.depths.as_ref()) {
        record_collapsed_descendants(state, ctx, scope, scope.id.value());
        let owner_var_id = ctx.subgraph_owner_var.get(scope.id.value()).cloned();
        let anchor_id: Option<String> = if let Some(var_id) = owner_var_id.as_ref() {
            Some(node_id(var_id))
        } else {
            let parent_sg: Option<SubgraphIdx> = visible_ancestor_subgraph(scope, ctx, state);
            parent_sg.map(|sg| ensure_beyond_depth_stub(arena, state, sg))
        };
        if let Some(anchor) = anchor_id {
            state
                .collapsed_anchor_by_root
                .insert(scope.id.value().to_string(), anchor.clone());
            if let Some(block_ctx) = scope.block_context.as_ref() {
                state
                    .suppressed_predicate_redirect
                    .insert(block_ctx.parent_span_offset().0, anchor);
            }
        }
        return;
    }

    let subgraph_here = should_subgraph(scope);
    let mut body_container = container;
    let mut body_subgraph: Option<SubgraphIdx> = None;
    if subgraph_here {
        let sg_descriptor = describe_subgraph(scope, &ctx.subgraph_owner_var, &ctx.variable_map);
        let idx = arena.push_subgraph(sg_descriptor);
        arena.append_child(container, ElementHandle::Subgraph(idx));
        body_container = Container::Subgraph(idx);
        body_subgraph = Some(idx);
        state
            .subgraph_by_scope
            .insert(scope.id.value().to_string(), idx);
        if let Some(owner_var) = ctx.subgraph_owner_var.get(scope.id.value()) {
            state.function_subgraph_by_fn.insert(owner_var.clone(), idx);
        }
    }
    for vid in &scope.variables {
        let Some(v) = ctx.variable_map.get(vid.value()).copied() else {
            continue;
        };
        let node = make_variable_node(v);
        let node_id_str = node.id().to_string();
        let idx = arena.push_node(node);
        state
            .node_id_origin_scope
            .insert(node_id_str, scope.id.value().to_string());
        arena.append_child(body_container, ElementHandle::Node(idx));
    }
    let empty_ops: Vec<super::write_op::WriteOp> = Vec::new();
    let ops_slice: &[super::write_op::WriteOp] = ctx
        .write_ops_by_scope
        .get(scope.id.value())
        .unwrap_or(&empty_ops);
    let ops_owned: Vec<super::write_op::WriteOp> = ops_slice.to_vec();
    for op in &ops_owned {
        let owner_var = ctx.variable_map.get(op.var_id.as_str()).copied();
        let declaration_kind: Option<VariableDeclarationKind> = owner_var
            .and_then(|v| v.defs.first())
            .and_then(|def| match def {
                SerializedDefinition::Variable(d) => Some(d.declaration_kind().clone()),
                _ => None,
            });
        let id = write_op_node_id(&op.ref_id);
        let node = VisualNode::Synthetic(SyntheticVisualNode {
            r#type: NodeTypeTag::Node,
            id: id.clone(),
            kind: SyntheticNodeKind::WriteReference,
            name: op.var_name.clone(),
            line: op.line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            extras: SyntheticExtras::WriteOp { declaration_kind },
        });
        let node_idx = arena.push_node(node);
        state
            .node_id_origin_scope
            .insert(id, scope.id.value().to_string());
        arena.append_child(body_container, ElementHandle::Node(node_idx));
    }
    build_children(arena, state, ctx, scope, body_container);
    if let Some(sg) = body_subgraph {
        attach_loop_test_anchor(arena, state, scope, sg);
        attach_switch_discriminant_anchor(arena, state, scope, sg);
    }
}

#[cfg(test)]
#[path = "build_scope_test.rs"]
mod build_scope_test;
