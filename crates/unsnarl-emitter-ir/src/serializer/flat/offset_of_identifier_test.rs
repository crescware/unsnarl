//! Mirrors `ts/src/serializer/flat/offset-of.test.ts` for the
//! `AstIdentifier` arm of `offsetOf`. The TS test pinned three cases:
//! `node.start` defined, `node.start` absent (fell back to 0), and
//! `node.start === 0` preserved. In the Rust IR the span is always
//! present, so the "absent" case collapses into the same code path
//! as the explicit zero — the two tests that still carry distinct
//! meaning are "non-zero start round-trips" and "start of 0 is
//! preserved verbatim".

use oxc_span::Span as OxcSpan;
use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_oxc_parity::AstType;

use super::offset_of_identifier;

fn identifier_at(start: u32) -> AstIdentifier {
    AstIdentifier::new(
        AstType::Identifier,
        "x".to_string(),
        OxcSpan::new(start, start + 1),
    )
}

#[test]
fn returns_span_start_when_nonzero() {
    assert_eq!(offset_of_identifier(&identifier_at(42)), 42);
}

#[test]
fn preserves_zero_does_not_coerce() {
    assert_eq!(offset_of_identifier(&identifier_at(0)), 0);
}
