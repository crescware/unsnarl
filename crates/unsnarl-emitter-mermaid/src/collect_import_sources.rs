//! Selects the set of ids whose outgoing edges should be rendered
//! in the import-edge bucket.

use std::collections::{HashMap, HashSet};

use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

pub fn collect_import_sources(node_map: &HashMap<String, &VisualNode>) -> HashSet<String> {
    let mut out = HashSet::new();
    for (id, n) in node_map {
        match n.kind() {
            NodeKind::SyntheticModuleSource | NodeKind::SyntheticImportIntermediate => {
                out.insert(id.clone());
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
#[path = "collect_import_sources_test.rs"]
mod collect_import_sources_test;
