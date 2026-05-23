//! Build an `Other`-kind `BlockContext` for a scope.
//!
//! Returns `None` when the parent or key is missing; otherwise
//! stamps the parent's type and start offset onto an
//! [`OtherBlockContext`], folding in any `else if` chain root from
//! [`if_chain_root_offset`].
//!
//! Both offsets are recorded in UTF-16 code units. The `oxc_parser`
//! crate returns UTF-8 bytes, so we convert via the pre-built
//! [`SourceIndex`] in O(log lines + line_length) per lookup.

use unsnarl_ir::primitive::{AstNode, SourceIndex, Utf8ByteOffset};
use unsnarl_ir::scope::{BlockContext, OtherBlockContext};

use crate::if_chain_root_offset::if_chain_root_offset;
use crate::path_entry::PathEntry;

pub fn block_context_of(
    parent: Option<&AstNode>,
    key: Option<&str>,
    path: &[PathEntry],
    index: &SourceIndex<'_>,
) -> Option<BlockContext> {
    let parent = parent?;
    let key = key?;
    let chain_root = if_chain_root_offset(Some(parent), Some(key), path)
        .map(|byte_offset| index.span_at(Utf8ByteOffset(byte_offset)).offset);
    let parent_offset = index.span_at(Utf8ByteOffset(parent.span.start)).offset;
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
