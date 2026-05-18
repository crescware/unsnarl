//! Predicate-container side info for a `Reference`.

use serde::Serialize;

use crate::predicate_container_type::PredicateContainerType;

#[derive(Serialize)]
pub struct PredicateContainer {
    pub r#type: PredicateContainerType,
    pub offset: u32,
}
