//! Mirrors `ts/src/visual-graph/builder/intermediate-key.ts`.

pub fn intermediate_key(source: &str, original_name: &str) -> String {
    format!("{source}::{original_name}")
}

#[cfg(test)]
#[path = "intermediate_key_test.rs"]
mod intermediate_key_test;
