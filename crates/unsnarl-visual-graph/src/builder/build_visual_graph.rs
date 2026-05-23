//! Entry point for the visual-graph build. Constructs the
//! `BuilderContext` side tables off the analysed `SerializedIR`,
//! walks the module / global root scope through `build_scope`,
//! emits let-chain edges from the precomputed `WriteOp` lists,
//! processes per-reference edges (read / write / owner / predicate
//! / completion), wires module sources and import intermediates,
//! flushes pending loop-test anchors into their host subgraphs,
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
use crate::visual_edge::VisualEdge;
use crate::visual_element_type::NodeTypeTag;
use crate::visual_graph::{VisualGraph, VisualGraphSource};
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, Container, ElementHandle};
use super::branch_container_key::branch_container_key;
use super::build_scope::build_scope;
use super::context::{BuildVisualGraphOptions, BuilderContext};
use super::edge_label_of_ref::edge_label_of_ref;
use super::enclosing_function_var::enclosing_function_var;
use super::ensure_expression_statement_node::ensure_expression_statement_node;
use super::expression_statement_node_id::expression_statement_node_id;
use super::find_host_subgraph::find_host_subgraph;
use super::intermediate_key::intermediate_key;
use super::is_ancestor_scope::is_ancestor_scope;
use super::last_write_op_in_scope_before::last_write_op_in_scope_before;
use super::module_root_id::MODULE_ROOT_ID;
use super::node_id::node_id;
use super::owner_target_id::owner_target_id;
use super::predicate_target_id::{predicate_target_id, PredicateAnchorMaps};
use super::previous_fallthrough_case::previous_fallthrough_case;
use super::push_edge::push_edge;
use super::read_origins::read_origins;
use super::resolve_read_target_id::resolve_read_target_id;
use super::ret_use_node_id::ret_use_node_id;
use super::sanitize::sanitize;
use super::set_predecessor_of::set_predecessor_of;
use super::state::{BuildState, LoopTestAnchorPosition};
use super::state_ref_id::state_ref_id;
use super::throw_use_node_id::throw_use_node_id;
use super::write_op::WriteOp;
use super::write_op_node_id::write_op_node_id;

pub fn build_visual_graph(ir: &SerializedIR, opts: &BuildVisualGraphOptions) -> VisualGraph {
    let _span = tracing::info_span!("build_visual_graph").entered();
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
    };
    let mut state = BuildState::new();
    let mut arena = BuildArena::new();

    let root = ir
        .scopes
        .iter()
        .find(|v| matches!(v.r#type, ScopeType::Module | ScopeType::Global));
    if let Some(root) = root {
        let _span = tracing::info_span!("build_scope").entered();
        build_scope(&mut arena, &mut state, &ctx, root, Container::Root);
    }

    {
        let _span = tracing::info_span!("emit_let_chain_edges").entered();
        emit_let_chain_edges(&mut state, &ctx);
    }
    {
        let _span = tracing::info_span!("emit_reference_edges").entered();
        emit_reference_edges(&mut arena, &mut state, &ctx, &var_var_ids);
    }

    let needs_module_root = state.edges.iter().any(|e| e.to == MODULE_ROOT_ID);
    if needs_module_root {
        let node = VisualNode::Synthetic(SyntheticVisualNode {
            r#type: NodeTypeTag::Node,
            id: MODULE_ROOT_ID.to_string(),
            kind: SyntheticNodeKind::SyntheticModuleSink,
            name: "module".to_string(),
            line: 0,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            extras: SyntheticExtras::None {},
        });
        let idx = arena.push_node(node);
        arena.append_child(Container::Root, ElementHandle::Node(idx));
    }

    {
        let _span = tracing::info_span!("emit_module_and_intermediate").entered();
        emit_module_and_intermediate(&mut arena, &mut state, &ctx);
    }
    {
        let _span = tracing::info_span!("apply_pending_loop_test_anchors").entered();
        apply_pending_loop_test_anchors(&mut arena, &state);
    }
    {
        let _span = tracing::info_span!("mark_unused").entered();
        mark_unused(&mut arena, ctx.ir, &var_var_ids);
    }

    // Edge redirection for collapsed scopes.
    if !state.collapsed_root_by_scope.is_empty() {
        let _span = tracing::info_span!("redirect_edges_into_collapsed").entered();
        redirect_edges_into_collapsed(
            &mut state.edges,
            ir,
            &state.collapsed_root_by_scope,
            &state.collapsed_anchor_by_root,
            &state.node_id_origin_scope,
        );
    }

    let elements = {
        let _span = tracing::info_span!("arena_finalize_root").entered();
        arena.finalize_root()
    };
    super::timing::drain_and_emit();
    VisualGraph {
        version: unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION,
        source: VisualGraphSource {
            path: ir.source.path.clone(),
            language: ir.source.language,
        },
        direction: Direction::RL,
        elements,
        edges: state.edges,
        boundary_edges: Vec::new(),
        pruning: None,
    }
}

fn emit_let_chain_edges(state: &mut BuildState, ctx: &BuilderContext<'_>) {
    // `write_ops_by_variable` is seeded in `ir.variables` source
    // order. `HashMap` iteration order is not stable; walking
    // `ir.variables` here keeps the rendered edge order stable
    // against the IR parity baselines.
    for v in &ctx.ir.variables {
        let Some(ops) = ctx.write_ops_by_variable.get(v.id.value()) else {
            continue;
        };
        if ops.is_empty() {
            continue;
        }
        for i in 0..ops.len() {
            let op = &ops[i];
            let mut prev_id = node_id(&op.var_id);
            let op_scope = ctx.scope_map.get(op.scope_id.as_str()).copied();
            let op_branch_key = op_scope.and_then(branch_container_key);
            let is_first_in_case = op_scope.is_some()
                && op_branch_key
                    .as_deref()
                    .is_some_and(|k| k.starts_with("switch:"))
                && !ops[..i]
                    .iter()
                    .any(|prev_op| prev_op.scope_id == op.scope_id);
            if is_first_in_case {
                if let Some(scope) = op_scope {
                    if let Some(prev_case) =
                        previous_fallthrough_case(scope, &ctx.sorted_cases_by_container)
                    {
                        if let Some(prev_case_last) = last_write_op_in_scope_before(
                            &op.var_id,
                            prev_case.id.value(),
                            op.offset,
                            &ctx.write_ops_by_variable,
                            &ctx.scope_map,
                        ) {
                            prev_id = write_op_node_id(&prev_case_last.ref_id);
                        }
                    }
                }
            } else {
                for j in (0..i).rev() {
                    let candidate = &ops[j];
                    if is_ancestor_scope(&candidate.scope_id, &op.scope_id, &ctx.scope_map) {
                        prev_id = write_op_node_id(&candidate.ref_id);
                        break;
                    }
                }
            }
            let edge_kind = if is_first_in_case
                && op_scope
                    .and_then(|s| previous_fallthrough_case(s, &ctx.sorted_cases_by_container))
                    .is_some()
            {
                "fallthrough"
            } else {
                "set"
            };
            push_edge(
                &mut state.emitted_edges,
                &mut state.edges,
                &prev_id,
                edge_kind,
                &write_op_node_id(&op.ref_id),
            );
        }
    }
}

fn emit_reference_edges(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    var_var_ids: &HashSet<&str>,
) {
    for r in &ctx.ir.references {
        let Some(resolved) = r.resolved.as_ref() else {
            continue;
        };
        if var_var_ids.contains(resolved.value()) {
            continue;
        }

        // Refs whose containing scope (or any ancestor) was collapsed.
        if let Some(collapsed_root) = state.collapsed_root_by_scope.get(r.from.value()).cloned() {
            if r.flags.write {
                continue;
            }
            let Some(target) = state.collapsed_anchor_by_root.get(&collapsed_root).cloned() else {
                continue;
            };
            let from_ids = read_origins(
                resolved.value(),
                r.identifier.span().offset.0,
                r.from.value(),
                ctx,
            );
            let label = edge_label_of_ref(r);
            for from_id in &from_ids {
                push_edge(
                    &mut state.emitted_edges,
                    &mut state.edges,
                    from_id,
                    label,
                    &target,
                );
            }
            continue;
        }

        // Predicate-anchor targets.
        let anchors = PredicateAnchorMaps {
            if_test: &state.if_test_anchor_by_offset,
            switch_discriminant: &state.switch_discriminant_anchor_by_offset,
            while_test: &state.while_test_anchor_by_offset,
            do_while_test: &state.do_while_test_anchor_by_offset,
            for_test: &state.for_test_anchor_by_offset,
        };
        let predicate_target = predicate_target_id(r, &anchors);
        if let Some(target) = predicate_target.as_ref() {
            if !r.flags.write {
                let from_ids = read_origins(
                    resolved.value(),
                    r.identifier.span().offset.0,
                    r.from.value(),
                    ctx,
                );
                let label = edge_label_of_ref(r);
                for from_id in &from_ids {
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        from_id,
                        label,
                        target,
                    );
                }
                continue;
            }
        }
        if predicate_target.is_none() && r.predicate_container.is_some() && !r.flags.write {
            if let Some(pc) = r.predicate_container.as_ref() {
                if let Some(redirect) = state
                    .suppressed_predicate_redirect
                    .get(&pc.offset.0)
                    .cloned()
                {
                    let from_ids = read_origins(
                        resolved.value(),
                        r.identifier.span().offset.0,
                        r.from.value(),
                        ctx,
                    );
                    let label = edge_label_of_ref(r);
                    for from_id in &from_ids {
                        push_edge(
                            &mut state.emitted_edges,
                            &mut state.edges,
                            from_id,
                            label,
                            &redirect,
                        );
                    }
                }
            }
            continue;
        }

        if r.flags.write {
            if r.flags.call || (r.flags.read && !r.owners.is_empty()) {
                let from_id = state_ref_id(r.id.value(), resolved.value(), ctx);
                let label = edge_label_of_ref(r);
                for owner_id in &r.owners {
                    if owner_id.value() == resolved.value() {
                        continue;
                    }
                    let target_id = owner_target_id(
                        owner_id.value(),
                        r.identifier.span().offset.0,
                        &ctx.write_ops_by_variable,
                    );
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        &from_id,
                        label,
                        &target_id,
                    );
                }
            }
            if r.flags.read {
                if let Some(op) = ctx.write_op_by_ref.get(r.id.value()) {
                    let wr_target_id = write_op_node_id(r.id.value());
                    let set_pred_id = set_predecessor_of(
                        op,
                        ctx.write_ops_by_variable
                            .get(resolved.value())
                            .map(Vec::as_slice)
                            .unwrap_or(&[]),
                        &ctx.scope_map,
                    );
                    let from_ids = read_origins(
                        resolved.value(),
                        r.identifier.span().offset.0,
                        r.from.value(),
                        ctx,
                    );
                    for from_id in &from_ids {
                        if from_id == &set_pred_id || from_id == &wr_target_id {
                            continue;
                        }
                        push_edge(
                            &mut state.emitted_edges,
                            &mut state.edges,
                            from_id,
                            "read",
                            &wr_target_id,
                        );
                    }
                }
            }
            continue;
        }

        // Pure read (no write flag).
        let label = edge_label_of_ref(r);
        let from_ids = read_origins(
            resolved.value(),
            r.identifier.span().offset.0,
            r.from.value(),
            ctx,
        );
        if !r.owners.is_empty() {
            for owner_id in &r.owners {
                if owner_id.value() == resolved.value() {
                    continue;
                }
                let target_id = owner_target_id(
                    owner_id.value(),
                    r.identifier.span().offset.0,
                    &ctx.write_ops_by_variable,
                );
                for from_id in &from_ids {
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        from_id,
                        label,
                        &target_id,
                    );
                }
            }
        } else {
            let enclosing_fn_var_id =
                enclosing_function_var(r.from.value(), &ctx.scope_map, &ctx.subgraph_owner_var);
            let host = find_host_subgraph(r, enclosing_fn_var_id.as_deref(), &ctx.scope_map, state);
            let target_container = match host {
                Some(sg) => Container::Subgraph(sg),
                None => Container::Root,
            };
            let expr_stmt_id = ensure_expression_statement_node(
                arena,
                state,
                r,
                &ctx.source_index,
                target_container,
            );
            let target_id = resolve_read_target_id(
                arena,
                state,
                ctx,
                expr_stmt_id.as_deref(),
                enclosing_fn_var_id.as_deref(),
                r,
            );
            for from_id in &from_ids {
                push_edge(
                    &mut state.emitted_edges,
                    &mut state.edges,
                    from_id,
                    label,
                    &target_id,
                );
            }
        }
    }
}

fn emit_module_and_intermediate(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
) {
    struct ModuleNode {
        id: String,
        line: u32,
        source: String,
    }
    struct Intermediate {
        id: String,
        name: String,
        line: u32,
    }
    // Preserve insertion order for deterministic on-disk output.
    let mut module_nodes_order: Vec<String> = Vec::new();
    let mut module_nodes: HashMap<String, ModuleNode> = HashMap::new();
    let mut intermediates_order: Vec<String> = Vec::new();
    let mut intermediates: HashMap<String, Intermediate> = HashMap::new();

    for v in &ctx.ir.variables {
        let Some(def) = v.defs.first() else {
            continue;
        };
        let (import_source, import_kind, imported_name, parent_line, node_line) = match def {
            SerializedDefinition::ImportBindingNamed(d) => (
                Some(d.import_source().to_string()),
                Some("named"),
                Some(d.imported_name().to_string()),
                d.parent().map(|p| p.span.line.0).unwrap_or(0),
                d.node().span.line.0,
            ),
            SerializedDefinition::ImportBindingDefault(d) => (
                Some(d.import_source().to_string()),
                Some("default"),
                None,
                d.parent().map(|p| p.span.line.0).unwrap_or(0),
                d.node().span.line.0,
            ),
            SerializedDefinition::ImportBindingNamespace(d) => (
                Some(d.import_source().to_string()),
                Some("namespace"),
                None,
                d.parent().map(|p| p.span.line.0).unwrap_or(0),
                d.node().span.line.0,
            ),
            _ => (None, None, None, 0, 0),
        };
        let Some(source) = import_source else {
            continue;
        };
        if let std::collections::hash_map::Entry::Vacant(slot) = module_nodes.entry(source.clone())
        {
            module_nodes_order.push(source.clone());
            slot.insert(ModuleNode {
                id: format!("mod_{}", sanitize(&source)),
                line: parent_line,
                source: source.clone(),
            });
        }
        if import_kind == Some("named") {
            if let Some(name) = imported_name.as_ref() {
                if Some(v.name()) != Some(name.as_str()) {
                    let key = intermediate_key(&source, name);
                    if let std::collections::hash_map::Entry::Vacant(slot) =
                        intermediates.entry(key.clone())
                    {
                        intermediates_order.push(key.clone());
                        slot.insert(Intermediate {
                            id: format!("import_{}", sanitize(&key)),
                            name: name.clone(),
                            line: node_line,
                        });
                    }
                }
            }
        }
    }

    for key in &module_nodes_order {
        let m = &module_nodes[key];
        let node = VisualNode::Synthetic(SyntheticVisualNode {
            r#type: NodeTypeTag::Node,
            id: m.id.clone(),
            kind: SyntheticNodeKind::SyntheticModuleSource,
            name: m.source.clone(),
            line: m.line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            extras: SyntheticExtras::None {},
        });
        let idx = arena.push_node(node);
        arena.append_child(Container::Root, ElementHandle::Node(idx));
    }
    for key in &intermediates_order {
        let inter = &intermediates[key];
        let node = VisualNode::Synthetic(SyntheticVisualNode {
            r#type: NodeTypeTag::Node,
            id: inter.id.clone(),
            kind: SyntheticNodeKind::SyntheticImportIntermediate,
            name: inter.name.clone(),
            line: inter.line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            extras: SyntheticExtras::None {},
        });
        let idx = arena.push_node(node);
        arena.append_child(Container::Root, ElementHandle::Node(idx));
    }
    for v in &ctx.ir.variables {
        let Some(def) = v.defs.first() else {
            continue;
        };
        let (source, kind, imported) = match def {
            SerializedDefinition::ImportBindingNamed(d) => (
                d.import_source().to_string(),
                "named",
                Some(d.imported_name().to_string()),
            ),
            SerializedDefinition::ImportBindingDefault(d) => {
                (d.import_source().to_string(), "default", None)
            }
            SerializedDefinition::ImportBindingNamespace(d) => {
                (d.import_source().to_string(), "namespace", None)
            }
            _ => continue,
        };
        let Some(mod_node) = module_nodes.get(&source) else {
            continue;
        };
        let local_id = node_id(v.id.value());
        let is_renamed = kind == "named" && imported.as_deref().is_some_and(|n| n != v.name());
        if is_renamed {
            if let Some(name) = imported.as_ref() {
                let key = intermediate_key(&source, name);
                if let Some(inter) = intermediates.get(&key) {
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        &mod_node.id,
                        "read",
                        &inter.id,
                    );
                    push_edge(
                        &mut state.emitted_edges,
                        &mut state.edges,
                        &inter.id,
                        "read",
                        &local_id,
                    );
                    continue;
                }
            }
        }
        push_edge(
            &mut state.emitted_edges,
            &mut state.edges,
            &mod_node.id,
            "read",
            &local_id,
        );
    }
}

fn apply_pending_loop_test_anchors(arena: &mut BuildArena, state: &BuildState) {
    for pending in &state.pending_loop_test_anchors {
        let container = Container::Subgraph(pending.subgraph);
        let handle = ElementHandle::Node(pending.node);
        match pending.position {
            LoopTestAnchorPosition::First => arena.prepend_child(container, handle),
            LoopTestAnchorPosition::Last => arena.append_child(container, handle),
        }
    }
}

fn mark_unused(arena: &mut BuildArena, ir: &SerializedIR, var_var_ids: &HashSet<&str>) {
    for id in &ir.unused_variable_ids {
        if var_var_ids.contains(id.value()) {
            continue;
        }
        let target = node_id(id.value());
        for node in arena.nodes.iter_mut() {
            if node.id() == target {
                node.set_unused(true);
                break;
            }
        }
    }
}

fn redirect_edges_into_collapsed(
    edges: &mut Vec<VisualEdge>,
    ir: &SerializedIR,
    collapsed_root_by_scope: &HashMap<String, String>,
    collapsed_anchor_by_root: &HashMap<String, String>,
    node_id_origin_scope: &HashMap<String, String>,
) {
    let mut origin_scope_by_node_id: HashMap<String, String> = node_id_origin_scope.clone();
    // Variables: include every variable, even those whose nodes were
    // never emitted because they live inside a collapsed scope.
    for v in &ir.variables {
        let id = node_id(v.id.value());
        origin_scope_by_node_id
            .entry(id)
            .or_insert_with(|| v.scope.value().to_string());
    }
    // References whose nodes (write op / return-use / throw-use /
    // expression statement) were never created because their
    // containing scope collapsed.
    for r in &ir.references {
        let from = r.from.value().to_string();
        let wid = write_op_node_id(r.id.value());
        origin_scope_by_node_id.entry(wid).or_insert(from.clone());
        let ruid = ret_use_node_id(r.id.value());
        origin_scope_by_node_id.entry(ruid).or_insert(from.clone());
        let tuid = throw_use_node_id(r.id.value());
        origin_scope_by_node_id.entry(tuid).or_insert(from.clone());
        if let Some(c) = r.expression_statement_container.as_ref() {
            let sid = expression_statement_node_id(c.start_span.offset.0);
            origin_scope_by_node_id.entry(sid).or_insert(from);
        }
    }

    let redirect = |id: &str| -> Option<String> {
        let scope = origin_scope_by_node_id.get(id);
        let Some(scope) = scope else {
            return Some(id.to_string());
        };
        let root = collapsed_root_by_scope.get(scope);
        let Some(root) = root else {
            return Some(id.to_string());
        };
        collapsed_anchor_by_root.get(root).cloned()
    };

    let original = std::mem::take(edges);
    let mut seen: HashSet<String> = HashSet::new();
    for e in original {
        let from = match redirect(&e.from) {
            Some(s) => s,
            None => continue,
        };
        let to = match redirect(&e.to) {
            Some(s) => s,
            None => continue,
        };
        if from == to {
            continue;
        }
        let key = format!("{from}\t{to}\t{}", e.label);
        if !seen.insert(key) {
            continue;
        }
        edges.push(VisualEdge {
            from,
            to,
            label: e.label,
        });
    }
}
