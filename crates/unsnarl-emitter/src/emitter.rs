//! Emitter trait.
//!
//! Mirrors `Emitter` in `ts/src/pipeline/emit/emitter.ts`. The TS form
//! declares `format`, `contentType`, and `extension` as readonly
//! string properties on each implementation; in Rust they are
//! returned by `&self` getters so the trait stays object-safe
//! (`&dyn Emitter`) for the future registry.

use unsnarl_ir::serialized::SerializedIR;

use crate::emit_options::EmitOptions;

pub trait Emitter {
    fn format(&self) -> &'static str;
    fn content_type(&self) -> &'static str;
    fn extension(&self) -> &'static str;
    fn emit(&self, ir: &SerializedIR, opts: &EmitOptions) -> String;
}
