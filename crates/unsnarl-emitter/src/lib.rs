//! Emitter trait + `IRSerializer` trait + shared option types.
//!
//! Mirrors `ts/src/pipeline/emit/` (`emitter.ts`, `emit-options.ts`) and
//! `ts/src/pipeline/serialize/` (`ir-serializer.ts`, `serialize-context.ts`).
//!
//! Implementations of `Emitter` live in sibling `unsnarl-emitter-*`
//! crates. `FlatSerializer` (the only `IRSerializer` implementation in
//! the TS port) lives in `unsnarl-emitter-ir`.

pub mod emit_options;
pub mod emitter;
pub mod ir_serializer;
pub mod serialize_context;

pub use emit_options::EmitOptions;
pub use emitter::Emitter;
pub use ir_serializer::IRSerializer;
pub use serialize_context::{SerializeContext, SerializeSourceMeta};
