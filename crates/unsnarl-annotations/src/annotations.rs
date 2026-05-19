//! Lookup contract for the IR side tables.
//!
//! The analyzer (Step 11, #120) implements this trait against an
//! arena-id keyed map. Downstream consumers (serializers, emitters)
//! hold `&dyn Annotations` so they cannot reach into the analyzer's
//! storage representation.
//!
//! The TS source interface (`ts/src/ir/annotations/annotations.ts`)
//! takes object references (`Reference` / `Scope` / `Variable`); the
//! Rust trait takes arena IDs because Step 7 (#116) moved
//! cross-entity linkage to `*Id` newtypes against `IrArena`, and
//! keeping the IR `'a`-free was a Step 7 invariant. ID values are
//! `Copy`, so passing them by value is the natural shape.
//!
//! The TS comment "Missing entries return zero-value defaults so
//! callers do not need to special-case absence" describes a behavior
//! the implementation must uphold; expressing it in the trait
//! signature would constrain how implementors store the defaults, so
//! the contract is left at the doc-comment level here and enforced
//! by each implementor (Step 11).

use unsnarl_ir::{ReferenceId, ScopeId, VariableId};

use crate::reference_annotation::ReferenceAnnotation;
use crate::scope_annotation::ScopeAnnotation;
use crate::variable_annotation::VariableAnnotation;

pub trait Annotations {
    fn of_reference(&self, id: ReferenceId) -> &ReferenceAnnotation;
    fn of_scope(&self, id: ScopeId) -> &ScopeAnnotation;
    fn of_variable(&self, id: VariableId) -> &VariableAnnotation;
}
