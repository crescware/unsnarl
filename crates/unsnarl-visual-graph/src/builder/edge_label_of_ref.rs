//! Builds the edge label off a reference's flag block: emits
//! `"read"` / `"write"` / `"call"` joined by `,` for any flags
//! set, falling back to `"ref"` when none are.

use unsnarl_ir::serialized::SerializedReference;

pub fn edge_label_of_ref(r: &SerializedReference) -> String {
    let mut parts: Vec<&'static str> = Vec::new();
    if r.flags.read {
        parts.push("read");
    }
    if r.flags.write {
        parts.push("write");
    }
    if r.flags.call {
        parts.push("call");
    }
    if parts.is_empty() {
        "ref".to_string()
    } else {
        parts.join(",")
    }
}

#[cfg(test)]
#[path = "edge_label_of_ref_test.rs"]
mod edge_label_of_ref_test;
