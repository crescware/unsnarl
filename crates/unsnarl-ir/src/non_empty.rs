//! Shared construction-site guard for the "must be non-empty" `String`
//! invariants recorded in `lib.rs`.
//!
//! The invariant is still enforced at each `new` constructor (no
//! branded type — see the `lib.rs` doc comment); this helper only
//! factors out the repeated check and the message convention so the
//! ~20 sites stop re-spelling `assert!(!x.is_empty(), "... must be
//! non-empty")` by hand.

/// Panics if `value` is empty.
///
/// `field` names the offending field for the panic message (e.g.
/// `"AstIdentifier.name"`); the `" must be non-empty"` suffix is
/// appended here so the per-site diagnostic is preserved without each
/// site repeating it. `#[track_caller]` makes the panic report the
/// calling `new` constructor's location rather than this helper's.
#[track_caller]
pub(crate) fn assert_non_empty(value: &str, field: &str) {
    assert!(!value.is_empty(), "{field} must be non-empty");
}

#[cfg(test)]
#[path = "non_empty_test.rs"]
mod non_empty_test;
