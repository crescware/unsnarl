//! `StatsEmitter`: renders a `SerializedIR` as a wc-like TSV table
//! of per-node edge counts (`descendants  ancestors  path:line label`).
//!
//! Mirrors `StatsEmitter` in `ts/src/emitter/stats/stats.ts`. The
//! TS implementation builds a `VisualGraph` from the IR (or pulls a
//! pre-pruned graph from `opts.prunedGraph`) and walks it to count
//! edges per node, rendering each direction as `?` for nodes that
//! touch a boundary edge. The Rust port follows the same control
//! flow; the pruned-graph short-circuit lands alongside Step 17
//! when `EmitOptions` starts attaching `prunedGraph`.

use std::collections::{HashMap, HashSet};

use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_visual_graph::builder::build_visual_graph::build_visual_graph;
use unsnarl_visual_graph::builder::context::BuildVisualGraphOptions;
use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;
use unsnarl_visual_graph::visual_graph::VisualGraph;

use crate::collect_nodes::collect_nodes;
use crate::format_label::format_label;

pub struct StatsEmitter;

impl StatsEmitter {
    pub const FORMAT: &'static str = "stats";
    pub const CONTENT_TYPE: &'static str = "text/tab-separated-values";
    pub const EXTENSION: &'static str = "tsv";
}

impl Emitter for StatsEmitter {
    fn format(&self) -> &'static str {
        Self::FORMAT
    }

    fn content_type(&self) -> &'static str {
        Self::CONTENT_TYPE
    }

    fn extension(&self) -> &'static str {
        Self::EXTENSION
    }

    fn emit(&self, ir: &SerializedIR, opts: &EmitOptions) -> String {
        let built;
        let graph = if let Some(pruned) = &opts.pruned_graph {
            pruned
        } else {
            built = build_visual_graph(ir, &BuildVisualGraphOptions::default());
            &built
        };
        render_stats(graph)
    }
}

fn render_stats(graph: &VisualGraph) -> String {
    // Source-order sort so the rows read top-to-bottom like the
    // file itself: editors that pick up `path:line` jump targets
    // land on the right place, and same-line ties keep their
    // original (preorder-walk) order via the sort being stable.
    let mut nodes = collect_nodes(&graph.elements);
    nodes.sort_by_key(|n| n.line());

    let mut out_counts: HashMap<&str, u32> = HashMap::new();
    let mut in_counts: HashMap<&str, u32> = HashMap::new();
    for e in &graph.edges {
        *out_counts.entry(e.from.as_str()).or_insert(0) += 1;
        *in_counts.entry(e.to.as_str()).or_insert(0) += 1;
    }

    let mut boundary_out: HashSet<&str> = HashSet::new();
    let mut boundary_in: HashSet<&str> = HashSet::new();
    for be in &graph.boundary_edges {
        match be {
            VisualBoundaryEdge::Out { inside, .. } => {
                boundary_out.insert(inside.as_str());
            }
            VisualBoundaryEdge::In { inside, .. } => {
                boundary_in.insert(inside.as_str());
            }
        }
    }

    let path = graph.source.path.as_str();
    let mut lines: Vec<String> = Vec::with_capacity(nodes.len() + 1);
    let mut sum_desc: u32 = 0;
    let mut sum_anc: u32 = 0;
    let mut desc_unknown = false;
    let mut anc_unknown = false;

    for n in &nodes {
        let id = n.id();
        let desc_num = out_counts.get(id).copied().unwrap_or(0);
        let anc_num = in_counts.get(id).copied().unwrap_or(0);
        let desc_cell = if boundary_out.contains(id) {
            "?".to_string()
        } else {
            desc_num.to_string()
        };
        let anc_cell = if boundary_in.contains(id) {
            "?".to_string()
        } else {
            anc_num.to_string()
        };

        if boundary_out.contains(id) {
            desc_unknown = true;
        } else {
            sum_desc += desc_num;
        }
        if boundary_in.contains(id) {
            anc_unknown = true;
        } else {
            sum_anc += anc_num;
        }

        lines.push(format!(
            "{desc_cell}\t{anc_cell}\t{}",
            format_label(path, n)
        ));
    }

    let total_desc = if desc_unknown {
        "?".to_string()
    } else {
        sum_desc.to_string()
    };
    let total_anc = if anc_unknown {
        "?".to_string()
    } else {
        sum_anc.to_string()
    };
    lines.push(format!("{total_desc}\t{total_anc}\t{} total", nodes.len()));

    let mut out = lines.join("\n");
    out.push('\n');
    out
}
