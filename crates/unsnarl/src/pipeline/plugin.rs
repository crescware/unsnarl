//! Pipeline-level plugin activation.
//!
//! Mirrors `ts/src/pipeline/plugin/` + the `resolvePlugin`
//! resolution layer in `ts/src/cli/run-cli/resolve-plugin.ts`. The
//! TS port discovers bundled plugins via dynamic `import()` at the
//! CLI boundary; the Rust port can't do that across crates, so this
//! module builds a [`PluginRegistry`] up front that knows the
//! bundled plugins by short name and lets the pipeline activate the
//! `--plugin` list before transforming the IR.

use unsnarl_plugin::{PluginActivateError, PluginRegistry, UnsnarlPlugin};

/// Build the default registry. Registers every plugin shipped with
/// this build under its short name (post-`unsnarl-plugin-` strip,
/// matching the form `collect_plugins` produces from
/// `--plugin <name>`).
pub fn default_registry() -> PluginRegistry {
    let mut registry = PluginRegistry::new();
    registry.register("react", unsnarl_plugin_react::plugin());
    registry
}

/// Resolve a list of post-strip plugin names into trait-object
/// references the pipeline can fold over. Mirrors the
/// `Promise.all(normalized.plugins.map(resolvePlugin))` step in
/// `ts/src/cli/run-cli/run-cli.ts`.
pub fn activate<'a>(
    registry: &'a PluginRegistry,
    names: &[String],
) -> Result<Vec<&'a dyn UnsnarlPlugin>, PluginActivateError> {
    registry.activate_all(names)
}

/// Fold a list of activated plugins over `ir`. Mirrors the TS
/// `config.plugins.reduce((acc, p) => p.transform(acc), serialized)`
/// step in `ts/src/pipeline/pipeline.ts`.
pub fn apply_plugins(
    mut ir: unsnarl_ir::serialized::SerializedIR,
    plugins: &[&dyn UnsnarlPlugin],
) -> unsnarl_ir::serialized::SerializedIR {
    for p in plugins {
        ir = p.transform(ir);
    }
    ir
}
