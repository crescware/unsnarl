use std::path::Path;

use unsnarl_root_query::ParsedRootQuery;

use super::radius_suffix::radius_suffix;
use super::root_query_token::root_query_token;

pub fn derive_output_basename(
    roots: &[ParsedRootQuery],
    descendants: Option<u32>,
    ancestors: Option<u32>,
    context: Option<u32>,
    input_path: &str,
) -> String {
    if !roots.is_empty() {
        let root_token = roots
            .iter()
            .map(root_query_token)
            .collect::<Vec<_>>()
            .join("+");
        let suffix = radius_suffix(descendants, ancestors, context);
        return format!("{root_token}{suffix}");
    }

    match Path::new(input_path).file_stem().and_then(|s| s.to_str()) {
        Some(stem) => stem.to_string(),
        None => input_path.to_string(),
    }
}

#[cfg(test)]
#[path = "derive_output_basename_test.rs"]
mod derive_output_basename_test;
