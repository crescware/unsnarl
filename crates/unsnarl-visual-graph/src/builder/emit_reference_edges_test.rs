//! Sibling tests for [`read_belongs_to_arm`].
//!
//! The helper gates a ternary-arm CallProxy: a read at `ref_offset`
//! belongs to the arm only when the offset is inside the recorded
//! `[start, end)` span. `None` means there is no arm (an ordinary,
//! non-ternary binding), so the proxy claims every read. The span is
//! half-open — the start byte is inside the arm, the end byte is not —
//! so the boundary cases are pinned directly here.

use super::read_belongs_to_arm;

#[test]
fn none_arm_claims_every_read() {
    assert!(read_belongs_to_arm(None, 0));
    assert!(read_belongs_to_arm(None, 12345));
}

#[test]
fn offset_inside_the_arm_belongs() {
    assert!(read_belongs_to_arm(Some(&(10, 20)), 15));
}

#[test]
fn start_boundary_is_inclusive() {
    assert!(read_belongs_to_arm(Some(&(10, 20)), 10));
}

#[test]
fn last_offset_before_end_belongs() {
    assert!(read_belongs_to_arm(Some(&(10, 20)), 19));
}

#[test]
fn end_boundary_is_exclusive() {
    assert!(!read_belongs_to_arm(Some(&(10, 20)), 20));
}

#[test]
fn offset_before_start_does_not_belong() {
    assert!(!read_belongs_to_arm(Some(&(10, 20)), 9));
}

#[test]
fn offset_after_end_does_not_belong() {
    assert!(!read_belongs_to_arm(Some(&(10, 20)), 25));
}
