//! `unsnarl-plugin-react`: peel React hook wrappers so the IR reads
//! as if `useCallback` / `useMemo` were not in the source.
//!
//! Mirrors `ts/src/plugins/unsnarl-plugin-react/index.ts`. The TS
//! module exports a single `UnsnarlPlugin` literal; here we expose a
//! [`UnsnarlPluginReact`] struct + [`plugin`] constructor so the
//! `unsnarl` crate can register a `Box<dyn UnsnarlPlugin>` in the
//! pipeline-side registry.

pub mod transform;
pub mod unsnarl_plugin_react;

pub use unsnarl_plugin_react::{plugin, UnsnarlPluginReact};
