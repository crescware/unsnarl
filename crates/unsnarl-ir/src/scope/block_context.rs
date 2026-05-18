//! Block-context metadata attached to scope serializations.
//!
//! `case_test` is only meaningful when the block is a switch-case
//! clause. `if_chain_root_offset` is set on `else if` chain branches
//! and points at the outermost `IfStatement` so all branches share a
//! merge key.

use serde::Serialize;

use crate::filled_string::FilledString;
use unsnarl_ast_type::AstType;

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

/// The discriminator is repeated inside each variant struct rather
/// than synthesised by serde so callers can construct either variant
/// directly without an extra wrapper layer, and the `Serialize` impl
/// simply delegates to the variant's struct.
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
