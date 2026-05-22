//! Boundary-edge direction tags.
//!
//! The two tags double as on-disk JSON values (`"out"` / `"in"`).
//! Each is its own unit enum so it can sit in a struct field whose
//! serialization is a const string.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BoundaryEdgeDirectionOut {
    #[serde(rename = "out")]
    Out,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BoundaryEdgeDirectionIn {
    #[serde(rename = "in")]
    In,
}
