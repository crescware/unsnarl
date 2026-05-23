//! Predicate-container side info for a `Reference`.

use serde::Serialize;

use unsnarl_oxc_parity::PredicateContainerType;

use crate::primitive::Utf16CodeUnitOffset;

#[derive(Clone, Serialize)]
pub struct PredicateContainer {
    pub r#type: PredicateContainerType,
    pub offset: Utf16CodeUnitOffset,
}
