//! Mirrors `ts/src/cli/run-cli/resolve-output-path/radius-suffix.ts`.

/// Stable a-b-c ordering keeps the filename deterministic regardless of
/// CLI argument order. Flags the user did not type are omitted, so an
/// implicit -C 10 default produces no suffix. -C is shorthand for setting
/// both -A and -B; if both are typed explicitly, -C has no remaining
/// effect on the run, so we drop it from the filename too.
pub fn radius_suffix(
    descendants: Option<u32>,
    ancestors: Option<u32>,
    context: Option<u32>,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(n) = descendants {
        parts.push(format!("a{n}"));
    }
    if let Some(n) = ancestors {
        parts.push(format!("b{n}"));
    }
    let both_explicit = descendants.is_some() && ancestors.is_some();
    if let Some(n) = context {
        if !both_explicit {
            parts.push(format!("c{n}"));
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
