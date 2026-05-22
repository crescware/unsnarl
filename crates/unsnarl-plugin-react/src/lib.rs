//! `unsnarl-plugin-react`: peel React hook wrappers so the IR reads
//! as if `useCallback` / `useMemo` were not in the source.
//!
//! Exposes a [`UnsnarlPluginReact`] struct + [`plugin`] constructor
//! so the `unsnarl` crate can register a `Box<dyn UnsnarlPlugin>` in
//! the pipeline-side registry.

pub mod transform;
pub mod unsnarl_plugin_react;

pub use unsnarl_plugin_react::{plugin, UnsnarlPluginReact};
