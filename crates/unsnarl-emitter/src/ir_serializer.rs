//! `IRSerializer` trait: produces a `SerializedIR` from analyzed source.

use unsnarl_ir::serialized::SerializedIR;

use crate::serialize_context::SerializeContext;

pub trait IRSerializer {
    fn id(&self) -> &'static str;
    fn serialize(&self, ctx: &SerializeContext) -> SerializedIR;
}
