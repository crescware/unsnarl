//! `GenerationCount`: how many ancestor / descendant generations to
//! pull in around an anchor.
//!
//! Used in two places that have the same semantic meaning:
//!
//! * `RootQuery::Direction.level` -- the `N` in `-r foo+aN` /
//!   `+bN` / `+cN` query tokens.
//! * CLI flags `-A` / `-B` / `-C` (descendants / ancestors /
//!   context generations).
//!
//! Distinct from `NestingDepth` (defined in `unsnarl-ir`): a
//! generation count measures graph-traversal distance from an
//! anchor, whereas a nesting depth measures lexical scope nesting.
//! They are both non-negative integers but should not be assignable
//! to each other; the type system enforces that here.
//!
//! `#[serde(transparent)]` keeps the on-disk JSON shape a bare
//! number for the test fixtures that round-trip the parsed query.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct GenerationCount(pub u32);
