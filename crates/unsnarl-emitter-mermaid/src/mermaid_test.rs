//! Pins the strategy-driven output difference of `render_mermaid`
//! itself (not just the per-component pieces). The prose comment in
//! `mermaid.rs` claims the strategy decides which renderer-specific
//! lines (the elk init directive) AND which empty-subgraph patches
//! (the elk "No nodes" placeholder plus its trailer `classDef`) the
//! whole render emits: elk emits both, dagre emits neither. These
//! tests drive the same fixture graph through both strategies and
//! assert exactly that split, with everything else held constant.

use unsnarl_ir::language::Language;
use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_graph::VisualGraph;
use unsnarl_visual_graph::visual_subgraph::{
    ControlSubgraphKind, ControlVisualSubgraph, VisualSubgraph,
};

use super::render_mermaid;
use crate::mermaid_fixtures::base_plain_subgraph;
use crate::strategy::MermaidStrategy;
use crate::theme::DARK_THEME;

const ELK_INIT_DIRECTIVE: &str = r#"%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%"#;

/// A graph whose only top-level element is an empty `if` subgraph.
/// An empty subgraph is the trigger for the strategy's
/// empty-subgraph patch, so this is the minimal fixture that
/// exercises both halves of the strategy split at once.
fn graph_with_one_empty_subgraph() -> VisualGraph {
    let sg: VisualSubgraph = ControlVisualSubgraph {
        id: "s_empty".to_string(),
        elements: Vec::new(),
        ..base_plain_subgraph(ControlSubgraphKind::If)
    }
    .into();
    VisualGraph::new(
        "input.ts",
        Language::Ts,
        Direction::RL,
        vec![sg.into()],
        Vec::new(),
        Vec::new(),
    )
}

fn render(strategy: MermaidStrategy) -> String {
    render_mermaid(
        &graph_with_one_empty_subgraph(),
        strategy,
        &DARK_THEME,
        false,
        None,
        None,
    )
}

#[test]
fn elk_prepends_the_init_directive_dagre_omits_it() {
    let elk = render(MermaidStrategy::Elk);
    let dagre = render(MermaidStrategy::Dagre);
    assert!(elk.starts_with(&format!("{ELK_INIT_DIRECTIVE}\n")));
    assert!(!dagre.contains(ELK_INIT_DIRECTIVE));
    assert!(!dagre.contains("%%{init"));
}

#[test]
fn elk_splices_the_no_nodes_placeholder_into_the_empty_subgraph_dagre_leaves_it_bare() {
    let elk = render(MermaidStrategy::Elk);
    let dagre = render(MermaidStrategy::Dagre);
    assert!(elk.contains(r#"elk_empty_s_empty["No nodes"]"#));
    assert!(!dagre.contains("No nodes"));
    assert!(!dagre.contains("elk_empty_"));
}

#[test]
fn elk_appends_the_placeholder_trailer_classdef_dagre_appends_no_trailer() {
    let elk = render(MermaidStrategy::Elk);
    let dagre = render(MermaidStrategy::Dagre);
    let c = &DARK_THEME.elk_empty_placeholder;
    let expected_classdef = format!(
        "  classDef elkEmptyPlaceholder fill:{},stroke:{};",
        c.fill, c.stroke
    );
    assert!(elk.contains(&expected_classdef));
    assert!(elk.contains("  class elk_empty_s_empty elkEmptyPlaceholder;"));
    assert!(!dagre.contains("elkEmptyPlaceholder"));
}

#[test]
fn the_strategy_independent_skeleton_is_identical_across_strategies() {
    // Everything outside the two strategy patches must be byte-for-byte
    // shared: the `flowchart` header, the subgraph block, and the
    // per-depth `nestL1` class the empty subgraph still receives. This
    // pins that the strategy decides ONLY the init directive and the
    // empty-subgraph patch, nothing else.
    let elk = render(MermaidStrategy::Elk);
    let dagre = render(MermaidStrategy::Dagre);
    for shared in [
        "flowchart RL",
        r#"  subgraph s_empty["if L1"]"#,
        "  class s_empty nestL1;",
    ] {
        assert!(elk.contains(shared), "elk output missing {shared:?}");
        assert!(dagre.contains(shared), "dagre output missing {shared:?}");
    }
}
