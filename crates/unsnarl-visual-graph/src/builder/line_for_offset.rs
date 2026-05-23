//! Returns the 1-based line number for a UTF-16 code-unit `offset`
//! into the source string carried by `source_index`. Backed by
//! [`unsnarl_ir::primitive::SourceIndex`]'s precomputed line-start
//! table, so the lookup is `O(log lines)` per call rather than
//! the linear-scan shape it had before.

use unsnarl_ir::primitive::SourceIndex;

pub fn line_for_offset(source_index: &SourceIndex<'_>, offset: u32) -> u32 {
    let _t = super::timing::TimingScope::start("line_for_offset");
    source_index.span_at(offset as usize).line.0
}

#[cfg(test)]
#[path = "line_for_offset_test.rs"]
mod line_for_offset_test;
