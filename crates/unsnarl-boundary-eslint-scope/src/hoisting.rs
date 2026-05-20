//! Hoist-pass helpers.
//!
//! Mirrors `ts/src/boundary/eslint-scope/hoisting/`. The TS directory
//! groups `hoist-declarations.ts`, `visit.ts`, and the four
//! `handle-*-declaration.ts` per-shape handlers (`Class`, `Function`,
//! `Import`, `Variable`). The Rust port keeps the same fan-out so the
//! TS ↔ Rust correspondence is one-to-one at the file level.
//!
//! The TS sources also include `is-identifier-node.ts` and
//! `node-like.ts`, both of which compensate for the TS layer's
//! unnormalised `NodeLike` (string-typed `type` field). Their work is
//! subsumed in Rust by `oxc_ast`'s strongly-typed AST enums, so they
//! are not ported as separate modules.

pub(crate) mod handle_class_declaration;
pub(crate) mod handle_function_declaration;
pub(crate) mod handle_import_declaration;
pub(crate) mod handle_variable_declaration;
pub(crate) mod hoist_declarations;
pub(crate) mod visit;
