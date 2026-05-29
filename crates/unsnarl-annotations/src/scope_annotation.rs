//! Side-table row for `ScopeData`.
//!
//! Field order matches the source interface (`blockContext`,
//! `callbackArgument`, `fallsThrough`, `exitsFunction`,
//! `nestingDepths`, `abruptStatements`); `SerializedScope` reads each
//! field individually at serialize time, so the order here is fixed
//! by `SerializedScope`'s declaration rather than by a `Serialize`
//! derive on this struct.
//!
//! `Serialize` is intentionally not derived. `callback_argument`'s
//! in-memory [`CallbackArgument`] carries the UTF-8 `HeadExpression`
//! callee, whose span-based on-disk form is produced only inside
//! `SerializedCallbackArgument` at serialize time -- the same
//! reasoning that keeps `Serialize` off [`super::ReferenceAnnotation`]
//! (see its module doc). Deriving here would force a second JSON
//! form that no pipeline path consumes, conflicting with the
//! workspace derive policy (`docs/derives.md`).

use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_ir::scope::{AbruptStatement, BlockContext, CallbackArgument};

pub struct ScopeAnnotation {
    pub block_context: Option<BlockContext>,
    pub callback_argument: Option<CallbackArgument>,
    pub falls_through: bool,
    pub exits_function: bool,
    pub nesting_depths: NestingDepths,
    pub abrupt_statements: Vec<AbruptStatement>,
}
