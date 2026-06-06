//! Redirect edges that crossed the depth-pruning boundary onto the
//! collapsed scope's anchor node, de-duplicating the result.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::serialized::SerializedIR;

use crate::visual_edge::VisualEdge;

use super::expression_statement_node_id::expression_statement_node_id;
use super::node_id::node_id;
use super::ret_use_node_id::ret_use_node_id;
use super::throw_use_node_id::throw_use_node_id;
use super::write_op_node_id::write_op_node_id;

pub fn redirect_edges_into_collapsed(
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
        edges.push(VisualEdge::new(from, to, e.label));
    }
}
