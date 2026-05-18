# Development Rules

## Verification Command

Upon completing any work, the project-defined verification command for the
implementation in use must be executed without exception. The concrete command
lives in the per-implementation CLAUDE.md.

## Prohibited Actions

- All linters, checkers, and test runners required for this project are
  already provided as project-defined scripts. Do not reach for tools outside
  of those scripts for those purposes. General-purpose shell utilities such as
  `grep`, `find`, and `cat` remain free to use.
- The provided script collection reflects deliberate design intent; for tasks
  covered by those scripts, use only the scripts that are already defined
  rather than assembling your own equivalents.
- Do not invent your own verification methods. Trust the provided commands as
  the source of truth for validating work.

## Verification Command (Rust implementation at <root>)

```bash
cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

TS 実装の検証コマンドは `ts/CLAUDE.md` を参照。
