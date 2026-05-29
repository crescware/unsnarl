//! IR contract types: the in-arena rows (`ScopeData` /
//! `VariableData` / `ReferenceData` / `DefinitionData`) and the
//! on-disk shape (`serialized::*`).
//!
//! Several enums that are conceptually about analyzer / serializer /
//! annotations (`ScopeType`, `DefinitionType`, `DiagnosticKind`,
//! `ImportKind`, `NestingKind`, `NestingDepths`) sit in this crate
//! because the contract types reference them and `unsnarl-ir` is
//! the bottom of the dependency graph. Their value sets are
//! unsnarl-side (curated by us, not parroted from oxc), so
//! co-locating them here does not entangle the IR contract with
//! oxc parity.
//!
//! `AstType`, `VariableDeclarationKind`, and `PredicateContainerType`
//! are deliberately NOT here: their values must match `oxc_ast`
//! value-for-value, so they live in `unsnarl-oxc-parity` to keep that
//! parity responsibility from leaking into the IR contract. See
//! that crate's doc comment for the membership criterion.
//!
//! `SerializedIrVersion` was previously its own module; the constant
//! and type alias now live directly inside
//! `serialized::serialized_ir` because `SerializedIR.version` is the
//! sole consumer and the indirection added no value.
//!
//! `NAME` (the CLI tool name) was previously exported here; it has
//! been removed because the IR contract has no interest in the
//! binary's name. The CLI binary uses `env!("CARGO_PKG_NAME")`
//! directly at its call site.
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
pub mod nesting_kind;
mod non_empty;
pub mod primitive;
pub mod reference;
pub mod scope;
pub mod scope_type;
pub mod serialized;

pub use arena::IrArena;
pub use definition_type::DefinitionType;
pub use diagnostic_kind::DiagnosticKind;
pub use ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
pub use import_kind::ImportKind;
pub use language::Language;
pub use nesting_kind::{NestingDepth, NestingDepths, NestingKind};
pub use primitive::{SourceColumn, SourceLine, Utf16CodeUnitOffset, Utf8ByteOffset};
pub use scope_type::ScopeType;
