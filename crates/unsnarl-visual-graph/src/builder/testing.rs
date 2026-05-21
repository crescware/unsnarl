//! Shared test fixtures for sibling tests under
//! `crates/unsnarl-visual-graph/src/builder/`. Mirrors the
//! `ts/src/visual-graph/builder/testing/` helpers (`make-scope.ts`
//! et al.) so the Rust tests can express each test case as a
//! field-by-field override of a known base, rather than rebuilding
//! the full struct in every test.
//!
//! Gated behind `#[cfg(test)]`; not part of the crate's public API.

use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::primitive::{SourceColumn, SourceLine, SourceOffset, Span};
use unsnarl_ir::reference::predicate_container::PredicateContainer;
use unsnarl_ir::scope::block_context::{BlockContext, OtherBlockContext};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::reference_id::SerializedReferenceId;
use unsnarl_ir::serialized::scope_id::SerializedScopeId;
use unsnarl_ir::serialized::serialized_reference::{
    SerializedCompletion, SerializedFlags, SerializedReference, SerializedReferenceIdentifier,
};
use unsnarl_ir::serialized::serialized_scope::{SerializedBlock, SerializedScope};
use unsnarl_oxc_parity::{AstType, PredicateContainerType};

use super::write_op::WriteOp;

/// Build a `Span` at the given 0-based offset on line 1.
pub(crate) fn span(offset: u32) -> Span {
    span_at(1, offset, offset)
}

/// Build a `Span` at an explicit `(line, column, offset)`.
pub(crate) fn span_at(line: u32, column: u32, offset: u32) -> Span {
    Span {
        line: SourceLine(line),
        column: SourceColumn(column),
        offset: SourceOffset(offset),
    }
}

/// Wrap a string as a `SerializedScopeId`.
///
/// The wrapper asserts non-empty, so callers that need a literal
/// `""` scope id (rare, but the TS suite exercises it for a few
/// formatter helpers) build a `SerializedScope` directly instead.
pub(crate) fn scope_id(value: &str) -> SerializedScopeId {
    SerializedScopeId::new(value.to_string())
}

/// Mirrors `baseScope()` from
/// `ts/src/visual-graph/builder/testing/make-scope.ts`. Returns a
/// `Block` scope with empty variables / references / children, a
/// 1-character body span, and zeroed nesting depths. Tests override
/// individual fields by binding the result and mutating, e.g.
///
/// ```ignore
/// let mut s = base_serialized_scope("scope1");
/// s.r#type = ScopeType::Function;
/// ```
pub(crate) fn base_serialized_scope(id: &str) -> SerializedScope {
    SerializedScope {
        id: scope_id(id),
        r#type: ScopeType::Block,
        is_strict: false,
        upper: None,
        child_scopes: Vec::new(),
        variable_scope: scope_id(id),
        block: SerializedBlock {
            r#type: AstType::BlockStatement,
            span: span(0),
            end_span: span_at(10, 1, 10),
        },
        variables: Vec::new(),
        references: Vec::new(),
        through: Vec::new(),
        function_expression_scope: false,
        block_context: None,
        falls_through: false,
        exits_function: false,
        nesting_depths: NestingDepths::uniform(NestingDepth(0)),
    }
}

/// Build an [`OtherBlockContext`] (the non-case-clause variant) and
/// wrap it as [`BlockContext::Other`].
pub(crate) fn other_block_context(
    parent_type: AstType,
    key: &str,
    parent_span_offset: u32,
    if_chain_root_offset: Option<u32>,
) -> BlockContext {
    BlockContext::Other(OtherBlockContext::new(
        parent_type,
        key.to_string(),
        SourceOffset(parent_span_offset),
        if_chain_root_offset.map(SourceOffset),
    ))
}

/// Mirrors `baseWriteOp()` from `testing/make-write-op.ts`. All
/// fields are pre-set to the TS defaults; callers mutate the
/// fields they care about for each fixture.
pub(crate) fn base_write_op() -> WriteOp {
    WriteOp {
        ref_id: "r".to_string(),
        var_id: "v".to_string(),
        var_name: "x".to_string(),
        line: 1,
        offset: 0,
        scope_id: "s".to_string(),
    }
}

/// Mirrors `baseRef()` from `testing/make-ref.ts`. Returns a
/// reference rooted in scope `s`, identifier `x` at offset 0, with
/// no completion / predicate / jsx / expression-statement payload.
pub(crate) fn base_serialized_reference() -> SerializedReference {
    SerializedReference {
        id: SerializedReferenceId::new("r".to_string()),
        identifier: SerializedReferenceIdentifier::new("x".to_string(), span(0)),
        from: scope_id("s"),
        resolved: None,
        owners: Vec::new(),
        init: false,
        flags: SerializedFlags {
            read: false,
            write: false,
            call: false,
            receiver: false,
        },
        predicate_container: None,
        completion: SerializedCompletion::Normal,
        jsx_element: None,
        expression_statement_container: None,
    }
}

/// Mirrors `predicateContainer(type, offset)` from
/// `testing/predicate-container.ts`.
pub(crate) fn predicate_container(
    r#type: PredicateContainerType,
    offset: u32,
) -> PredicateContainer {
    PredicateContainer {
        r#type,
        offset: SourceOffset(offset),
    }
}
