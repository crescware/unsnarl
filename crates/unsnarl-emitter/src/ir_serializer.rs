//! `IRSerializer` trait: produces a `SerializedIR` from analyzed source.
//!
//! Mirrors `IRSerializer` in `ts/src/pipeline/serialize/ir-serializer.ts`.

use unsnarl_ir::serialized::SerializedIR;

use crate::serialize_context::SerializeContext;

pub trait IRSerializer {
    fn id(&self) -> &'static str;
    fn serialize(&self, ctx: &SerializeContext) -> SerializedIR;
}
