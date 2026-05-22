//! Renderer-specific strategy for the mermaid emitter.
//!
//! The strategy is modeled as an enum (with `dagre` and `elk` as
//! its variants) so the renderer's match exhausts every option.

pub mod strategy_kind;

pub use strategy_kind::{EmptySubgraphContext, EmptySubgraphPatch, MermaidStrategy};
