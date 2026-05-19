//! Build an `Other`-kind `BlockContext` for a scope.
//!
//! Mirrors `ts/src/analyzer/block-context-of.ts`. Returns `None` when
//! the parent or key is missing; otherwise stamps the parent's type
//! and start offset onto an [`OtherBlockContext`], folding in any
//! `else if` chain root from [`if_chain_root_offset`].

use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope::{BlockContext, OtherBlockContext};
use unsnarl_ir::SourceOffset;

use crate::if_chain_root_offset::if_chain_root_offset;
use crate::path_entry::PathEntry;

pub fn block_context_of(
    parent: Option<&AstNode>,
    key: Option<&str>,
    path: &[PathEntry],
) -> Option<BlockContext> {
    let parent = parent?;
    let key = key?;
    let chain_root = if_chain_root_offset(Some(parent), Some(key), path).map(SourceOffset);
    Some(BlockContext::Other(OtherBlockContext::new(
        parent.r#type.clone(),
        key.to_string(),
        SourceOffset(parent.span.start),
        chain_root,
    )))
}

#[cfg(test)]
#[path = "block_context_of_test.rs"]
mod block_context_of_test;
