//! Predicate-container side info for a `Reference`.

use serde::Serialize;

use unsnarl_oxc_parity::PredicateContainerType;

#[derive(Serialize)]
pub struct PredicateContainer {
    pub r#type: PredicateContainerType,
    pub offset: u32,
}
