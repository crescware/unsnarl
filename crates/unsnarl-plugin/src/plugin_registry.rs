//! Small registry that maps a short plugin name (the form the CLI
//! collects after stripping the `unsnarl-plugin-` prefix) to a
//! concrete [`UnsnarlPlugin`] instance.
//!
//! Because plugin types cannot be dynamic-imported across crates,
//! the consumer (the `unsnarl` crate's pipeline-side plugin module)
//! builds a registry up front by registering each bundled plugin
//! under its short name. The registry then resolves the post-strip
//! CLI name list into a `Vec<&dyn UnsnarlPlugin>` for the pipeline
//! to fold over.

use std::fmt;

use crate::unsnarl_plugin::UnsnarlPlugin;

/// Error returned when [`PluginRegistry::activate_all`] is asked to
/// activate a plugin that has not been registered. Renders as:
/// `Plugin 'unsnarl-plugin-<name>' is not bundled with this unsnarl build.`
#[derive(Debug)]
pub struct PluginActivateError {
    name: String,
}

impl PluginActivateError {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for PluginActivateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Plugin 'unsnarl-plugin-{}' is not bundled with this unsnarl build.",
            self.name
        )
    }
}

impl std::error::Error for PluginActivateError {}

struct RegistryEntry {
    short_name: String,
    plugin: Box<dyn UnsnarlPlugin>,
}

#[derive(Default)]
pub struct PluginRegistry {
    entries: Vec<RegistryEntry>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register `plugin` under the given short name (e.g. `"react"`).
    /// The short name is the form the CLI's
    /// [`collect_plugins`](https://docs.rs/) helper produces after
    /// stripping the `unsnarl-plugin-` prefix; the CLI's name list
    /// alias-resolves to it before the pipeline asks the registry to
    /// activate.
    pub fn register(&mut self, short_name: impl Into<String>, plugin: Box<dyn UnsnarlPlugin>) {
        self.entries.push(RegistryEntry {
            short_name: short_name.into(),
            plugin,
        });
    }

    /// Resolve a list of short names into the matching plugin
    /// instances, preserving the input order. Fails on the first
    /// unknown name.
    pub fn activate_all(
        &self,
        names: &[String],
    ) -> Result<Vec<&dyn UnsnarlPlugin>, PluginActivateError> {
        let mut out = Vec::with_capacity(names.len());
        for name in names {
            let entry = self
                .entries
                .iter()
                .find(|e| e.short_name == *name)
                .ok_or_else(|| PluginActivateError::new(name.clone()))?;
            out.push(entry.plugin.as_ref());
        }
        Ok(out)
    }
}

#[cfg(test)]
#[path = "plugin_registry_test.rs"]
mod plugin_registry_test;
