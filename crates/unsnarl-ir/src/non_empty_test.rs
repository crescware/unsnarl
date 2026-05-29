use super::*;

#[test]
fn passes_for_non_empty() {
    assert_non_empty("x", "Field.name");
}

#[test]
#[should_panic(expected = "Field.name must be non-empty")]
fn panics_for_empty() {
    assert_non_empty("", "Field.name");
}
