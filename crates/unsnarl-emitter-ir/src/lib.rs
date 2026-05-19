//! IR JSON emitter.
//!
//! Hosts the `FlatSerializer` (mirroring `ts/src/serializer/flat/`)
//! that converts an analyzed `IrArena` + `Annotations` pair into a
//! `SerializedIR`, and the `IrEmitter` (mirroring
//! `ts/src/emitter/ir/ir.ts`) that renders that `SerializedIR` to
//! JSON. Both are exposed at the crate root so downstream consumers
//! pull them through `unsnarl_emitter_ir::{FlatSerializer, IrEmitter}`.

pub mod ir;
pub mod serializer;

pub use ir::IrEmitter;
pub use serializer::flat::FlatSerializer;
