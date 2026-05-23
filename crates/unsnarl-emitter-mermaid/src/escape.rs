//! HTML-style escaping for text spliced into mermaid node / subgraph
//! labels.
//!
//! Walks the input once and pushes the replacement bytes into a
//! pre-sized buffer so the per-label allocation count drops to a
//! single owned `String`.

pub fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    escape_into(&mut out, value);
    out
}

/// Destination-arg variant of [`escape`]: writes the escaped bytes
/// directly into `out` so callers that are already building a larger
/// string (a node line, a subgraph label, ...) avoid the per-label
/// `String` allocation that the owned-return form would force.
pub fn escape_into(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            other => out.push(other),
        }
    }
}

#[cfg(test)]
#[path = "escape_test.rs"]
mod escape_test;
