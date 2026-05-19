//! Tests for the [`analyze`] entry function.
//!
//! Step 8.5 only verified that `analyze` panicked with a documented
//! `todo!()` payload. Step 9's first commit replaces that placeholder
//! with a real seeded `ScopeBuilderState`, so the test is updated to
//! assert that `analyze` now completes for a trivial parsed source.
//! Later commits will move from "completes without panicking" to
//! observable IR assertions (Step 9.8 ports the TS `*.test.ts` suite
//! in full).
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
fn analyze_completes_for_trivial_source() {
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
    let mut visitor = NoopVisitor;
    let _ = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
}
