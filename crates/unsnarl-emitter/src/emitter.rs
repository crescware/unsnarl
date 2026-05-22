//! Emitter trait.
//!
//! `format`, `content_type`, and `extension` are returned by `&self`
//! getters so the trait stays object-safe (`&dyn Emitter`) for the
//! registry.

use unsnarl_ir::serialized::SerializedIR;

use crate::emit_options::EmitOptions;

pub trait Emitter {
    fn format(&self) -> &'static str;
    fn content_type(&self) -> &'static str;
    fn extension(&self) -> &'static str;
    fn emit(&self, ir: &SerializedIR, opts: &EmitOptions) -> String;
}
