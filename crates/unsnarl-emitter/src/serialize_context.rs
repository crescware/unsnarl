//! `SerializeContext`: the bundle of inputs passed to `IRSerializer::serialize`.
//!
//! Mirrors `SerializeContext` in `ts/src/pipeline/serialize/serialize-context.ts`.

use unsnarl_annotations::Annotations;
use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::language::Language;
use unsnarl_ir::IrArena;

pub struct SerializeSourceMeta {
    pub path: String,
    pub language: Language,
}

pub struct SerializeContext<'a> {
    pub arena: &'a IrArena,
    pub root_scope: ScopeId,
    pub annotations: &'a dyn Annotations,
    pub source: SerializeSourceMeta,
    pub diagnostics: &'a [Diagnostic],
    pub raw: &'a str,
}
