//! Serializer family: builds a `SerializedIR` from the in-memory
//! `IrArena` + side-table `Annotations`.
//!
//! The single `IRSerializer` implementation lives under [`flat`] as
//! the "flat serializer" variant. (`import_kind`, `nesting_kind`,
//! `serialized_ir_version`, `variable_declaration_kind` belong on
//! `unsnarl-ir` / `unsnarl-oxc-parity` and are not duplicated here.)

pub mod flat;
