//! Lookup contract for the IR side tables.
//!
//! The analyzer implements this trait against an arena-id keyed map.
//! Downstream consumers (serializers, emitters) hold `&dyn Annotations`
//! so they cannot reach into the analyzer's storage representation.
//!
//! Lookups take arena IDs because cross-entity linkage uses `*Id`
//! newtypes against `IrArena`, and the IR is kept `'a`-free. ID
//! values are `Copy`, so passing them by value is the natural shape.
//!
//! "Missing entries return zero-value defaults so callers do not
//! need to special-case absence" — expressing this in the trait
//! signature would constrain how implementors store the defaults,
//! so the contract is left at the doc-comment level here and
//! enforced by each implementor.

use unsnarl_ir::{ReferenceId, ScopeId, VariableId};

use crate::reference_annotation::ReferenceAnnotation;
use crate::scope_annotation::ScopeAnnotation;
use crate::variable_annotation::VariableAnnotation;

pub trait Annotations {
    fn of_reference(&self, id: ReferenceId) -> &ReferenceAnnotation;
    fn of_scope(&self, id: ScopeId) -> &ScopeAnnotation;
    fn of_variable(&self, id: VariableId) -> &VariableAnnotation;
}
