use unsnarl_root_query::GenerationCount;

/// Stable a-b-c ordering keeps the filename deterministic regardless of
/// CLI argument order. Flags the user did not type are omitted, so an
/// implicit -C 10 default produces no suffix. -C is shorthand for setting
/// both -A and -B; if both are typed explicitly, -C has no remaining
/// effect on the run, so we drop it from the filename too.
pub fn radius_suffix(
    descendants: Option<GenerationCount>,
    ancestors: Option<GenerationCount>,
    context: Option<GenerationCount>,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(n) = descendants {
        parts.push(format!("a{}", n.0));
    }
    if let Some(n) = ancestors {
        parts.push(format!("b{}", n.0));
    }
    let both_explicit = descendants.is_some() && ancestors.is_some();
    if let Some(n) = context {
        if !both_explicit {
            parts.push(format!("c{}", n.0));
        }
    }
    if parts.is_empty() {
        String::new()
    } else {
        format!("-{}", parts.join("-"))
    }
}

#[cfg(test)]
#[path = "radius_suffix_test.rs"]
mod radius_suffix_test;
