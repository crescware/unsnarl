//! HTML-style escaping for text spliced into mermaid node / subgraph
//! labels.
//!
//! Mirrors `ts/src/emitter/mermaid/escape.ts`. The TS version uses
//! four chained `String.prototype.replace` calls; the Rust port
//! walks the input once and pushes the replacement bytes into a
//! pre-sized buffer so the per-label allocation count drops to a
//! single owned `String`.

pub fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
#[path = "escape_test.rs"]
mod escape_test;
