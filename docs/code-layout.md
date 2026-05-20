# Code Layout

This document describes the directory and test-file conventions used in this codebase.

## Implementation lives in `<name>.rs`, not `<name>/mod.rs`

Every module is implemented as a single file named after the module — `<name>.rs` — rather than as a directory module whose entry point is `<name>/mod.rs`. Subdirectories are still permitted where modules genuinely group at finer granularity; in that case the parent module's implementation lives in a *sibling* `<name>.rs` file, not in `<name>/mod.rs`.

```text
path/to/parent/
├── child.rs            # the module's public surface + impl
└── child_test.rs       # the module's tests
```

```text
path/to/parent/
├── group.rs            # parent module: declares `pub mod leaf_a; pub mod leaf_b;`
├── group_test.rs       # tests for `group.rs` (optional)
└── group/
    ├── leaf_a.rs
    ├── leaf_a_test.rs
    ├── leaf_b.rs
    └── leaf_b_test.rs
```

### Why not `mod.rs` and `test.rs`?

A directory-module layout (`<name>/mod.rs` + `<name>/test.rs`) is a natural fit for Rust's module resolution, but in day-to-day editing it loses badly to tooling. Editor tabs, notification popups, file-tree filter results, search hit summaries, and many code-review UIs routinely display only the file's base name and elide the parent directory. A repository whose source tree is mostly `mod.rs` and `test.rs` therefore reads as an unreadable stack of identical names; switching between two open `mod.rs` tabs or attributing a failing `test.rs` to its module costs a deliberate path inspection every time. Naming each file after the module it implements puts the identifying information in the place tools actually show.

## Tests live in a sibling file, not inline

Each module's tests live in `<name>_test.rs`, wired through the impl file with a `#[cfg(test)] #[path = "..."] mod <name>_test;` declaration. They do **not** live in an inline `#[cfg(test)] mod tests { ... }` block at the bottom of the impl file, and they do **not** live in the crate-level `tests/` directory.

```rust
// path/to/foo.rs

pub struct Foo { /* ... */ }

#[cfg(test)]
#[path = "foo_test.rs"]
mod foo_test;
```

```rust
// path/to/foo_test.rs

use super::*;

#[test]
fn some_property_holds() { /* ... */ }
```

`#[path]` is resolved relative to the directory containing the file in which the `mod` declaration appears, so `#[path = "foo_test.rs"]` written in `path/to/foo.rs` points to `path/to/foo_test.rs` — a true sibling. The test module is still a child of the impl module in the module tree, so `use super::*;` continues to reach private items.

### Why not inline `#[cfg(test)] mod tests` (the idiomatic Rust shape)?

The conventional Rust pattern of writing tests in the same file as the implementation is well understood; we deliberately diverge from it.

Inline tests have real advantages: private items are reachable directly, IDE jump-to-test is trivial, and the existence of tests is visible without leaving the file. The sibling layout preserves all of these — `use super::*;` reaches privates, modern IDEs handle the jump, and the impl file advertises the sibling test module near the bottom. What sibling layout does not preserve is the bundling of test bytes into the implementation file's reads, which is precisely the cost we want to avoid.

In an AI agentic coding workflow, every read of a source file pulls the entire file into the agent's context window. When tests are inlined, every read of `foo.rs` also drags in every test for `foo.rs` — frequently several times the size of the implementation itself — even when the agent only needs to inspect or edit the implementation, and vice versa. That context cost shows up on every iteration, compounds across multi-file searches, and crowds out room for the actual task.

`unsnarl` is a code-analysis tool whose modules consistently have more test code than implementation code (each behavior tends to need many fixture-based assertions). The inline pattern therefore makes the cost above unusually high for this project specifically.

Splitting tests into a sibling `_test.rs` file lets the agent (and human readers) load the implementation alone, the tests alone, or both, according to the task at hand. The TypeScript and Go ecosystems already separate tests into sibling files (`foo.ts` + `foo.test.ts`; `foo.go` + `foo_test.go`); aligning the Rust side with that shape gives a familiar layout to anyone coming from those ecosystems, even though our motivation for the split differs.

### File name is `<module>_test.rs`, not `test.rs`

The sibling-file pattern was originally introduced with tests in `<module>/test.rs`. We have since renamed every test file to `<module>_test.rs` for the same tooling-readability reason described above: when an editor tab simply reads `test.rs`, the failing test's home module is invisible. `<module>_test.rs` keeps the impl's name in the filename and is the form most directly recognizable to readers coming from Go (`<module>_test.go`).

A top-level integration test file under `crates/<crate>/tests/` — if one is ever genuinely needed — would follow Cargo's existing conventions for that directory and is out of scope for this document.
