//! Markdown emitter (wraps mermaid output).
//!
//! The markdown emitter is the only emitter implementation that
//! depends on another emitter implementation
//! (`unsnarl-emitter-mermaid`) — it embeds the mermaid render
//! inside a fenced ```mermaid block.

pub mod code_fence_lang;
pub mod format_depth_query;
pub mod format_highlight_query;
pub mod format_pruning_query;
pub mod markdown;

pub use markdown::MarkdownEmitter;
