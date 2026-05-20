//! On-disk IR shape.

pub mod reference_id;
pub mod scope_id;
pub mod serialized_definition;
pub mod serialized_expression_statement_head;
pub mod serialized_ir;
pub mod serialized_reference;
pub mod serialized_scope;
pub mod serialized_variable;
pub mod variable_id;

pub use reference_id::SerializedReferenceId;
pub use scope_id::SerializedScopeId;
pub use serialized_definition::{
    DefinitionName, DefinitionNode, ImportBindingDefaultDef, ImportBindingNamedDef,
    ImportBindingNamespaceDef, SerializedDefinition, SimpleDef, SimpleDefType, VariableDef,
};
pub use serialized_expression_statement_head::{SerializedHeadExpression, SerializedHeadOperand};
pub use serialized_ir::{SerializedIR, SerializedSource};
pub use serialized_reference::{
    SerializedCompletion, SerializedExpressionStatementContainer, SerializedFlags,
    SerializedJsxElement, SerializedReference, SerializedReferenceIdentifier,
};
pub use serialized_scope::{SerializedBlock, SerializedScope};
pub use serialized_variable::SerializedVariable;
pub use variable_id::SerializedVariableId;
