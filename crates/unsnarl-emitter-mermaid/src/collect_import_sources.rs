//! Selects the set of ids whose outgoing edges should be rendered
//! in the import-edge bucket.
//!
//! Mirrors `ts/src/emitter/mermaid/collect-import-sources.ts`.

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
