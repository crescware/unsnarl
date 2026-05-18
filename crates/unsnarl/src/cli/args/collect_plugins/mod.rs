//! Plugin-name collection and bundled-name validation for `--plugin`.
//!
//! `collect_plugins` is a 1:1 port of `ts/src/cli/args/collect-plugins.ts`:
//! split a comma-delimited value, trim each fragment, drop empties, strip
//! the `unsnarl-plugin-` prefix, and deduplicate against the accumulator.
//! `parse_plugin_occurrence` is the clap `value_parser` wrapper that calls
//! `collect_plugins` for one CLI occurrence and rejects any name not in
//! `BUNDLED_PLUGINS`. Rejection happens at clap parse time so unknown
//! plugins surface as exit code 2 (`#108` Step 4 acceptance criterion).
//!
//! The TS truth for rejection is `ts/src/cli/run-cli/resolve-plugin.ts`,
//! which lets the dynamic `import()` fail. Rust plugins compile in as
//! sibling crates and have no dynamic resolve, so the rejection layer is
//! raised to clap. `BUNDLED_PLUGINS` is therefore declared here as the
//! compile-time enumeration.

const BUNDLED_PLUGINS: &[&str] = &["react"];
const PLUGIN_PREFIX: &str = "unsnarl-plugin-";

pub fn collect_plugins(value: &str, prev: &[String]) -> Vec<String> {
    let mut result: Vec<String> = prev.to_vec();
    for fragment in value.split(',') {
        let trimmed = fragment.trim();
        if trimmed.is_empty() {
            continue;
        }
        let stripped = trimmed
            .strip_prefix(PLUGIN_PREFIX)
            .unwrap_or(trimmed)
            .to_string();
        if !result.contains(&stripped) {
            result.push(stripped);
        }
    }
    result
}

pub fn parse_plugin_occurrence(value: &str) -> Result<Vec<String>, String> {
    let names = collect_plugins(value, &[]);
    for name in &names {
        if !BUNDLED_PLUGINS.contains(&name.as_str()) {
            return Err(format!("unknown plugin '{name}'"));
        }
    }
    Ok(names)
}

#[cfg(test)]
mod test;
