//! Visitor callbacks consumed by [`crate::analyze::analyze`].
//!
//! Mirrors `AnalysisVisitor` in `ts/src/boundary/eslint-scope/visitor.ts`.
//! The trait is intentionally empty in Step 8.5; Step 9 will add
//! `on_reference` / `on_scope` / `on_diagnostic` once their input types
//! (`ReferenceVisitInput`, `ScopeVisitInput`, `Diagnostic`) are in place.

pub trait AnalysisVisitor {
    // Step 9.
}
