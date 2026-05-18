//! Predicate-container side info for a `Reference`. Ports
//! `ts/src/ir/reference/predicate-container.ts`.

use serde::Serialize;

use crate::predicate_container_type::PredicateContainerType;

#[derive(Serialize)]
pub struct PredicateContainer {
    pub r#type: PredicateContainerType,
    pub offset: u32,
}
