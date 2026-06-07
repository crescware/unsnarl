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
