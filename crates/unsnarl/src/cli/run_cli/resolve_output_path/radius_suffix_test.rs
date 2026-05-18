use super::*;

#[test]
fn all_null_yields_empty_string() {
    assert_eq!(radius_suffix(None, None, None), "");
}

#[test]
fn only_descendants_yields_a_n() {
    assert_eq!(radius_suffix(Some(1), None, None), "-a1");
}

#[test]
fn only_ancestors_yields_b_n() {
    assert_eq!(radius_suffix(None, Some(2), None), "-b2");
}

#[test]
fn only_context_yields_c_n() {
    assert_eq!(radius_suffix(None, None, Some(3)), "-c3");
}

#[test]
fn descendants_and_ancestors_yields_a_n_b_m_alphabetical() {
    assert_eq!(radius_suffix(Some(1), Some(2), None), "-a1-b2");
}

#[test]
fn descendants_and_context_yields_a_n_c_m() {
    assert_eq!(radius_suffix(Some(7), None, Some(3)), "-a7-c3");
}

#[test]
fn ancestors_and_context_yields_b_n_c_m() {
    assert_eq!(radius_suffix(None, Some(2), Some(3)), "-b2-c3");
}

#[test]
fn all_three_drops_c_when_both_a_and_b_are_explicit() {
    assert_eq!(radius_suffix(Some(1), Some(2), Some(3)), "-a1-b2");
}

#[test]
fn zero_is_preserved_verbatim() {
    assert_eq!(radius_suffix(Some(0), Some(0), None), "-a0-b0");
}
