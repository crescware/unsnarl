use super::*;

#[test]
fn appends_a_single_short_name() {
    assert_eq!(collect_plugins("react", &[]), vec!["react".to_string()]);
}

#[test]
fn strips_the_unsnarl_plugin_prefix_when_present() {
    assert_eq!(
        collect_plugins("unsnarl-plugin-react", &[]),
        vec!["react".to_string()]
    );
}

#[test]
fn splits_a_comma_delimited_value_into_multiple_entries() {
    assert_eq!(
        collect_plugins("react,vue", &[]),
        vec!["react".to_string(), "vue".to_string()]
    );
}

#[test]
fn trims_surrounding_whitespace_per_entry() {
    assert_eq!(
        collect_plugins(" react , vue ", &[]),
        vec!["react".to_string(), "vue".to_string()]
    );
}

#[test]
fn drops_empty_fragments_from_consecutive_commas() {
    assert_eq!(
        collect_plugins("react,,vue", &[]),
        vec!["react".to_string(), "vue".to_string()]
    );
}

#[test]
fn deduplicates_repeated_names_within_a_single_value() {
    assert_eq!(
        collect_plugins("react,vue,react", &[]),
        vec!["react".to_string(), "vue".to_string()]
    );
}

#[test]
fn deduplicates_names_already_present_in_prev() {
    let prev = vec!["react".to_string()];
    assert_eq!(collect_plugins("react", &prev), vec!["react".to_string()]);
}

#[test]
fn treats_prefixed_form_as_same_name_as_short_form_for_dedup() {
    let prev = vec!["react".to_string()];
    assert_eq!(
        collect_plugins("unsnarl-plugin-react", &prev),
        vec!["react".to_string()]
    );
}

#[test]
fn returns_prev_unchanged_for_empty_value() {
    let prev = vec!["react".to_string()];
    assert_eq!(collect_plugins("", &prev), vec!["react".to_string()]);
}

#[test]
fn accumulates_across_repeated_invocations() {
    let first = collect_plugins("react", &[]);
    let second = collect_plugins("vue", &first);
    assert_eq!(second, vec!["react".to_string(), "vue".to_string()]);
}
