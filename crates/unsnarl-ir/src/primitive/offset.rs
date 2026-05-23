//! Encoding-distinguished source-offset newtypes.
//!
//! Source positions cross two encoding boundaries inside this
//! workspace:
//!
//! * `oxc_parser` exposes `Span::start` / `Span::end` as **UTF-8
//!   byte offsets** into the original source string.
//! * The IR (`Span.offset`, `SourceColumn`, `block_context`'s
//!   `parent_span_offset`, etc.) carries **UTF-16 code-unit offsets**
//!   so that the on-disk JSON matches JavaScript-string indexing
//!   semantics — JS `String#charCodeAt` and friends operate in
//!   UTF-16 code units.
//!
//! A bare `u32` cannot tell the two apart, so it is easy to feed a
//! UTF-8 byte offset to a function that expects UTF-16 (or vice
//! versa). The bug surfaces only on source files containing
//! non-ASCII text (surrogate pairs, multi-byte UTF-8 code points)
//! and is invisible to the type checker.
//!
//! These two newtypes encode the distinction so the type checker
//! refuses the swap. Both are `#[serde(transparent)]` so the
//! on-disk JSON keeps emitting a bare number — the type-level
//! discipline is internal.
//!
//! ## Conversion sites
//!
//! * The boundary in `unsnarl-oxc-boundary` (where `oxc Span` lands
//!   in the IR) is the **only** place that should translate
//!   [`Utf8ByteOffset`] → [`Utf16CodeUnitOffset`]; the conversion is
//!   performed by [`crate::primitive::SourceIndex::span_at`].
//! * Every IR consumer (analyzer, emitters, visual-graph builder)
//!   should only see [`Utf16CodeUnitOffset`] and never construct or
//!   consume a [`Utf8ByteOffset`].

use serde::Serialize;

/// Byte offset into the source string when treated as UTF-8 bytes.
/// This is the encoding `oxc_parser` produces (its `Span::start` /
/// `Span::end` are UTF-8 byte indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Utf8ByteOffset(pub u32);

/// Code-unit offset into the source string when treated as a
/// JavaScript string (UTF-16). This is the encoding carried by every
/// `Span.offset` in the IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Utf16CodeUnitOffset(pub u32);
