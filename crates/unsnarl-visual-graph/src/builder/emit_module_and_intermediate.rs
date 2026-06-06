//! Group every import binding (and any renamed-import intermediate)
//! under a per-source module subgraph.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::serialized::SerializedDefinition;

use crate::direction::Direction;
use crate::visual_node::SyntheticVisualNode;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, NodeIdx};
use super::context::BuilderContext;
use super::intermediate_key::intermediate_key;
use super::node_id::node_id;
use super::push_edge::push_edge;
use super::sanitize::sanitize;
use super::state::BuildState;

/// Groups every import binding (and any renamed-import intermediate)
/// under a per-source module subgraph, mirroring how functions /
/// classes / control blocks become subgraphs everywhere else. The
/// binding nodes are emitted at the module root by `build_scope`;
/// here they are re-parented into the subgraph and the old
/// `module -> binding` edge is replaced by containment. The only
/// surviving import edge is `intermediate -> local` for renamed named
/// imports, which keeps the original-name node wired to its local
/// alias inside the subgraph.
pub fn emit_module_and_intermediate(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
) {
    struct ModuleSubgraph {
        id: String,
        source: String,
        line: u32,
        children: Vec<ElementHandle>,
        // Original exported names that already have an intermediate
        // node in this subgraph, so a second alias of the same name
        // reuses it rather than emitting a duplicate.
        intermediates: HashSet<String>,
    }
    // Preserve import-source insertion order for deterministic output.
    let mut order: Vec<String> = Vec::new();
    let mut modules: HashMap<String, ModuleSubgraph> = HashMap::new();

    // Index the binding nodes `build_scope` already emitted so the
    // re-parent step can find their arena handle by id.
    let mut node_idx_by_id: HashMap<String, NodeIdx> = HashMap::new();
    for (i, n) in arena.nodes.iter().enumerate() {
        node_idx_by_id.insert(n.id().to_string(), NodeIdx(i));
    }

    // Binding handles moved off the root list into a subgraph.
    let mut moved: HashSet<NodeIdx> = HashSet::new();

    for v in &ctx.ir.variables {
        let Some(def) = v.defs.first() else {
            continue;
        };
        let (source, kind, imported_name, parent_line, node_line) = match def {
            SerializedDefinition::ImportBindingNamed(d) => (
                d.import_source().to_string(),
                "named",
                Some(d.imported_name().to_string()),
                d.parent().map(|p| p.span.line.0).unwrap_or(0),
                d.node().span.line.0,
            ),
            SerializedDefinition::ImportBindingDefault(d) => (
                d.import_source().to_string(),
                "default",
                None,
                d.parent().map(|p| p.span.line.0).unwrap_or(0),
                d.node().span.line.0,
            ),
            SerializedDefinition::ImportBindingNamespace(d) => (
                d.import_source().to_string(),
                "namespace",
                None,
                d.parent().map(|p| p.span.line.0).unwrap_or(0),
                d.node().span.line.0,
            ),
            _ => continue,
        };

        let entry = modules.entry(source.clone()).or_insert_with(|| {
            order.push(source.clone());
            ModuleSubgraph {
                id: format!("sg_{}", sanitize(&source)),
                source: source.clone(),
                line: parent_line,
                children: Vec::new(),
                intermediates: HashSet::new(),
            }
        });

        let local_id = node_id(v.id.value());
        let Some(&local_idx) = node_idx_by_id.get(&local_id) else {
            continue;
        };

        let is_renamed = kind == "named" && imported_name.as_deref().is_some_and(|n| n != v.name());
        if is_renamed {
            let name = imported_name.as_deref().unwrap_or_default();
            let key = intermediate_key(&source, name);
            let inter_id = format!("import_{}", sanitize(&key));
            if entry.intermediates.insert(name.to_string()) {
                let inter_node =
                    SyntheticVisualNode::import_intermediate(inter_id.clone(), name, node_line)
                        .into();
                let inter_idx = arena.push_node(inter_node);
                entry.children.push(ElementHandle::Node(inter_idx));
            }
            push_edge(
                &mut state.emitted_edges,
                &mut state.edges,
                &inter_id,
                "read",
                &local_id,
            );
        }
        entry.children.push(ElementHandle::Node(local_idx));
        moved.insert(local_idx);
    }

    // Drop the re-parented bindings from the root list, then attach
    // each module subgraph (with its collected children) at the root
    // in import-source order.
    arena.detach_root_nodes(&moved);

    for key in &order {
        let m = modules.remove(key).expect("module recorded in order");
        let descriptor =
            OwnedVisualSubgraph::module(m.id, m.line, m.source, Vec::new(), Direction::RL).into();
        let sg_idx = arena.push_subgraph(descriptor);
        for child in m.children {
            arena.append_child(Container::Subgraph(sg_idx), child);
        }
        arena.append_child(Container::Root, ElementHandle::Subgraph(sg_idx));
    }
}
