//! Renderer-specific strategy for the mermaid emitter.
//!
//! Mirrors `ts/src/emitter/mermaid/strategy/`. The TS port keeps
//! one `MermaidStrategy` interface plus two implementations
//! (`dagreStrategy`, `elkStrategy`); the Rust port collapses both
//! into one enum so the renderer's match exhausts every option.

pub mod strategy_kind;

pub use strategy_kind::{EmptySubgraphContext, EmptySubgraphPatch, MermaidStrategy};
