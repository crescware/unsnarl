//! Predicate-container side info for a `Reference`.

use serde::Serialize;

use unsnarl_oxc_parity::PredicateContainerType;

use crate::primitive::Utf16CodeUnitOffset;

#[derive(Clone, Serialize)]
pub struct PredicateContainer {
    pub r#type: PredicateContainerType,
    pub offset: Utf16CodeUnitOffset,
}

impl PredicateContainer {
    pub fn new(r#type: PredicateContainerType, offset: Utf16CodeUnitOffset) -> Self {
        Self { r#type, offset }
    }
}
