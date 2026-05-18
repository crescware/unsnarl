//! Block-context metadata attached to scope serializations. Ports
//! `ts/src/ir/scope/block-context.ts`.
//!
//! `caseTest` is only meaningful when the block is a switch-case
//! clause. `ifChainRootOffset` is set on `else if` chain branches and
//! points at the outermost `IfStatement` so all branches share a merge
//! key.

use serde::Serialize;

use crate::ast_type::AstType;
use crate::filled_string::FilledString;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseClauseBlockContext {
    pub kind: super::block_context_kind::BlockContextKind,
    pub parent_type: AstType,
    pub key: FilledString,
    pub parent_span_offset: u32,
    pub case_test: Option<FilledString>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherBlockContext {
    pub kind: super::block_context_kind::BlockContextKind,
    pub parent_type: AstType,
    pub key: FilledString,
    pub parent_span_offset: u32,
    pub if_chain_root_offset: Option<u32>,
}

/// `variant("kind", ...)` in TS. The discriminator is repeated inside
/// each variant (matching the TS shape) rather than synthesised by
/// serde so callers can construct either variant directly without an
/// extra wrapper layer.
pub enum BlockContext {
    CaseClause(CaseClauseBlockContext),
    Other(OtherBlockContext),
}

impl Serialize for BlockContext {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CaseClause(b) => b.serialize(serializer),
            Self::Other(b) => b.serialize(serializer),
        }
    }
}
