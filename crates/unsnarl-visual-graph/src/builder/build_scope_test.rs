//! Sibling tests for [`build_scope`].

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::serialized_definition::{
    DefinitionName, DefinitionNode, SerializedDefinition, SimpleDef, SimpleDefType, VariableDef,
};
use unsnarl_ir::serialized::serialized_scope::SerializedBlock;
use unsnarl_ir::serialized::SerializedVariable;
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind};

use super::build_scope;
use crate::builder::arena::{BuildArena, Container, ElementHandle, NodeIdx, SubgraphIdx};
use crate::builder::builder_fixtures::{
    base_builder_context, base_def, base_serialized_scope, base_serialized_variable, base_write_op,
    empty_serialized_ir, other_block_context, scope_id, span_offset_line, variable_id,
};
use crate::builder::state::BuildState;
use crate::builder::write_op::WriteOp;
use crate::visual_node::{BindingNodeKind, SyntheticExtras, SyntheticNodeKind, VisualNode};
use crate::visual_subgraph::{ControlSubgraphKind, OwnedSubgraphKind, VisualSubgraph};

fn block(
    r#type: AstType,
    span_offset: u32,
    span_line: u32,
    end_offset: u32,
    end_line: u32,
) -> SerializedBlock {
    SerializedBlock {
        r#type,
        span: span_offset_line(span_offset, span_line),
        end_span: span_offset_line(end_offset, end_line),
    }
}

fn first_root_subgraph(arena: &BuildArena) -> SubgraphIdx {
    match arena.root_children[0] {
        ElementHandle::Subgraph(idx) => idx,
        _ => panic!("expected subgraph child"),
    }
}

fn collect_subgraph_node_ids(arena: &BuildArena, sg: SubgraphIdx) -> Vec<(String, NodeIdx)> {
    arena
        .subgraph(sg)
        .children
        .iter()
        .filter_map(|h| match h {
            ElementHandle::Node(idx) => Some((arena.node(*idx).id().to_string(), *idx)),
            _ => None,
        })
        .collect()
}

#[test]
fn plain_block_scope_wraps_variables_in_block_subgraph() {
    let mut ir = empty_serialized_ir();
    let mut scope = base_serialized_scope("s");
    scope.variables = vec![variable_id("v1")];
    ir.scopes.push(scope);
    let mut v = base_serialized_variable();
    v.id = variable_id("v1");
    ir.variables.push(v);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_scope(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    assert_eq!(arena.root_children.len(), 1);
    let sg_idx = first_root_subgraph(&arena);
    let descriptor = arena.subgraph(sg_idx).descriptor.clone();
    let VisualSubgraph::Control(c) = descriptor else {
        panic!("expected control");
    };
    assert!(matches!(c.kind, ControlSubgraphKind::Block));

    let nodes = collect_subgraph_node_ids(&arena, sg_idx);
    let (_, node_idx) = nodes.iter().find(|(id, _)| id == "n_v1").expect("n_v1");
    let VisualNode::Binding(b) = arena.node(*node_idx).clone() else {
        panic!("expected binding");
    };
    assert!(matches!(b.kind, BindingNodeKind::LetBinding));
    assert_eq!(b.name, "x");
}

#[test]
fn write_ops_in_scope_appear_with_declaration_kind() {
    let mut ir = empty_serialized_ir();
    let mut scope = base_serialized_scope("s");
    scope.variables = vec![variable_id("v1")];
    ir.scopes.push(scope);
    let mut v = base_serialized_variable();
    v.id = variable_id("v1");
    v.defs = vec![base_def(VariableDeclarationKind::Let)];
    ir.variables.push(v);
    let mut ctx = base_builder_context(&ir);
    let op = WriteOp {
        ref_id: "r1".to_string(),
        var_id: "v1".to_string(),
        var_name: "x".to_string(),
        line: 4,
        offset: 0,
        scope_id: "s".to_string(),
    };
    ctx.write_ops_by_scope.insert("s".to_string(), vec![op]);

    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_scope(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sg_idx = first_root_subgraph(&arena);
    let nodes = collect_subgraph_node_ids(&arena, sg_idx);
    let (_, wr_idx) = nodes.iter().find(|(id, _)| id == "wr_r1").expect("wr_r1");
    let VisualNode::Synthetic(s) = arena.node(*wr_idx).clone() else {
        panic!("expected synthetic");
    };
    assert!(matches!(s.kind, SyntheticNodeKind::WriteReference));
    assert_eq!(s.name, "x");
    assert_eq!(s.line, 4);
    let SyntheticExtras::WriteOp { declaration_kind } = s.extras else {
        panic!("expected WriteOp extras");
    };
    assert!(matches!(
        declaration_kind,
        Some(VariableDeclarationKind::Let)
    ));
    let _ = base_write_op; // silence unused
}

#[test]
fn function_owner_scope_wraps_body_in_function_subgraph_and_registers_state() {
    let mut ir = empty_serialized_ir();
    let mut fn_scope = base_serialized_scope("fn");
    fn_scope.r#type = ScopeType::Function;
    fn_scope.variables = vec![variable_id("param")];
    fn_scope.block = block(AstType::FunctionDeclaration, 0, 1, 10, 5);
    ir.scopes.push(fn_scope);
    let param = SerializedVariable::new(
        variable_id("param"),
        "p".to_string(),
        scope_id("s"),
        vec![span_offset_line(0, 1)],
        Vec::new(),
        vec![SerializedDefinition::Simple(SimpleDef {
            name: DefinitionName::new("p".to_string(), span_offset_line(0, 1)),
            node: DefinitionNode {
                r#type: AstType::Identifier,
                span: span_offset_line(0, 1),
            },
            parent: None,
            r#type: SimpleDefType::Parameter,
        })],
    );
    let owner = SerializedVariable::new(
        variable_id("ownerVar"),
        "myFn".to_string(),
        scope_id("s"),
        vec![span_offset_line(0, 1)],
        Vec::new(),
        vec![SerializedDefinition::Variable(VariableDef::new(
            DefinitionName::new("myFn".to_string(), span_offset_line(0, 1)),
            DefinitionNode {
                r#type: AstType::Identifier,
                span: span_offset_line(0, 1),
            },
            None,
            None,
            VariableDeclarationKind::Let,
        ))],
    );
    ir.variables.push(param);
    ir.variables.push(owner);

    let mut ctx = base_builder_context(&ir);
    ctx.subgraph_owner_var
        .insert("fn".to_string(), "ownerVar".to_string());

    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_scope(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sg_idx = first_root_subgraph(&arena);
    let VisualSubgraph::Owned(o) = arena.subgraph(sg_idx).descriptor.clone() else {
        panic!("expected owned");
    };
    assert!(matches!(o.kind, OwnedSubgraphKind::Function));
    let nodes = collect_subgraph_node_ids(&arena, sg_idx);
    assert!(nodes.iter().any(|(id, _)| id == "n_param"));
    assert_eq!(state.subgraph_by_scope.get("fn"), Some(&sg_idx));
    assert_eq!(state.function_subgraph_by_fn.get("ownerVar"), Some(&sg_idx));
}

#[test]
fn control_kind_scope_for_wraps_body_in_control_subgraph() {
    let mut ir = empty_serialized_ir();
    let mut for_scope = base_serialized_scope("for1");
    for_scope.r#type = ScopeType::For;
    for_scope.variables = vec![variable_id("v")];
    for_scope.block = block(AstType::ForStatement, 0, 1, 10, 3);
    ir.scopes.push(for_scope);
    let mut v = base_serialized_variable();
    v.id = variable_id("v");
    ir.variables.push(v);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_scope(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    let sg_idx = first_root_subgraph(&arena);
    let VisualSubgraph::Control(c) = arena.subgraph(sg_idx).descriptor.clone() else {
        panic!("expected control");
    };
    assert!(matches!(c.kind, ControlSubgraphKind::For));
    let nodes = collect_subgraph_node_ids(&arena, sg_idx);
    assert!(nodes.iter().any(|(id, _)| id == "n_v"));
}

#[test]
fn recurses_into_child_scopes() {
    let mut ir = empty_serialized_ir();
    let mut outer = base_serialized_scope("outer");
    outer.r#type = ScopeType::Module;
    outer.child_scopes = vec![scope_id("inner")];
    ir.scopes.push(outer);
    let mut inner = base_serialized_scope("inner");
    inner.upper = Some(scope_id("outer"));
    inner.variables = vec![variable_id("vIn")];
    inner.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        0,
        None,
    ));
    ir.scopes.push(inner);
    let mut v_in = base_serialized_variable();
    v_in.id = variable_id("vIn");
    ir.variables.push(v_in);

    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    build_scope(&mut arena, &mut state, &ctx, &ir.scopes[0], Container::Root);

    // Outer is Module so it is NOT wrapped in a subgraph. The inner if
    // subgraph lives directly at root.
    let kinds: Vec<_> = arena
        .root_children
        .iter()
        .filter_map(|h| match h {
            ElementHandle::Subgraph(idx) => {
                if let VisualSubgraph::Control(c) = arena.subgraph(*idx).descriptor.clone() {
                    Some(c.kind)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();
    assert!(kinds.iter().any(|k| matches!(k, ControlSubgraphKind::If)));
}
