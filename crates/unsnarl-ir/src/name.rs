//! Package name. Ports `ts/src/name.ts`.
//!
//! TS reads `pkg.name` from `package.json`; Rust pins the value at the
//! crate root.

pub const NAME: &str = "unsnarl";
