# Code Layout

This document describes the directory and test-file conventions used in this codebase.

## Modules are directories, not single files

Every non-trivial module is laid out as a directory module (`<name>/mod.rs`) rather than a single file (`<name>.rs`). For example:

```text
path/to/parent/
├── mod.rs              # declares `pub mod child;`
└── child/
    ├── mod.rs          # the module's public surface
    └── test.rs         # the module's tests
```

This structure is what allows tests to live in a sibling file (see the next section).

## Tests live in a sibling file, not inline

Each module's tests live in `<module>/test.rs`, wired through `#[cfg(test)] mod test;` in `<module>/mod.rs`. They do **not** live in an inline `#[cfg(test)] mod tests { ... }` block at the bottom of `mod.rs`, and they do **not** live in the crate-level `tests/` directory.

```rust
// path/to/foo/mod.rs

pub struct Foo { /* ... */ }

#[cfg(test)]
mod test;
```

```rust
// path/to/foo/test.rs

use super::*;

#[test]
fn some_property_holds() { /* ... */ }
```

### Why not inline `#[cfg(test)] mod tests` (the idiomatic Rust shape)?

The conventional Rust pattern of writing tests in the same file as the implementation is well understood; we deliberately diverge from it.

Inline tests have real advantages: private items are reachable directly, IDE jump-to-test is trivial, and the existence of tests is visible without leaving the file. The sibling layout preserves all of these — `use super::*;` reaches privates, modern IDEs handle the jump, and `mod.rs` advertises `#[cfg(test)] mod test;` near the top. What sibling layout does not preserve is the bundling of test bytes into the implementation file's reads, which is precisely the cost we want to avoid.

In an AI agentic coding workflow, every read of a source file pulls the entire file into the agent's context window. When tests are inlined, every read of `foo/mod.rs` also drags in every test for `foo/mod.rs` — frequently several times the size of the implementation itself — even when the agent only needs to inspect or edit the implementation, and vice versa. That context cost shows up on every iteration, compounds across multi-file searches, and crowds out room for the actual task.

`unsnarl` is a code-analysis tool whose modules consistently have more test code than implementation code (each behavior tends to need many fixture-based assertions). The inline pattern therefore makes the cost above unusually high for this project specifically.

Splitting tests into a sibling `test.rs` file lets the agent (and human readers) load the implementation alone, the tests alone, or both, according to the task at hand. The TypeScript and Go ecosystems already separate tests into sibling files (`foo.ts` + `foo.test.ts`; `foo.go` + `foo_test.go`); aligning the Rust side with that shape gives a familiar layout to anyone coming from those ecosystems, even though our motivation for the split differs.

### File name is `test.rs`, not `<module>_test.rs`

Inside `<module>/`, the parent directory already provides the namespace, so the test file is named `test.rs` (module path: `<crate>::...::<module>::test`) rather than repeating the module name.

A top-level integration test file under `crates/<crate>/tests/` — if one is ever genuinely needed — would follow Cargo's existing conventions for that directory and is out of scope for this document.