//! Format a three-line resolution notice shared by stderr output and
//! the markdown Notice section.
//!
//! Mirrors `ts/src/visual-graph/prune/format-resolution-notice.ts`.

use crate::prune::root_query_resolution::{ResolvedAs, RootQueryResolution};

pub fn format_resolution_notice(r: &RootQueryResolution) -> Vec<String> {
    let second = match r.resolved_as {
        ResolvedAs::Name => "An exact identifier match was found; interpreting as identifier.",
        ResolvedAs::Line => "No exact identifier match was found; interpreting as line number.",
    };
    vec![
        format!("uns: '{}' is ambiguous.", r.raw),
        format!("  {second}"),
        format!("  To disambiguate, use '-r {}'.", r.line.0),
    ]
}
