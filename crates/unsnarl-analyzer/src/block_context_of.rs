//! Build an `Other`-kind `BlockContext` for a scope.
//!
//! Mirrors `ts/src/analyzer/block-context-of.ts`. Returns `None` when
//! the parent or key is missing; otherwise stamps the parent's type
//! and start offset onto an [`OtherBlockContext`], folding in any
//! `else if` chain root from [`if_chain_root_offset`].
//!
//! Both offsets are recorded in UTF-16 code units to match the TS
//! reference (npm `oxc-parser` returns positions in UTF-16; the Rust
//! `oxc_parser` crate returns UTF-8 bytes, so we convert).

use unsnarl_ir::primitive::{span_from_offset, AstNode};
use unsnarl_ir::scope::{BlockContext, OtherBlockContext};

use crate::if_chain_root_offset::if_chain_root_offset;
use crate::path_entry::PathEntry;

pub fn block_context_of(
    parent: Option<&AstNode>,
    key: Option<&str>,
    path: &[PathEntry],
    raw: &str,
) -> Option<BlockContext> {
    let parent = parent?;
    let key = key?;
    let chain_root = if_chain_root_offset(Some(parent), Some(key), path)
        .map(|byte_offset| span_from_offset(raw, byte_offset as usize).offset);
    let parent_offset = span_from_offset(raw, parent.span.start as usize).offset;
    Some(BlockContext::Other(OtherBlockContext::new(
        parent.r#type.clone(),
        key.to_string(),
        parent_offset,
        chain_root,
    )))
}

#[cfg(test)]
#[path = "block_context_of_test.rs"]
mod block_context_of_test;
