use super::*;
use unsnarl_ir::SourceLine;
use unsnarl_visual_graph::prune::ResolvedAs;

fn capture(resolutions: Option<&[RootQueryResolution]>) -> String {
    let mut buf = Vec::new();
    emit_resolution_notices(resolutions, &mut buf);
    String::from_utf8(buf).expect("stderr should be valid UTF-8")
}

#[test]
fn writes_nothing_when_resolutions_is_none() {
    assert_eq!(capture(None), "");
}

#[test]
fn writes_nothing_when_resolutions_is_empty() {
    let empty: Vec<RootQueryResolution> = Vec::new();
    assert_eq!(capture(Some(&empty)), "");
}

#[test]
fn writes_the_identifier_match_notice_exactly() {
    let resolutions = vec![RootQueryResolution {
        raw: "L12".to_string(),
        line: SourceLine(12),
        name: "L12".to_string(),
        resolved_as: ResolvedAs::Name,
    }];
    assert_eq!(
        capture(Some(&resolutions)),
        concat!(
            "uns: 'L12' is ambiguous.\n",
            "  An exact identifier match was found; interpreting as identifier.\n",
            "  To disambiguate, use '-r 12'.\n",
        )
    );
}

#[test]
fn writes_the_line_fallback_notice_exactly() {
    let resolutions = vec![RootQueryResolution {
        raw: "L12".to_string(),
        line: SourceLine(12),
        name: "L12".to_string(),
        resolved_as: ResolvedAs::Line,
    }];
    assert_eq!(
        capture(Some(&resolutions)),
        concat!(
            "uns: 'L12' is ambiguous.\n",
            "  No exact identifier match was found; interpreting as line number.\n",
            "  To disambiguate, use '-r 12'.\n",
        )
    );
}

#[test]
fn writes_one_notice_block_per_resolution_in_order() {
    let resolutions = vec![
        RootQueryResolution {
            raw: "L1".to_string(),
            line: SourceLine(1),
            name: "L1".to_string(),
            resolved_as: ResolvedAs::Name,
        },
        RootQueryResolution {
            raw: "L2".to_string(),
            line: SourceLine(2),
            name: "L2".to_string(),
            resolved_as: ResolvedAs::Line,
        },
    ];
    assert_eq!(
        capture(Some(&resolutions)),
        concat!(
            "uns: 'L1' is ambiguous.\n",
            "  An exact identifier match was found; interpreting as identifier.\n",
            "  To disambiguate, use '-r 1'.\n",
            "uns: 'L2' is ambiguous.\n",
            "  No exact identifier match was found; interpreting as line number.\n",
            "  To disambiguate, use '-r 2'.\n",
        )
    );
}
