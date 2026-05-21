//! `MermaidStrategy`: enum selecting the renderer-specific lines.
//!
//! Mirrors `ts/src/emitter/mermaid/strategy/strategy.ts` together
//! with `dagre-strategy.ts` / `elk-strategy.ts`. The TS port keeps
//! the strategy as an object with three callable members so it can
//! be passed as a dependency; the Rust port collapses that into a
//! two-variant enum because the choice has zero open extension
//! points -- new strategies would land here as a new variant rather
//! than via an external `impl`.

use crate::theme::ColorTheme;

/// Context the renderer hands to
/// [`MermaidStrategy::empty_subgraph_placeholder`] when a subgraph
/// is about to close with no body content.
pub struct EmptySubgraphContext<'a> {
    /// Subgraph id about to close with no body content.
    pub subgraph_id: &'a str,
    /// Indent prefix to use for any line emitted inside the
    /// subgraph.
    pub indent: &'a str,
}

/// Patch returned by [`MermaidStrategy::empty_subgraph_placeholder`]
/// when the strategy wants to splice a placeholder line into the
/// empty subgraph.
pub struct EmptySubgraphPatch {
    pub line: String,
    pub placeholder_id: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MermaidStrategy {
    Dagre,
    Elk,
}

impl MermaidStrategy {
    /// Lines emitted before `flowchart` (e.g. an
    /// `%%{init: ...}%%` directive on elk).
    pub fn preamble_lines(self) -> &'static [&'static str] {
        match self {
            Self::Dagre => &[],
            Self::Elk => &[r#"%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%"#],
        }
    }

    /// Called when a subgraph is about to close with zero emitted
    /// children. Returns `Some(...)` to insert a single line inside
    /// the subgraph and register a placeholder id; returns `None`
    /// to leave it empty.
    pub fn empty_subgraph_placeholder(
        self,
        ctx: EmptySubgraphContext<'_>,
    ) -> Option<EmptySubgraphPatch> {
        match self {
            Self::Dagre => None,
            Self::Elk => {
                let placeholder_id = format!("elk_empty_{}", ctx.subgraph_id);
                let line = format!(r#"{}{}["No nodes"]"#, ctx.indent, placeholder_id);
                Some(EmptySubgraphPatch {
                    line,
                    placeholder_id,
                })
            }
        }
    }

    /// Lines appended at the end of the diagram, after every node,
    /// edge and other classDef. Receives every placeholder id
    /// produced during the run plus the active color theme so the
    /// strategy can attach a `classDef` / `class` styling block
    /// whose colors match the rest of the diagram.
    pub fn trailer_lines(self, placeholder_ids: &[String], theme: &ColorTheme) -> Vec<String> {
        match self {
            Self::Dagre => Vec::new(),
            Self::Elk => {
                if placeholder_ids.is_empty() {
                    return Vec::new();
                }
                let c = &theme.elk_empty_placeholder;
                let mut out: Vec<String> = Vec::with_capacity(placeholder_ids.len() + 1);
                out.push(format!(
                    "  classDef elkEmptyPlaceholder fill:{},stroke:{};",
                    c.fill, c.stroke
                ));
                for id in placeholder_ids {
                    out.push(format!("  class {id} elkEmptyPlaceholder;"));
                }
                out
            }
        }
    }
}

#[cfg(test)]
#[path = "strategy_kind_test.rs"]
mod strategy_kind_test;
