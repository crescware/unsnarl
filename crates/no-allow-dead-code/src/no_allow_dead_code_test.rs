use super::*;

// Build the forbidden attribute strings dynamically so that this test file
// itself does not contain a literal `#[allow(dead_code)]` / `#[expect(...)]`,
// which would otherwise be flagged when the workspace-wide test scans here.
fn attr(outer: &str, kind: &str, body: &str) -> String {
    format!("#{outer}[{kind}({body})]")
}

#[test]
fn flags_outer_allow_with_only_dead_code() {
    assert!(line_has_forbidden_dead_code(&attr(
        "",
        "allow",
        "dead_code"
    )));
}

#[test]
fn flags_inner_allow_with_only_dead_code() {
    assert!(line_has_forbidden_dead_code(&attr(
        "!",
        "allow",
        "dead_code"
    )));
}

#[test]
fn flags_outer_expect_with_only_dead_code() {
    assert!(line_has_forbidden_dead_code(&attr(
        "",
        "expect",
        "dead_code"
    )));
}

#[test]
fn flags_inner_expect_with_only_dead_code() {
    assert!(line_has_forbidden_dead_code(&attr(
        "!",
        "expect",
        "dead_code"
    )));
}

#[test]
fn flags_dead_code_first_in_list() {
    assert!(line_has_forbidden_dead_code(&attr(
        "",
        "allow",
        "dead_code, unused"
    )));
}

#[test]
fn flags_dead_code_not_first_in_list() {
    assert!(line_has_forbidden_dead_code(&attr(
        "",
        "allow",
        "unused, dead_code"
    )));
}

#[test]
fn does_not_flag_unrelated_allow() {
    assert!(!line_has_forbidden_dead_code(&attr(
        "",
        "allow",
        "clippy::too_many_arguments"
    )));
    assert!(!line_has_forbidden_dead_code(&attr("", "allow", "unused")));
}

#[test]
fn does_not_flag_token_with_dead_code_substring() {
    // `dead_code_like` shares a prefix with `dead_code` but must not match,
    // because the body is tokenized before comparison.
    assert!(!line_has_forbidden_dead_code(&attr(
        "",
        "allow",
        "dead_code_like"
    )));
}

#[test]
fn does_not_flag_bare_mention_of_dead_code() {
    // A doc comment or string literal that happens to contain the words
    // "dead_code" outside an allow/expect attribute is not a violation.
    assert!(!line_has_forbidden_dead_code(
        "// note: this triggers dead_code if removed"
    ));
}

#[test]
fn workspace_has_no_dead_code_suppression() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    // `CARGO_MANIFEST_DIR` is `<workspace>/crates/unsnarl-source-policy`;
    // climb two levels to reach the workspace root.
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root must exist above this crate");

    // Skip this crate itself: its tests construct the forbidden attribute
    // strings as test inputs, which would otherwise self-trigger.
    let violations = find_violations(workspace_root, &["no-allow-dead-code"]);

    if !violations.is_empty() {
        let mut msg = String::from(
            "Found forbidden dead_code lint suppression(s). \
             Delete the dead code instead of silencing the lint.\n",
        );
        for v in &violations {
            msg.push_str(&format!(
                "  {}:{}: {}\n",
                v.path.display(),
                v.line_number,
                v.line.trim()
            ));
        }
        panic!("{msg}");
    }
}
