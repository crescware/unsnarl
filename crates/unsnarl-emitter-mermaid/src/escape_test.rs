use super::escape;

#[test]
fn ampersand_is_escaped() {
    assert_eq!(escape("a&b"), "a&amp;b");
}

#[test]
fn double_quote_is_escaped() {
    assert_eq!(escape("a\"b"), "a&quot;b");
}

#[test]
fn less_than_is_escaped() {
    assert_eq!(escape("a<b"), "a&lt;b");
}

#[test]
fn greater_than_is_escaped() {
    assert_eq!(escape("a>b"), "a&gt;b");
}

#[test]
fn all_four_together() {
    assert_eq!(escape("&\"<>"), "&amp;&quot;&lt;&gt;");
}

#[test]
fn ampersand_first_ordering_preserves_nested_entities() {
    assert_eq!(escape("&lt;"), "&amp;lt;");
}

#[test]
fn no_op_for_plain_ascii() {
    assert_eq!(escape("abc 123"), "abc 123");
}

#[test]
fn empty_string() {
    assert_eq!(escape(""), "");
}

#[test]
fn does_not_escape_single_quotes() {
    assert_eq!(escape("a'b"), "a'b");
}
