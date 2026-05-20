//! Public [`UnsnarlPlugin`] entrypoint for the React plugin.
//!
//! Mirrors the default export of
//! `ts/src/plugins/unsnarl-plugin-react/index.ts`. The TS literal:
//!
//! ```ts
//! const plugin: UnsnarlPlugin = {
//!   meta: { name: "unsnarl-plugin-react" },
//!   transform(ir) { ... },
//! };
//! export default plugin;
//! ```
//!
//! becomes a unit struct implementing [`UnsnarlPlugin`] plus a free
//! [`plugin`] function returning a `Box<dyn UnsnarlPlugin>` ready to
//! be registered with [`unsnarl_plugin::PluginRegistry`].

use unsnarl_ir::serialized::SerializedIR;
use unsnarl_plugin::UnsnarlPlugin;

use crate::transform::transform_ir;

/// Canonical plugin name. Matches the TS `meta.name`.
pub const PLUGIN_NAME: &str = "unsnarl-plugin-react";

pub struct UnsnarlPluginReact;

impl UnsnarlPlugin for UnsnarlPluginReact {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn transform(&self, ir: SerializedIR) -> SerializedIR {
        transform_ir(ir)
    }
}

/// Boxed instance suitable for `PluginRegistry::register`.
pub fn plugin() -> Box<dyn UnsnarlPlugin> {
    Box::new(UnsnarlPluginReact)
}
