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
