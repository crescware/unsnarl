use super::*;

#[test]
fn creates_a_single_element_vec_on_first_push() {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    push_to(&mut m, "a", "x".to_string());
    assert_eq!(m.get("a"), Some(&vec!["x".to_string()]));
}

#[test]
fn appends_to_existing_vec_on_subsequent_pushes_in_order() {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    push_to(&mut m, "a", "x".to_string());
    push_to(&mut m, "a", "y".to_string());
    push_to(&mut m, "a", "z".to_string());
    assert_eq!(
        m.get("a"),
        Some(&vec!["x".to_string(), "y".to_string(), "z".to_string()])
    );
}

#[test]
fn does_not_affect_other_keys() {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    push_to(&mut m, "a", "x".to_string());
    push_to(&mut m, "b", "y".to_string());
    assert_eq!(m.get("a"), Some(&vec!["x".to_string()]));
    assert_eq!(m.get("b"), Some(&vec!["y".to_string()]));
}

#[test]
fn preserves_duplicate_values_verbatim() {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    push_to(&mut m, "k", "v".to_string());
    push_to(&mut m, "k", "v".to_string());
    assert_eq!(m.get("k"), Some(&vec!["v".to_string(), "v".to_string()]));
}
