//! ECMAScript Completion Record types.
//!
//! `Completion` / `AbruptCompletion` live in this parent module
//! rather than a same-named child to avoid Rust's `module_inception`
//! shape.
//!
//! `[[Target]]` is mirrored on `Break` / `Continue` only; `Return` /
//! `Throw` carry no target in the spec. `[[Value]]` is intentionally
//! NOT mirrored — unsnarl carries value flow through Reference rows.

pub mod completion_type;

pub use completion_type::CompletionType;

pub enum Completion {
    Normal,
    Abrupt(AbruptCompletion),
}

pub enum AbruptCompletion {
    Return {
        start_offset: u32,
        end_offset: u32,
    },
    Throw {
        start_offset: u32,
        end_offset: u32,
    },
    Break {
        target: Option<String>,
        start_offset: u32,
        end_offset: u32,
    },
    Continue {
        target: Option<String>,
        start_offset: u32,
        end_offset: u32,
    },
}
