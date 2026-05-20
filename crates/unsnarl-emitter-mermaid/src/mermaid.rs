//! `MermaidEmitter`: renders a `SerializedIR` as a mermaid
//! flowchart, respecting the active layout strategy and color
//! theme.
//!
//! Mirrors `MermaidEmitter` in `ts/src/emitter/mermaid/mermaid.ts`.
//! The TS implementation builds a `VisualGraph` from the IR (or
//! pulls a pre-pruned graph from `opts.prunedGraph`) and walks it
//! to emit text. The Rust port follows the same control flow; the
//! pruned-graph short-circuit lands alongside Step 17 when
//! pruning starts attaching `prunedGraph` to `EmitOptions`.

use std::collections::{HashMap, HashSet};

use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_visual_graph::builder::build_visual_graph::build_visual_graph;
use unsnarl_visual_graph::builder::context::BuildVisualGraphOptions;
use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_graph::VisualGraph;
use unsnarl_visual_graph::visual_node::VisualNode;

use crate::collect_import_sources::collect_import_sources;
use crate::collect_nodes_into::collect_nodes_into;
use crate::collect_wrapped_owner_ids::collect_wrapped_owner_ids;
use crate::push_edge_lines::push_edge_lines;
use crate::render_boundary_edges::render_boundary_edges;
use crate::render_class_defs::render_class_defs;
use crate::render_pruning_comment::render_pruning_comment;
use crate::render_state::RenderState;
use crate::render_synthetic_node_block::render_synthetic_node_block;
use crate::render_top_level_nodes::render_top_level_nodes;
use crate::render_top_level_subgraphs::render_top_level_subgraphs;
use crate::split_edges::split_edges;
use crate::strategy::MermaidStrategy;
use crate::theme::ColorTheme;

pub struct MermaidEmitter {
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
}

impl MermaidEmitter {
    pub const FORMAT: &'static str = "mermaid";
    pub const CONTENT_TYPE: &'static str = "text/vnd.mermaid";
    pub const EXTENSION: &'static str = "mmd";

    /// Construct a `MermaidEmitter` bound to the given strategy and
    /// color theme. Both choices live at the CLI / pipeline
    /// boundary; this constructor does not pick defaults so that
    /// every caller is forced to make both decisions explicitly
    /// (matching the TS constructor's required options).
    pub fn new(strategy: MermaidStrategy, theme: &'static ColorTheme) -> Self {
        Self { strategy, theme }
    }
}

impl Emitter for MermaidEmitter {
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
        let graph = build_visual_graph(ir, &BuildVisualGraphOptions::default());
        render_mermaid(&graph, self.strategy, self.theme, opts.debug)
    }
}

fn render_mermaid(
    graph: &VisualGraph,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
) -> String {
    // The strategy decides which renderer-specific lines (e.g. the
    // elk init directive) and which empty-subgraph patches are
    // needed. dagre struggles with nested subgraphs that share
    // edges across boundaries (the function wrapper containing the
    // FunctionName node and the body subgraph, with edges reaching
    // from outside into the body) and produces colliding routes
    // and inconsistent node-vs-body ordering, which is why elk is
    // the default at the CLI / pipeline boundary. dagre is still
    // selectable for environments that cannot register the elk
    // loader (e.g. GitHub's markdown preview).
    let mut lines: Vec<String> = Vec::new();
    for l in strategy.preamble_lines() {
        lines.push((*l).to_string());
    }
    lines.push(format!("flowchart {}", graph.direction.as_str()));
    render_pruning_comment(graph, &mut lines);

    let mut node_map: HashMap<String, &VisualNode> = HashMap::new();
    collect_nodes_into(&graph.elements, &mut node_map);

    // FunctionName nodes that own a function subgraph are absorbed
    // into a wrapper subgraph alongside the body, so they must NOT
    // also be emitted as a sibling node at their declaring scope.
    let mut wrapped_owner_ids: HashSet<String> = HashSet::new();
    collect_wrapped_owner_ids(&graph.elements, &mut wrapped_owner_ids);

    let mut state = RenderState {
        lines,
        node_map,
        wrapped_owner_ids,
        placeholder_ids: Vec::new(),
        nest_class_map: HashMap::new(),
        strategy,
        theme,
        debug,
    };

    // Emit top-level "tree" nodes (anything that isn't a synthetic
    // top-level import/module/sink), then top-level subgraphs,
    // then synthetic top-level nodes -- preserves the historical
    // Mermaid output ordering and keeps the module/intermediate
    // cluster grouped near the import edges.
    render_top_level_nodes(&mut state, graph);
    render_top_level_subgraphs(&mut state, graph);

    // Edges originating from an import-side synthetic node (as
    // selected by `collect_import_sources`) are import edges and
    // rendered after the synthetic node block. Edges that merely
    // point INTO a synthetic node (e.g. `n_x -->|read| module_root`)
    // stay with the body edges to preserve the historical ordering.
    let import_sources = collect_import_sources(&state.node_map);
    let (body_edges, import_edges) = split_edges(&graph.edges, &import_sources);
    push_edge_lines(
        body_edges.iter().copied(),
        &mut state.lines,
        Some(&state.node_map),
    );

    render_synthetic_node_block(&mut state, graph);
    push_edge_lines(
        import_edges.iter().copied(),
        &mut state.lines,
        Some(&state.node_map),
    );

    let mut stub_ids: Vec<String> = Vec::new();
    render_boundary_edges(graph, &mut state.lines, &mut stub_ids);
    // Depth-limit stubs share the boundary-stub class so they pick
    // up the same dashed-circle treatment as the pruning ones.
    // Walks the element tree directly (rather than `state.node_map.values()`)
    // so the emitted order matches the deterministic insertion order
    // TS gets out of `Map.prototype.values()`.
    collect_beyond_depth_stub_ids(&graph.elements, &mut stub_ids);

    let var_ids = collect_var_node_ids(&graph.elements);
    render_class_defs(
        &stub_ids,
        &var_ids,
        &state.nest_class_map,
        state.theme,
        &mut state.lines,
    );

    for l in strategy.trailer_lines(&state.placeholder_ids, state.theme) {
        state.lines.push(l);
    }

    let mut out = state.lines.join("\n");
    out.push('\n');
    out
}

fn collect_var_node_ids(elements: &[VisualElement]) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    walk_collect_kind(elements, NodeKind::VarBinding, &mut ids);
    ids
}

fn collect_beyond_depth_stub_ids(elements: &[VisualElement], out: &mut Vec<String>) {
    walk_collect_kind(elements, NodeKind::SyntheticBeyondDepth, out);
}

/// Deep-walks the element tree in preorder and appends every
/// matching node's id to `out`. The walk order mirrors the TS
/// `collectNodesInto` insertion order so the rendered class blocks
/// stay deterministic byte-for-byte against the TS baselines.
fn walk_collect_kind(elements: &[VisualElement], kind: NodeKind, out: &mut Vec<String>) {
    for e in elements {
        match e {
            VisualElement::Node(n) => {
                if n.kind() == kind {
                    out.push(n.id().to_string());
                }
            }
            VisualElement::Subgraph(s) => {
                walk_collect_kind(s.elements(), kind, out);
            }
        }
    }
}
