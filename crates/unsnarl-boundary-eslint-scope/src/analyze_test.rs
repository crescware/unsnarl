//! Tests for the [`analyze`] skeleton (Step 8.5).
//!
//! Step 9 will port the full `ts/src/boundary/eslint-scope/*.test.ts` suite
//! here. For the skeleton, we only verify that the entry function is
//! callable from a freshly parsed [`ParsedSource`] and panics with the
//! documented `todo!()` payload — i.e. the parser → scope-builder hand-off
//! type-checks today and the `ParsedSource` API surface
//! (`program` / `source_type` / `raw`) is necessary and sufficient.
//!
//! [`ParsedSource`]: crate::parser::ParsedSource

use oxc_allocator::Allocator;
use unsnarl_ir::Language;

use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::visitor::AnalysisVisitor;

struct NoopVisitor;
impl AnalysisVisitor for NoopVisitor {}

#[test]
#[should_panic(expected = "Step 9")]
fn analyze_entry_is_reachable_from_parsed_source() {
    let allocator = Allocator::default();
    let code = "const x = 1;\n";
    let parsed = OxcParser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .unwrap();
    let _ = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &NoopVisitor,
    );
}
