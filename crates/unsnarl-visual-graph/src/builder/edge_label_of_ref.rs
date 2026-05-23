//! Builds the edge label off a reference's flag block: emits
//! `"read"` / `"write"` / `"call"` joined by `,` for any flags
//! set, falling back to `"ref"` when none are.

use unsnarl_ir::serialized::SerializedReference;

/// The `read` / `write` / `call` flag triple expands to eight static
/// labels, so the function returns a `&'static str` pulled from a
/// match arm rather than building a `Vec<&str>` + `join(",")` on every
/// call. `emit_reference_edges` invokes this once per emitted edge
/// (~95k times on `mermaid.js`); the static-table form drops two
/// short-lived allocations from every iteration.
pub fn edge_label_of_ref(r: &SerializedReference) -> &'static str {
    match (r.flags.read, r.flags.write, r.flags.call) {
        (false, false, false) => "ref",
        (true, false, false) => "read",
        (false, true, false) => "write",
        (false, false, true) => "call",
        (true, true, false) => "read,write",
        (true, false, true) => "read,call",
        (false, true, true) => "write,call",
        (true, true, true) => "read,write,call",
    }
}

#[cfg(test)]
#[path = "edge_label_of_ref_test.rs"]
mod edge_label_of_ref_test;
