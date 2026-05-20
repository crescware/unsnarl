//! Walker types shared across `scope_build_visitor` and `classify/*`.
//!
//! Mirrors `walk/path-entry.ts` (plus `walk/walk-action.ts` /
//! `walk/walk-node.ts`'s implicit `path` shape). TS records
//! `(node, key)` for each ancestor in the path; the Rust port keeps
//! the same shape but uses `AstKind<'a>` for `node` to preserve the
//! full parent context (per #118 comment 4 judgment A: the internal
//! classify layer needs structural access like `parent.computed`
//! that a materialised `AstNode` cannot provide).

use oxc_ast::AstKind;

pub(crate) struct PathEntry<'a> {
    pub node: AstKind<'a>,
    pub key: Option<&'static str>,
}
