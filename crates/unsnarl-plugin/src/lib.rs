//! Plugin trait + registry.
//!
//! Mirrors `ts/src/pipeline/plugin/unsnarl-plugin.ts`. The Rust port
//! splits the TS `UnsnarlPlugin` interface into a trait, and adds a
//! small registry that the `unsnarl` crate uses to map CLI plugin
//! names to concrete plugin instances. The trait + registry are
//! pure IR-side machinery; they intentionally depend only on
//! `unsnarl-ir` and never on any emitter / pipeline crate so plugin
//! authors can implement them without pulling in the full pipeline
//! dependency graph.

pub mod plugin_registry;
pub mod unsnarl_plugin;

pub use plugin_registry::{PluginActivateError, PluginRegistry};
pub use unsnarl_plugin::UnsnarlPlugin;
