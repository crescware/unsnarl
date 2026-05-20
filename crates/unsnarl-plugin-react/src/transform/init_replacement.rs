//! Pending replacement for a wrapped variable's `defs[0].init`.
//!
//! Mirrors the inline TS type:
//!
//! ```ts
//! type InitReplacement = Readonly<{ type: AstType; span: Span }>;
//! ```
//!
//! Carries the inner function block's [`AstType`] and [`Span`] so
//! the rebuild step can write a fresh `DefinitionNode` for the
//! wrapper-peeled binding.

use unsnarl_ir::primitive::Span;
use unsnarl_oxc_parity::AstType;

pub struct InitReplacement {
    pub ty: AstType,
    pub span: Span,
}
