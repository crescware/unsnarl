//! `RenderState`: mutable scratch space threaded through the
//! per-element render helpers.
//!
//! The collections are owned by the struct and the struct is
//! threaded through the render helpers via `&mut RenderState`.

use std::collections::{HashMap, HashSet};

use unsnarl_visual_graph::visual_node::VisualNode;

use crate::strategy::MermaidStrategy;
use crate::theme::ColorTheme;

pub struct RenderState<'a> {
    pub lines: Vec<String>,
    pub node_map: HashMap<String, &'a VisualNode>,
    pub wrapped_owner_ids: HashSet<String>,
    pub placeholder_ids: Vec<String>,
    /// Subgraph ids grouped by 0-based palette slot. Filled as
    /// subgraphs (including function wrappers) are emitted with
    /// their depth; consumed by `render_class_defs` to emit the
    /// per-level `classDef nestL<n>` / `class ... nestL<n>` rows.
    pub nest_class_map: HashMap<usize, Vec<String>>,
    pub strategy: MermaidStrategy,
    pub theme: &'static ColorTheme,
    pub debug: bool,
}
