//! Pipeline-level plugin activation.
//!
//! Because plugins cannot be discovered through dynamic `import()`
//! across crates, this module builds a [`PluginRegistry`] up front
//! that knows the bundled plugins by short name and lets the
//! pipeline activate the `--plugin` list before transforming the IR.

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
/// references the pipeline can fold over.
pub fn activate<'a>(
    registry: &'a PluginRegistry,
    names: &[String],
) -> Result<Vec<&'a dyn UnsnarlPlugin>, PluginActivateError> {
    registry.activate_all(names)
}

/// Fold a list of activated plugins over `ir`, threading the IR
/// through each `transform` in turn.
pub fn apply_plugins(
    mut ir: unsnarl_ir::serialized::SerializedIR,
    plugins: &[&dyn UnsnarlPlugin],
) -> unsnarl_ir::serialized::SerializedIR {
    if plugins.is_empty() {
        return ir;
    }
    let _span = tracing::info_span!("apply_plugins", count = plugins.len()).entered();
    for p in plugins {
        ir = p.transform(ir);
    }
    ir
}

#[cfg(test)]
#[path = "plugin_test.rs"]
mod plugin_test;
