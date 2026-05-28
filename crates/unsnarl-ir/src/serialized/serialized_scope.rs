//! Serialized counterpart of `Scope`.

use serde::Serialize;

use crate::nesting_kind::NestingDepths;
use crate::primitive::Span;
use crate::scope::abrupt_statement::AbruptStatement;
use crate::scope::block_context::BlockContext;
use crate::scope_type::ScopeType;
use crate::serialized::reference_id::SerializedReferenceId;
use crate::serialized::scope_id::SerializedScopeId;
use crate::serialized::variable_id::SerializedVariableId;
use unsnarl_oxc_parity::AstType;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedBlock {
    pub r#type: AstType,
    pub span: Span,
    pub end_span: Span,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedScope {
    pub id: SerializedScopeId,
    pub r#type: ScopeType,
    pub is_strict: bool,
    pub upper: Option<SerializedScopeId>,
    pub child_scopes: Vec<SerializedScopeId>,
    pub variable_scope: SerializedScopeId,
    pub block: SerializedBlock,
    pub variables: Vec<SerializedVariableId>,
    pub references: Vec<SerializedReferenceId>,
    pub through: Vec<SerializedReferenceId>,
    pub function_expression_scope: bool,
    pub block_context: Option<BlockContext>,
    pub falls_through: bool,
    pub exits_function: bool,
    pub nesting_depths: NestingDepths,
    pub abrupt_statements: Vec<AbruptStatement>,
}

#[cfg(test)]
#[path = "serialized_scope_test.rs"]
mod serialized_scope_test;
