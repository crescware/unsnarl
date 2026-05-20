use super::*;

fn capture(pruning: Option<&[PrunePerQueryDetail]>) -> String {
    let mut buf = Vec::new();
    emit_pruning_warnings(pruning, &mut buf);
    String::from_utf8(buf).expect("stderr should be valid UTF-8")
}

#[test]
fn writes_nothing_when_pruning_is_none() {
    assert_eq!(capture(None), "");
}

#[test]
fn writes_nothing_when_pruning_is_empty() {
    let empty: Vec<PrunePerQueryDetail> = Vec::new();
    assert_eq!(capture(Some(&empty)), "");
}

#[test]
fn writes_nothing_when_every_entry_has_matched_above_zero() {
    let entries = vec![
        PrunePerQueryDetail {
            query: "render".to_string(),
            matched: 3,
        },
        PrunePerQueryDetail {
            query: "init".to_string(),
            matched: 1,
        },
    ];
    assert_eq!(capture(Some(&entries)), "");
}

#[test]
fn writes_a_warning_line_for_an_entry_with_matched_zero() {
    let entries = vec![PrunePerQueryDetail {
        query: "render".to_string(),
        matched: 0,
    }];
    assert_eq!(
        capture(Some(&entries)),
        "uns: warning: query 'render' matched 0 roots\n"
    );
}

#[test]
fn writes_one_warning_line_per_zero_match_entry_skipping_matched_ones() {
    let entries = vec![
        PrunePerQueryDetail {
            query: "render".to_string(),
            matched: 0,
        },
        PrunePerQueryDetail {
            query: "init".to_string(),
            matched: 2,
        },
        PrunePerQueryDetail {
            query: "boot".to_string(),
            matched: 0,
        },
    ];
    assert_eq!(
        capture(Some(&entries)),
        "uns: warning: query 'render' matched 0 roots\nuns: warning: query 'boot' matched 0 roots\n"
    );
}
