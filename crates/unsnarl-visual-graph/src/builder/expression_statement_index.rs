//! The visual layer's single source of truth for "where the
//! (non-synthetic) `ExpressionStatement`s are".
//!
//! Built once per fixture from the IR's references: every reference
//! whose `expression_statement_container` is `Some` contributes its
//! enclosing `ExpressionStatement`, keyed by the statement's UTF-16
//! start offset.
//!
//! ## Why synthetic arrow-body statements never appear here
//!
//! The analyzer leaves `expression_statement_container` as `None` for
//! a reference whose nearest `ExpressionStatement` is a synthetic
//! arrow body (`build_analysis_visitor::fire_reference`). Because the
//! index is built only from `Some` containers, those statements are
//! absent *by construction* -- there is no path that inserts one. That
//! is what makes [`ExpressionStatementIndex::enclosing`]'s span
//! containment reach exactly the statement the analyzer's own AST walk
//! would have reached. The equivalence is held by the single `build`
//! constructor plus the private field, not by a comment repeated at
//! each call site.
//!
//! ## Why this correlation lives in the visual layer, not the IR
//!
//! The IR's `callbackArgument` annotation carries only the structural
//! fact (`callee` + `argIndex`). Deciding *whether* a callback is
//! hosted by a statement-level CallProxy wrapper -- and under which
//! `expr_stmt_<offset>` it groups -- is a rendering concern, resolved
//! from these spans. Keeping it here is what lets the IR carry only
//! structure and the visual layer carry layout.

use std::collections::HashMap;

use unsnarl_ir::serialized::{SerializedExpressionStatementContainer, SerializedReference};

/// Span-indexed view of the non-synthetic `ExpressionStatement`s in a
/// fixture. The backing map is private so the only way to obtain one
/// is [`Self::build`] (or [`Self::empty`] for tests), which is what
/// guarantees the synthetic-exclusion invariant documented above.
pub struct ExpressionStatementIndex<'a> {
    by_offset: HashMap<u32, &'a SerializedExpressionStatementContainer>,
}

impl<'a> ExpressionStatementIndex<'a> {
    /// Scan `references` once, indexing every borrowed
    /// `expression_statement_container` by its start offset. The first
    /// container seen for a given offset wins; later references inside
    /// the same statement carry an identical container.
    pub fn build(references: &'a [SerializedReference]) -> Self {
        let mut by_offset: HashMap<u32, &'a SerializedExpressionStatementContainer> =
            HashMap::new();
        for r in references {
            if let Some(c) = r.expression_statement_container.as_ref() {
                by_offset.entry(c.start_span.offset.0).or_insert(c);
            }
        }
        Self { by_offset }
    }

    /// An empty index, for builder unit fixtures whose scopes have no
    /// registered `ExpressionStatement`s.
    pub fn empty() -> Self {
        Self {
            by_offset: HashMap::new(),
        }
    }

    /// The innermost registered `ExpressionStatement` whose span
    /// contains `[start, end]`, or `None` when none encloses it.
    ///
    /// `ExpressionStatement` spans nest without partial overlap, so
    /// among the containers that contain `[start, end]` the one with
    /// the largest start offset is the innermost. Returning the
    /// container itself -- not a bare offset -- means a caller never
    /// holds an offset that might not be registered: the
    /// `expr_stmt_<offset>` id, head, and span lines all come straight
    /// off the result.
    pub fn enclosing(
        &self,
        start: u32,
        end: u32,
    ) -> Option<&'a SerializedExpressionStatementContainer> {
        self.by_offset
            .values()
            .copied()
            .filter(|c| c.start_span.offset.0 <= start && c.end_span.offset.0 >= end)
            .max_by_key(|c| c.start_span.offset.0)
    }
}

#[cfg(test)]
#[path = "expression_statement_index_test.rs"]
mod expression_statement_index_test;
