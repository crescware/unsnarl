//! Walker types shared across `scope_build_visitor` and `classify/*`.
//!
//! Records `(node, key)` for each ancestor in the walker path.
//! `node` is `AstKind<'a>` rather than the materialised `AstNode`
//! so the internal classify layer can read structural fields like
//! `parent.computed` that the type-and-span-only `AstNode` cannot
//! provide.

use oxc_ast::AstKind;

pub(crate) struct PathEntry<'a> {
    pub node: AstKind<'a>,
    pub key: Option<&'static str>,
}
