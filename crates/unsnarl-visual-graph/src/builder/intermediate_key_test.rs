//! Sibling tests for [`intermediate_key`].

use super::intermediate_key;

#[test]
fn relative_module_specifier_pairs_with_name() {
    assert_eq!(intermediate_key("./mod.js", "foo"), "./mod.js::foo");
}

#[test]
fn arbitrary_strings_pass_through_unchanged() {
    assert_eq!(intermediate_key("a/b/c", "Some Name"), "a/b/c::Some Name");
}

#[test]
fn empty_strings_yield_only_the_separator() {
    assert_eq!(intermediate_key("", ""), "::");
}

#[test]
fn bare_package_specifier_pairs_with_name() {
    assert_eq!(intermediate_key("react", "useState"), "react::useState");
}
