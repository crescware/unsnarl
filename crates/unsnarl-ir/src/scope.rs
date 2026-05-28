//! Scope-side IR contract types.
//!
//! `ScopeData` lives in this parent module rather than a same-named
//! child to avoid Rust's `module_inception` shape.
//!
//! Cross-entity links go through arena IDs (`upper: Option<ScopeId>`,
//! `child_scopes: Vec<ScopeId>`, etc.). The analyzer pass mutates
//! `child_scopes` / `variables` / `references` / `through` /
//! `set` during scope analysis; `set` is gated behind
//! `insert_into_set` so the "variable names are non-empty" invariant
//! cannot be violated.

use std::collections::HashMap;

use crate::ids::{ReferenceId, ScopeId, VariableId};
use crate::primitive::AstNode;
use crate::scope_type::ScopeType;

pub mod abrupt_statement;
pub mod block_context;
pub mod block_context_kind;
pub mod definition;
pub mod variable;

pub use abrupt_statement::{AbruptStatement, AbruptStatementType};
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
    set: HashMap<String, VariableId>,
    pub references: Vec<ReferenceId>,
    pub through: Vec<ReferenceId>,
    pub function_expression_scope: bool,
}

impl ScopeData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        r#type: ScopeType,
        is_strict: bool,
        upper: Option<ScopeId>,
        child_scopes: Vec<ScopeId>,
        variable_scope: ScopeId,
        block: AstNode,
        variables: Vec<VariableId>,
        references: Vec<ReferenceId>,
        through: Vec<ReferenceId>,
        function_expression_scope: bool,
    ) -> Self {
        Self {
            r#type,
            is_strict,
            upper,
            child_scopes,
            variable_scope,
            block,
            variables,
            set: HashMap::new(),
            references,
            through,
            function_expression_scope,
        }
    }

    pub fn insert_into_set(&mut self, name: String, id: VariableId) {
        assert!(
            !name.is_empty(),
            "ScopeData.set key (variable name) must be non-empty"
        );
        self.set.insert(name, id);
    }

    pub fn set(&self) -> &HashMap<String, VariableId> {
        &self.set
    }
}

pub type Scope = ScopeData;
