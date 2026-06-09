use super::*;

fn span(start: u32, end: u32) -> Span {
    Span::new(start, end)
}

fn sid(n: usize) -> ScopeId {
    ScopeId::from_usize(n)
}

#[test]
fn strictly_contains_rejects_equal_spans() {
    assert!(!strictly_contains(span(0, 10), span(0, 10)));
}

#[test]
fn strictly_contains_accepts_inner_span() {
    assert!(strictly_contains(span(0, 10), span(2, 8)));
    assert!(strictly_contains(span(0, 10), span(0, 8)));
    assert!(strictly_contains(span(0, 10), span(2, 10)));
}

#[test]
fn strictly_contains_rejects_disjoint_or_overlapping() {
    assert!(!strictly_contains(span(0, 5), span(6, 10)));
    assert!(!strictly_contains(span(0, 6), span(4, 10)));
}

#[test]
fn innermost_enclosing_arm_picks_the_tightest() {
    // outer arm [0,40) wraps inner arm [20,30); a span at [22,28)
    // should resolve to the inner arm, not the outer.
    let arms = vec![(span(0, 40), sid(5)), (span(20, 30), sid(7))];
    assert_eq!(
        innermost_enclosing_arm(&arms, span(22, 28), None),
        Some(sid(7))
    );
}

#[test]
fn innermost_enclosing_arm_skips_excluded_self() {
    // The arm must not report itself as its own enclosing arm.
    let arms = vec![(span(20, 30), sid(7))];
    assert_eq!(
        innermost_enclosing_arm(&arms, span(20, 30), Some(sid(7))),
        None
    );
}

#[test]
fn innermost_enclosing_arm_none_when_uncontained() {
    let arms = vec![(span(20, 30), sid(7))];
    assert_eq!(innermost_enclosing_arm(&arms, span(0, 50), None), None);
}

#[test]
fn contains_includes_equal_and_inner_spans() {
    assert!(contains(span(0, 10), span(0, 10)));
    assert!(contains(span(0, 10), span(2, 8)));
    assert!(contains(span(0, 10), span(0, 8)));
    assert!(contains(span(0, 10), span(2, 10)));
}

#[test]
fn contains_rejects_disjoint_or_overlapping() {
    assert!(!contains(span(0, 5), span(6, 10)));
    assert!(!contains(span(0, 6), span(4, 10)));
}

#[test]
fn innermost_arm_between_pulls_in_equal_span_arm() {
    // A bare function/arrow/class arm: the arm-root scope's span equals
    // the arm span exactly (here [20,30)). The scope's current parent is
    // the module [0,100). The equal-span arm must still be selected so
    // the scope is pulled into the arm rather than left floating.
    let arms = vec![(span(20, 30), sid(9))];
    assert_eq!(
        innermost_arm_between(&arms, span(20, 30), span(0, 100)),
        Some(sid(9))
    );
}

#[test]
fn innermost_arm_between_equal_arm_beats_wider_enclosing_arm() {
    // A nested ternary: an outer arm [0,40) strictly contains an inner
    // arm [20,30); a bare arm-root scope at [20,30) equals the inner arm.
    // The tightest (equal) arm wins over the wider enclosing one.
    let arms = vec![(span(0, 40), sid(5)), (span(20, 30), sid(7))];
    assert_eq!(
        innermost_arm_between(&arms, span(20, 30), span(0, 100)),
        Some(sid(7))
    );
}

#[test]
fn innermost_arm_between_requires_arm_inside_current_parent() {
    // A callback at [24,28) whose current parent is the module [0,100).
    // The consequent arm [20,40) sits between them and wins.
    let arms = vec![(span(20, 40), sid(9))];
    assert_eq!(
        innermost_arm_between(&arms, span(24, 28), span(0, 100)),
        Some(sid(9))
    );
}

#[test]
fn innermost_arm_between_rejects_arm_outside_parent() {
    // If the current parent [22,30) is already tighter than the arm
    // [20,40), the arm is not between scope and parent.
    let arms = vec![(span(20, 40), sid(9))];
    assert_eq!(
        innermost_arm_between(&arms, span(24, 28), span(22, 30)),
        None
    );
}

#[test]
fn innermost_arm_between_rejects_arm_equal_to_parent() {
    // Pins the asymmetric guard: the arm->span check is non-strict
    // (equal spans pull the scope into the arm), but the arm->upper_span
    // check stays strict, so an arm whose span equals the scope's current
    // parent does not sit "between" them and must be rejected.
    let arms = vec![(span(20, 30), sid(9))];
    assert_eq!(
        innermost_arm_between(&arms, span(20, 30), span(20, 30)),
        None
    );
}
