//! Record of a `LineOrName` query that was disambiguated.
//!
//! Emitted by [`resolve_ambiguous_queries`](super::resolve_ambiguous_queries)
//! and consumed by the markdown emitter's Notice section and the
//! CLI's stderr emitter.

use unsnarl_ir::SourceLine;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedAs {
    Name,
    Line,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootQueryResolution {
    pub raw: String,
    pub line: SourceLine,
    pub name: String,
    pub resolved_as: ResolvedAs,
}
