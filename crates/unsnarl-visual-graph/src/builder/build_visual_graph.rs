//! Entry point for the visual-graph build. Constructs the
//! `BuilderContext` side tables off the analysed `SerializedIR`,
//! walks the module / global root scope through `build_scope`,
//! emits let-chain edges from the precomputed `WriteOp` lists,
//! processes per-reference edges (read / write / owner / predicate
//! / completion), groups import bindings into per-source module
//! subgraphs, flushes pending loop-test anchors into their host subgraphs,
//! marks unused variables, and finally collapses edges that crossed
//! the depth-pruning boundary.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::{
    SerializedDefinition, SerializedIR, SerializedReference, SerializedScope, SerializedVariable,
    SimpleDefType,
};
use unsnarl_oxc_parity::AstType;

use crate::direction::Direction;
use crate::visual_graph::VisualGraph;
use crate::visual_node::SyntheticVisualNode;

use super::apply_pending_loop_test_anchors::apply_pending_loop_test_anchors;
use super::arena::{BuildArena, Container, ElementHandle};
use super::branch_container_key::branch_container_key;
use super::build_scope::build_scope;
use super::context::{BuildVisualGraphOptions, BuilderContext};
use super::emit_let_chain_edges::emit_let_chain_edges;
use super::emit_module_and_intermediate::emit_module_and_intermediate;
use super::emit_reference_edges::emit_reference_edges;
use super::expression_statement_index::ExpressionStatementIndex;
use super::mark_unused::mark_unused;
use super::module_root_id::MODULE_ROOT_ID;
use super::redirect_edges_into_collapsed::redirect_edges_into_collapsed;
use super::state::BuildState;
use super::write_op::WriteOp;

pub fn build_visual_graph(ir: &SerializedIR, opts: &BuildVisualGraphOptions) -> VisualGraph {
    let _span = unsnarl_instrumentation::span!("build_visual_graph");
    let mut variable_map: HashMap<&str, &SerializedVariable> = HashMap::new();
    for v in &ir.variables {
        variable_map.insert(v.id.value(), v);
    }
    let mut scope_map: HashMap<&str, &SerializedScope> = HashMap::new();
    for s in &ir.scopes {
        scope_map.insert(s.id.value(), s);
    }

    // `var` declarations remain visible as nodes (via scope.variables)
    // but their references are excluded from edge / WriteOp emission.
    let var_var_ids: HashSet<&str> = ir
        .variables
        .iter()
        .filter_map(|v| match v.defs.first() {
            Some(SerializedDefinition::Variable(d))
                if matches!(
                    d.declaration_kind(),
                    unsnarl_oxc_parity::VariableDeclarationKind::Var
                ) =>
            {
                Some(v.id.value())
            }
            _ => None,
        })
        .collect();

    let mut subgraph_owner_var: HashMap<String, String> = HashMap::new();
    for variable in &ir.variables {
        let Some(def) = variable.defs.first() else {
            continue;
        };
        let block_offset: Option<u32> = match def {
            SerializedDefinition::Simple(s) if matches!(s.r#type, SimpleDefType::FunctionName) => {
                Some(s.node.span.offset.0)
            }
            SerializedDefinition::Variable(d) => {
                let Some(init) = d.init() else {
                    continue;
                };
                if matches!(
                    init.r#type,
                    AstType::FunctionExpression | AstType::ArrowFunctionExpression
                ) {
                    Some(init.span.offset.0)
                } else {
                    None
                }
            }
            _ => None,
        };
        let Some(block_offset) = block_offset else {
            continue;
        };
        let Some(fn_scope) = ir.scopes.iter().find(|v| {
            v.upper.as_ref().map(|u| u.value()) == Some(variable.scope.value())
                && v.block.span.offset.0 == block_offset
        }) else {
            continue;
        };
        subgraph_owner_var.insert(
            fn_scope.id.value().to_string(),
            variable.id.value().to_string(),
        );
    }

    let mut refs_by_variable: HashMap<&str, Vec<&SerializedReference>> = HashMap::new();
    for r in &ir.references {
        let Some(resolved) = r.resolved.as_ref() else {
            continue;
        };
        if var_var_ids.contains(resolved.value()) {
            continue;
        }
        refs_by_variable
            .entry(resolved.value())
            .or_default()
            .push(r);
    }
    for arr in refs_by_variable.values_mut() {
        arr.sort_by_key(|r| r.identifier.span().offset.0);
    }

    let mut write_ops_by_variable: HashMap<String, Vec<WriteOp>> = HashMap::new();
    let mut write_ops_by_scope: HashMap<String, Vec<WriteOp>> = HashMap::new();
    let mut write_op_by_ref: HashMap<String, WriteOp> = HashMap::new();
    for v in &ir.variables {
        let refs = refs_by_variable
            .get(v.id.value())
            .cloned()
            .unwrap_or_default();
        let mut ops: Vec<WriteOp> = Vec::new();
        for r in refs {
            if !r.flags.write {
                continue;
            }
            if r.init {
                // The init Write reference is the binding's initial PutValue.
                // The Variable node itself stands in for the declaration, so
                // emitting a WriteOp here would double-count it.
                continue;
            }
            let op = WriteOp {
                ref_id: r.id.value().to_string(),
                var_id: v.id.value().to_string(),
                var_name: v.name().to_string(),
                line: r.identifier.span().line.0,
                offset: r.identifier.span().offset.0,
                scope_id: r.from.value().to_string(),
            };
            ops.push(op.clone());
            write_op_by_ref.insert(r.id.value().to_string(), op.clone());
            write_ops_by_scope
                .entry(r.from.value().to_string())
                .or_default()
                .push(op);
        }
        if !ops.is_empty() {
            write_ops_by_variable.insert(v.id.value().to_string(), ops);
        }
    }

    let mut sorted_cases_by_container: HashMap<String, Vec<&SerializedScope>> = HashMap::new();
    let mut branch_scopes_by_container: HashMap<String, Vec<&SerializedScope>> = HashMap::new();
    for s in &ir.scopes {
        let Some(ckey) = branch_container_key(s) else {
            continue;
        };
        if ckey.starts_with("switch:") {
            sorted_cases_by_container
                .entry(ckey.clone())
                .or_default()
                .push(s);
        }
        branch_scopes_by_container.entry(ckey).or_default().push(s);
    }
    for arr in sorted_cases_by_container.values_mut() {
        arr.sort_by_key(|s| s.block.span.offset.0);
    }

    let expression_statement_index = ExpressionStatementIndex::build(&ir.references);

    let ctx = BuilderContext {
        ir,
        variable_map,
        scope_map,
        subgraph_owner_var,
        write_ops_by_variable,
        write_ops_by_scope,
        write_op_by_ref,
        sorted_cases_by_container,
        branch_scopes_by_container,
        depths: opts.depths.clone(),
        source_index: SourceIndex::build(&ir.raw),
        expression_statement_index,
    };
    let mut state = BuildState::new();
    let mut arena = BuildArena::new();

    let root = ir
        .scopes
        .iter()
        .find(|v| matches!(v.r#type, ScopeType::Module | ScopeType::Global));
    if let Some(root) = root {
        let _span = unsnarl_instrumentation::span!("build_scope");
        build_scope(&mut arena, &mut state, &ctx, root, Container::Root);
    }

    {
        let _span = unsnarl_instrumentation::span!("emit_let_chain_edges");
        emit_let_chain_edges(&mut state, &ctx);
    }
    {
        let _span = unsnarl_instrumentation::span!("emit_reference_edges");
        emit_reference_edges(&mut arena, &mut state, &ctx, &var_var_ids);
    }

    let needs_module_root = state.edges.iter().any(|e| e.to == MODULE_ROOT_ID);
    if needs_module_root {
        let node = SyntheticVisualNode::module_sink(MODULE_ROOT_ID, "module", 0).into();
        let idx = arena.push_node(node);
        arena.append_child(Container::Root, ElementHandle::Node(idx));
    }

    {
        let _span = unsnarl_instrumentation::span!("emit_module_and_intermediate");
        emit_module_and_intermediate(&mut arena, &mut state, &ctx);
    }
    {
        let _span = unsnarl_instrumentation::span!("apply_pending_loop_test_anchors");
        apply_pending_loop_test_anchors(&mut arena, &state);
    }
    {
        let _span = unsnarl_instrumentation::span!("mark_unused");
        mark_unused(&mut arena, ctx.ir, &var_var_ids);
    }

    // Edge redirection for collapsed scopes.
    if !state.collapsed_root_by_scope.is_empty() {
        let _span = unsnarl_instrumentation::span!("redirect_edges_into_collapsed");
        redirect_edges_into_collapsed(
            &mut state.edges,
            ir,
            &state.collapsed_root_by_scope,
            &state.collapsed_anchor_by_root,
            &state.node_id_origin_scope,
        );
    }

    let elements = {
        let _span = unsnarl_instrumentation::span!("arena_finalize_root");
        arena.finalize_root()
    };
    unsnarl_instrumentation::drain_and_emit!("build_visual_graph::timing");
    VisualGraph::new(
        ir.source.path.clone(),
        ir.source.language,
        Direction::RL,
        elements,
        state.edges,
        Vec::new(),
    )
}
