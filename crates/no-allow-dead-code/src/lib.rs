//! Source-tree guard that forbids `#[allow(dead_code)]` and its
//! variants. See `docs/dead-code-policy.md` for the rule, the
//! enforcement mechanism, and why this is not expressed as a
//! rustc/clippy lint level.

pub mod no_allow_dead_code;
