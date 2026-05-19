use std::path::Path;

use unsnarl_root_query::{GenerationCount, ParsedRootQuery};

use super::radius_suffix::radius_suffix;
use super::root_query_token::root_query_token;

pub fn derive_output_basename(
    roots: &[ParsedRootQuery],
    descendants: Option<GenerationCount>,
    ancestors: Option<GenerationCount>,
    context: Option<GenerationCount>,
    input_path: &Path,
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

    match input_path.file_stem().and_then(|s| s.to_str()) {
        Some(stem) => stem.to_string(),
        None => input_path.to_string_lossy().into_owned(),
    }
}

#[cfg(test)]
#[path = "derive_output_basename_test.rs"]
mod derive_output_basename_test;
