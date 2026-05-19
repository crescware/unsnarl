//! Block-context metadata attached to scope serializations.
//!
//! `case_test` is only meaningful when the block is a switch-case
//! clause. `if_chain_root_offset` is set on `else if` chain branches
//! and points at the outermost `IfStatement` so all branches share a
//! merge key.

use serde::Serialize;

use unsnarl_oxc_parity::AstType;

use crate::primitive::SourceOffset;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseClauseBlockContext {
    kind: super::block_context_kind::BlockContextKind,
    parent_type: AstType,
    key: String,
    parent_span_offset: SourceOffset,
    case_test: Option<String>,
}

impl CaseClauseBlockContext {
    pub fn new(
        parent_type: AstType,
        key: String,
        parent_span_offset: SourceOffset,
        case_test: Option<String>,
    ) -> Self {
        assert!(
            !key.is_empty(),
            "CaseClauseBlockContext.key must be non-empty"
        );
        if let Some(t) = case_test.as_deref() {
            assert!(
                !t.is_empty(),
                "CaseClauseBlockContext.case_test, when present, must be non-empty"
            );
        }
        Self {
            kind: super::block_context_kind::BlockContextKind::CaseClause,
            parent_type,
            key,
            parent_span_offset,
            case_test,
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherBlockContext {
    kind: super::block_context_kind::BlockContextKind,
    parent_type: AstType,
    key: String,
    parent_span_offset: SourceOffset,
    if_chain_root_offset: Option<SourceOffset>,
}

impl OtherBlockContext {
    pub fn new(
        parent_type: AstType,
        key: String,
        parent_span_offset: SourceOffset,
        if_chain_root_offset: Option<SourceOffset>,
    ) -> Self {
        assert!(!key.is_empty(), "OtherBlockContext.key must be non-empty");
        Self {
            kind: super::block_context_kind::BlockContextKind::Other,
            parent_type,
            key,
            parent_span_offset,
            if_chain_root_offset,
        }
    }
}

/// The discriminator is repeated inside each variant struct rather
/// than synthesised by serde so callers can construct either variant
/// directly without an extra wrapper layer, and the `Serialize` impl
/// simply delegates to the variant's struct.
#[derive(Clone)]
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
