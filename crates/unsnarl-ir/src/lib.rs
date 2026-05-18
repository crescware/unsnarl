//! IR contract types: the in-arena rows (`ScopeData` /
//! `VariableData` / `ReferenceData` / `DefinitionData`) and the
//! on-disk shape (`serialized::*`).
//!
//! Several enums that are conceptually about analyzer / serializer /
//! annotations (`ScopeType`, `DefinitionType`, `DiagnosticKind`,
//! `PredicateContainerType`, `ImportKind`, `VariableDeclarationKind`,
//! `NestingKind`, `NestingDepths`, `SerializedIrVersion`) sit in
//! this crate because the contract types reference them and
//! `unsnarl-ir` is the bottom of the dependency graph.
//!
//! `AstType` is deliberately NOT here: it has to match `oxc_ast`
//! value-for-value, so it lives in its own crate (`unsnarl-ast-type`)
//! to keep that parity responsibility from leaking into the IR
//! contract.
//!
//! Non-empty string invariants are enforced at construction sites
//! (newtype `new` for IDs, `new` constructors on structs / enum
//! variants that carry "must be non-empty" `String` fields) rather
//! than through a separate branded type.

pub mod arena;
pub mod completion;
pub mod definition_type;
pub mod diagnostic;
pub mod diagnostic_kind;
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
pub use definition_type::DefinitionType;
pub use diagnostic_kind::DiagnosticKind;
pub use ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
pub use import_kind::ImportKind;
pub use language::Language;
pub use name::NAME;
pub use nesting_kind::{NestingDepths, NestingKind};
pub use predicate_container_type::PredicateContainerType;
pub use scope_type::ScopeType;
pub use serialized_ir_version::{SerializedIrVersion, SERIALIZED_IR_VERSION};
pub use variable_declaration_kind::VariableDeclarationKind;
