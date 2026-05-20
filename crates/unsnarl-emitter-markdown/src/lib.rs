//! Markdown emitter (wraps mermaid output).
//!
//! Mirrors `ts/src/emitter/markdown/`. The markdown emitter is the
//! only emitter implementation that depends on another emitter
//! implementation (`unsnarl-emitter-mermaid`) — it embeds the
//! mermaid render inside a fenced ```mermaid block. The
//! depth-/highlight-/pruning-query helpers from the TS directory
//! return alongside Steps 17–19 when `EmitOptions` grows the
//! corresponding option fields.

pub mod code_fence_lang;
pub mod format_depth_query;
pub mod format_pruning_query;
pub mod markdown;

pub use markdown::MarkdownEmitter;
