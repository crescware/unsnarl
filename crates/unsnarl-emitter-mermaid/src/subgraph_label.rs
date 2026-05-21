//! Builds the human-readable label for a subgraph (e.g.
//! `foo()<br/>L3-12`).
//!
//! Mirrors `ts/src/emitter/mermaid/subgraph-label.ts`.

use std::collections::HashMap;

use unsnarl_visual_graph::subgraph_kind::SubgraphKind;
use unsnarl_visual_graph::visual_node::VisualNode;
use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

use crate::escape::escape;
use crate::line_range_label::line_range_label;

pub fn subgraph_label(
    sg: &VisualSubgraph,
    node_map: &HashMap<String, &VisualNode>,
    debug: bool,
) -> String {
    let base = base_label(sg, node_map);
    if debug {
        format!("{base}<br/>{}", sg.kind().as_str())
    } else {
        base
    }
}

fn base_label(sg: &VisualSubgraph, node_map: &HashMap<String, &VisualNode>) -> String {
    let range = line_range_label(sg);
    match sg.kind() {
        SubgraphKind::Function => {
            // Prefer the name baked onto the subgraph at build
            // time; the owner node may be absent after pruning even
            // when the subgraph survives. ownerName is empty when
            // the owner variable was missing at build time -- fall
            // back to the live node_map entry in that case.
            let owner_node_id = sg.owner_node_id();
            if owner_node_id.is_none() {
                return format!("(anonymous)<br/>{range}");
            }
            let owner_name = sg.owner_name().unwrap_or("");
            let resolved = if !owner_name.is_empty() {
                owner_name.to_string()
            } else if let Some(id) = owner_node_id {
                node_map
                    .get(id)
                    .map(|n| n.name().to_string())
                    .unwrap_or_default()
            } else {
                String::new()
            };
            format!("{}()<br/>{range}", escape(&resolved))
        }
        SubgraphKind::Class => match sg.class_name() {
            None => format!("class (anonymous)<br/>{range}"),
            Some(name) => format!("class {}<br/>{range}", escape(name)),
        },
        SubgraphKind::Switch => format!("switch {range}"),
        SubgraphKind::Case => match sg.case_test() {
            None => format!("default {range}"),
            Some(test) => format!("case {} {range}", escape(test)),
        },
        SubgraphKind::If => format!("if {range}"),
        SubgraphKind::Else => format!("else {range}"),
        SubgraphKind::IfElseContainer => {
            if sg.has_else() {
                format!("if-else {range}")
            } else {
                format!("if {range}")
            }
        }
        SubgraphKind::Try => format!("try {range}"),
        SubgraphKind::Catch => format!("catch {range}"),
        SubgraphKind::Finally => format!("finally {range}"),
        SubgraphKind::For => format!("for {range}"),
        SubgraphKind::While => format!("while {range}"),
        SubgraphKind::DoWhile => format!("do-while {range}"),
        SubgraphKind::Return => format!("return {range}"),
        SubgraphKind::Throw => format!("throw {range}"),
        SubgraphKind::Block => format!("block {range}"),
    }
}

#[cfg(test)]
#[path = "subgraph_label_test.rs"]
mod subgraph_label_test;
