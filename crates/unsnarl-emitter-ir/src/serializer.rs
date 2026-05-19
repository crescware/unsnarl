//! Serializer family: builds a `SerializedIR` from the in-memory
//! `IrArena` + side-table `Annotations`.
//!
//! Mirrors `ts/src/serializer/`. The four files that Step 7 (#116)
//! lifted directly into `unsnarl-ir` / `unsnarl-oxc-parity`
//! (`import-kind`, `nesting-kind`, `serialized-ir-version`,
//! `variable-declaration-kind`) are not duplicated here; the rest of
//! the TS serializer tree lives under [`flat`] as the "flat
//! serializer" variant — the only `IRSerializer` implementation TS
//! ships.

pub mod flat;
