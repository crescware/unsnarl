//! Resolve an `else if` chain root's start offset for a nested
//! IfStatement.
//!
//! Compares ancestor nodes by [`Span`] start/end pairs (rather than
//! by reference identity) because the materialised [`AstNode`]
//! values are not pointer-identifiable; span uniqueness within a
//! parsed program suffices for the chain walk.

use oxc_span::Span;

use unsnarl_ir::primitive::AstNode;
use unsnarl_oxc_parity::AstType;

use crate::path_entry::PathEntry;

pub fn if_chain_root_offset(
    parent: Option<&AstNode>,
    key: Option<&str>,
    path: &[PathEntry],
) -> Option<u32> {
    let parent = parent?;
    if !matches!(parent.r#type, AstType::IfStatement) {
        return None;
    }
    if !matches!(key, Some("consequent") | Some("alternate")) {
        return None;
    }
    let mut chain_top_span: Span = parent.span;
    let mut chain_top_offset: u32 = parent.span.start;
    let mut walked = false;
    let len = path.len();
    // Walk from `path.len() - 1` down to `1` inclusive when len >=
    // 2; otherwise the loop body never runs.
    if len >= 2 {
        let mut i = len - 1;
        loop {
            let entry = &path[i];
            if !spans_match(entry.node.span, chain_top_span) {
                break;
            }
            if entry.key != Some("alternate") {
                break;
            }
            let ancestor_entry = &path[i - 1];
            if !matches!(ancestor_entry.node.r#type, AstType::IfStatement) {
                break;
            }
            chain_top_span = ancestor_entry.node.span;
            chain_top_offset = ancestor_entry.node.span.start;
            walked = true;
            if i <= 1 {
                break;
            }
            i -= 1;
        }
    }
    if !walked {
        return None;
    }
    Some(chain_top_offset)
}

fn spans_match(a: Span, b: Span) -> bool {
    a.start == b.start && a.end == b.end
}

#[cfg(test)]
#[path = "if_chain_root_offset_test.rs"]
mod if_chain_root_offset_test;
