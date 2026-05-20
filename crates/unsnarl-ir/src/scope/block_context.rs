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

    pub fn kind(&self) -> &super::block_context_kind::BlockContextKind {
        &self.kind
    }

    pub fn parent_type(&self) -> &AstType {
        &self.parent_type
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn parent_span_offset(&self) -> SourceOffset {
        self.parent_span_offset
    }

    pub fn case_test(&self) -> Option<&str> {
        self.case_test.as_deref()
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

    pub fn kind(&self) -> &super::block_context_kind::BlockContextKind {
        &self.kind
    }

    pub fn parent_type(&self) -> &AstType {
        &self.parent_type
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn parent_span_offset(&self) -> SourceOffset {
        self.parent_span_offset
    }

    pub fn if_chain_root_offset(&self) -> Option<SourceOffset> {
        self.if_chain_root_offset
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

impl BlockContext {
    pub fn parent_type(&self) -> &AstType {
        match self {
            Self::CaseClause(b) => b.parent_type(),
            Self::Other(b) => b.parent_type(),
        }
    }

    pub fn key(&self) -> &str {
        match self {
            Self::CaseClause(b) => b.key(),
            Self::Other(b) => b.key(),
        }
    }

    pub fn parent_span_offset(&self) -> SourceOffset {
        match self {
            Self::CaseClause(b) => b.parent_span_offset(),
            Self::Other(b) => b.parent_span_offset(),
        }
    }

    pub fn kind(&self) -> &super::block_context_kind::BlockContextKind {
        match self {
            Self::CaseClause(b) => b.kind(),
            Self::Other(b) => b.kind(),
        }
    }

    /// Convenience accessor that returns the `caseTest` payload when
    /// `self` is a [`CaseClauseBlockContext`], `None` otherwise.
    pub fn case_test(&self) -> Option<&str> {
        match self {
            Self::CaseClause(b) => b.case_test(),
            Self::Other(_) => None,
        }
    }

    /// Convenience accessor that returns the `ifChainRootOffset`
    /// payload when `self` is an [`OtherBlockContext`], `None`
    /// otherwise.
    pub fn if_chain_root_offset(&self) -> Option<SourceOffset> {
        match self {
            Self::CaseClause(_) => None,
            Self::Other(b) => b.if_chain_root_offset(),
        }
    }
}

impl Serialize for BlockContext {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CaseClause(b) => b.serialize(serializer),
            Self::Other(b) => b.serialize(serializer),
        }
    }
}
