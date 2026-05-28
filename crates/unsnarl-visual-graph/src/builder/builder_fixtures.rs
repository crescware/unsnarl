//! Shared test fixtures for sibling tests under
//! `crates/unsnarl-visual-graph/src/builder/`. Lets each test case
//! express itself as a field-by-field override of a known base,
//! rather than rebuilding the full struct in every test.
//!
//! Gated behind `#[cfg(test)]`; not part of the crate's public API.

use std::collections::HashMap;

use unsnarl_ir::language::Language;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::primitive::{SourceColumn, SourceLine, Span, Utf16CodeUnitOffset};
use unsnarl_ir::reference::predicate_container::PredicateContainer;
use unsnarl_ir::scope::block_context::{BlockContext, CaseClauseBlockContext, OtherBlockContext};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::reference_id::SerializedReferenceId;
use unsnarl_ir::serialized::scope_id::SerializedScopeId;
use unsnarl_ir::serialized::serialized_definition::{
    DefinitionName, DefinitionNode, SerializedDefinition, SimpleDef, SimpleDefType, VariableDef,
};
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_ir::serialized::serialized_reference::{
    SerializedCompletion, SerializedFlags, SerializedJsxElement, SerializedReference,
    SerializedReferenceIdentifier,
};
use unsnarl_ir::serialized::serialized_scope::{SerializedBlock, SerializedScope};
use unsnarl_ir::serialized::serialized_variable::SerializedVariable;
use unsnarl_ir::serialized::variable_id::SerializedVariableId;
use unsnarl_ir::serialized::{SerializedIR, SerializedSource};
use unsnarl_oxc_parity::{AstType, PredicateContainerType, VariableDeclarationKind};

use super::context::BuilderContext;
use super::write_op::WriteOp;

/// Build a `Span` at the given 0-based offset on line 1.
pub(crate) fn span(offset: u32) -> Span {
    span_at(1, offset, offset)
}

/// Build a `Span` at an explicit `(line, column, offset)`.
pub(crate) fn span_at(line: u32, column: u32, offset: u32) -> Span {
    Span::new(
        SourceLine(line),
        SourceColumn(column),
        Utf16CodeUnitOffset(offset),
    )
}

/// Wrap a string as a `SerializedScopeId`.
///
/// The wrapper asserts non-empty, so callers that need a literal
/// `""` scope id (rare, exercised by a few formatter helpers) build
/// a `SerializedScope` directly instead.
pub(crate) fn scope_id(value: &str) -> SerializedScopeId {
    SerializedScopeId::new(value)
}

/// Returns a `Block` scope with empty variables / references /
/// children, a 1-character body span, and zeroed nesting depths.
/// Tests override individual fields by binding the result and
/// mutating, e.g.
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
        callback_argument: None,
        falls_through: false,
        exits_function: false,
        nesting_depths: NestingDepths::uniform(NestingDepth(0)),
        abrupt_statements: Vec::new(),
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
        Utf16CodeUnitOffset(parent_span_offset),
        if_chain_root_offset.map(Utf16CodeUnitOffset),
    ))
}

/// Build a [`CaseClauseBlockContext`] and wrap as
/// [`BlockContext::CaseClause`]. Defaults represent a "no-overrides"
/// case clause; pass an explicit `case_test` to model a concrete
/// case.
pub(crate) fn case_clause_block_context(
    parent_type: AstType,
    key: &str,
    parent_span_offset: u32,
    case_test: Option<&str>,
) -> BlockContext {
    BlockContext::CaseClause(CaseClauseBlockContext::new(
        parent_type,
        key.to_string(),
        Utf16CodeUnitOffset(parent_span_offset),
        case_test.map(|s| s.to_string()),
    ))
}

/// Returns a baseline `WriteOp` with placeholder strings and zeroed
/// offsets; callers mutate the fields they care about for each
/// fixture.
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

/// Returns a baseline `SerializedReference` rooted in scope `s`,
/// identifier `x` at offset 0, with no completion / predicate / jsx /
/// expression-statement payload.
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

/// Build a [`PredicateContainer`] at the supplied offset.
pub(crate) fn predicate_container(
    r#type: PredicateContainerType,
    offset: u32,
) -> PredicateContainer {
    PredicateContainer::new(r#type, Utf16CodeUnitOffset(offset))
}

/// Build a [`Span`] from `(offset, line)`; the column equals the
/// offset by convention.
pub(crate) fn span_offset_line(offset: u32, line: u32) -> Span {
    span_at(line, offset, offset)
}

/// Build a [`DefinitionName`] with a non-empty name at the supplied
/// span.
pub(crate) fn definition_name(name: &str, span: Span) -> DefinitionName {
    DefinitionName::new(name.to_string(), span)
}

/// Build a [`DefinitionNode`] with the supplied AST type / span.
pub(crate) fn definition_node(r#type: AstType, span: Span) -> DefinitionNode {
    DefinitionNode { r#type, span }
}

/// Returns a baseline `Variable` definition for `x` at offset 0,
/// with no `init`.
pub(crate) fn base_def(declaration_kind: VariableDeclarationKind) -> SerializedDefinition {
    SerializedDefinition::Variable(VariableDef::new(
        definition_name("x", span(0)),
        definition_node(AstType::Identifier, span(0)),
        None,
        None,
        declaration_kind,
    ))
}

/// Returns a baseline `Simple` definition; the 5 "no-extra-fields"
/// variants reuse the `x`-at-offset-0 common shape.
pub(crate) fn base_simple_def(r#type: SimpleDefType) -> SerializedDefinition {
    SerializedDefinition::Simple(SimpleDef {
        name: definition_name("x", span(0)),
        node: definition_node(AstType::Identifier, span(0)),
        parent: None,
        r#type,
    })
}

/// Wrap a string as a `SerializedVariableId`.
pub(crate) fn variable_id(value: &str) -> SerializedVariableId {
    SerializedVariableId::new(value)
}

/// Wrap a string as a `SerializedReferenceId`.
pub(crate) fn reference_id(value: &str) -> SerializedReferenceId {
    SerializedReferenceId::new(value)
}

/// Builds a `Let`-declared variable named `x` in scope `s` with one
/// identifier span at offset 0.
pub(crate) fn base_serialized_variable() -> SerializedVariable {
    SerializedVariable::new(
        variable_id("v"),
        "x".to_string(),
        scope_id("s"),
        vec![span(0)],
        Vec::new(),
        vec![base_def(VariableDeclarationKind::Let)],
    )
}

/// Returns a `SerializedCompletion::Normal` literal.
pub(crate) fn normal_completion() -> SerializedCompletion {
    SerializedCompletion::Normal
}

/// Build a `SerializedCompletion::Return` covering the supplied
/// start / end spans.
pub(crate) fn return_completion(
    start_offset: u32,
    end_offset: u32,
    start_line: u32,
    end_line: u32,
) -> SerializedCompletion {
    SerializedCompletion::Return {
        start_span: span_offset_line(start_offset, start_line),
        end_span: span_offset_line(end_offset, end_line),
    }
}

/// Build a `SerializedCompletion::Throw` covering the supplied
/// start / end spans.
pub(crate) fn throw_completion(
    start_offset: u32,
    end_offset: u32,
    start_line: u32,
    end_line: u32,
) -> SerializedCompletion {
    SerializedCompletion::Throw {
        start_span: span_offset_line(start_offset, start_line),
        end_span: span_offset_line(end_offset, end_line),
    }
}

/// Build a [`SerializedJsxElement`] covering the supplied start /
/// end spans.
pub(crate) fn jsx_container(
    start_offset: u32,
    end_offset: u32,
    start_line: u32,
    end_line: u32,
) -> SerializedJsxElement {
    SerializedJsxElement {
        start_span: span_offset_line(start_offset, start_line),
        end_span: span_offset_line(end_offset, end_line),
    }
}

/// Build an empty [`SerializedIR`] (no scopes / variables /
/// references / diagnostics) backed by a `Ts` source at `x.ts`.
pub(crate) fn empty_serialized_ir() -> SerializedIR {
    SerializedIR {
        version: SERIALIZED_IR_VERSION,
        source: SerializedSource {
            path: "x.ts".to_string(),
            language: Language::Ts,
        },
        raw: String::new(),
        scopes: Vec::new(),
        variables: Vec::new(),
        references: Vec::new(),
        unused_variable_ids: Vec::new(),
        diagnostics: Vec::new(),
    }
}

/// Build a [`BuilderContext`] whose `variable_map` and `scope_map`
/// are derived from `ir.variables` / `ir.scopes`. Every other side
/// table is empty; callers mutate the fields they care about for
/// each fixture.
pub(crate) fn base_builder_context(ir: &SerializedIR) -> BuilderContext<'_> {
    let variable_map = ir.variables.iter().map(|v| (v.id.value(), v)).collect();
    let scope_map = ir.scopes.iter().map(|s| (s.id.value(), s)).collect();
    let mut branch_scopes_by_container: HashMap<String, Vec<&SerializedScope>> = HashMap::new();
    for s in &ir.scopes {
        if let Some(ckey) = super::branch_container_key::branch_container_key(s) {
            branch_scopes_by_container.entry(ckey).or_default().push(s);
        }
    }
    BuilderContext {
        ir,
        variable_map,
        scope_map,
        subgraph_owner_var: HashMap::new(),
        write_ops_by_variable: HashMap::new(),
        write_ops_by_scope: HashMap::new(),
        write_op_by_ref: HashMap::new(),
        sorted_cases_by_container: HashMap::new(),
        branch_scopes_by_container,
        depths: None,
        source_index: unsnarl_ir::primitive::SourceIndex::build(&ir.raw),
    }
}
