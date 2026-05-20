//! Append-or-create helper for `Map<String, Vec<String>>`.
//!
//! Mirrors `ts/src/visual-graph/prune/push-to.ts`.

use std::collections::HashMap;

pub fn push_to(map: &mut HashMap<String, Vec<String>>, key: &str, value: String) {
    match map.get_mut(key) {
        Some(arr) => arr.push(value),
        None => {
            map.insert(key.to_string(), vec![value]);
        }
    }
}

#[cfg(test)]
#[path = "push_to_test.rs"]
mod push_to_test;
