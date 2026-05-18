//! Scope-side IR contract types. Ports `ts/src/ir/scope/`.
//!
//! `Scope` / `ScopeData` lives in this parent module rather than a
//! same-named child (`scope/scope.rs`) to avoid Rust's
//! `module_inception` shape. TS holds direct object references
//! (`upper: Scope | null`, `childScopes: Scope[]`, etc.); the Rust IR
//! replaces those with arena IDs. Fields stay mutable in their
//! owning `IrArena`: the analyzer pass pushes onto `child_scopes` /
//! `variables` / `references` / `through` during scope analysis.

use std::collections::HashMap;

use crate::ids::{ReferenceId, ScopeId, VariableId};
use crate::primitive::AstNode;
use crate::scope_type::ScopeType;

pub mod block_context;
pub mod block_context_kind;
pub mod definition;
pub mod variable;

pub use block_context::{BlockContext, CaseClauseBlockContext, OtherBlockContext};
pub use block_context_kind::BlockContextKind;
pub use definition::{Definition, DefinitionData};
pub use variable::{Variable, VariableData};

pub struct ScopeData {
    pub r#type: ScopeType,
    pub is_strict: bool,
    pub upper: Option<ScopeId>,
    pub child_scopes: Vec<ScopeId>,
    pub variable_scope: ScopeId,
    pub block: AstNode,
    pub variables: Vec<VariableId>,
    pub set: HashMap<String, VariableId>,
    pub references: Vec<ReferenceId>,
    pub through: Vec<ReferenceId>,
    pub function_expression_scope: bool,
}

pub type Scope = ScopeData;
