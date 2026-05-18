//! IR contract types: the in-arena rows (`ScopeData` /
//! `VariableData` / `ReferenceData` / `DefinitionData`) and the
//! on-disk shape (`serialized::*`).
//!
//! Several enums that are conceptually about analyzer / parser /
//! serializer / annotations (`AstType`, `ScopeType`,
//! `DefinitionType`, `DiagnosticKind`, `PredicateContainerType`,
//! `ImportKind`, `VariableDeclarationKind`, `NestingKind`,
//! `NestingDepths`, `SerializedIrVersion`, `FilledString`) sit in
//! this crate because the contract types reference them and
//! `unsnarl-ir` is the bottom of the dependency graph.

pub mod arena;
pub mod ast_type;
pub mod completion;
pub mod definition_type;
pub mod diagnostic;
pub mod diagnostic_kind;
pub mod filled_string;
pub mod ids;
pub mod import_kind;
pub mod language;
pub mod name;
pub mod nesting_kind;
pub mod predicate_container_type;
pub mod primitive;
pub mod reference;
pub mod scope;
pub mod scope_type;
pub mod serialized;
pub mod serialized_ir_version;
pub mod variable_declaration_kind;

pub use arena::IrArena;
pub use ast_type::{as_ast_type, AstType, UNKNOWN_AST_TYPE};
pub use definition_type::DefinitionType;
pub use diagnostic_kind::DiagnosticKind;
pub use filled_string::FilledString;
pub use ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
pub use import_kind::ImportKind;
pub use language::Language;
pub use name::NAME;
pub use nesting_kind::{NestingDepths, NestingKind};
pub use predicate_container_type::PredicateContainerType;
pub use scope_type::ScopeType;
pub use serialized_ir_version::{SerializedIrVersion, SERIALIZED_IR_VERSION};
pub use variable_declaration_kind::VariableDeclarationKind;
