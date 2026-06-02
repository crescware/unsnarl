# Development Rules

## Project Conventions

Before starting any work, read every file under `docs/`. The conventions
recorded there (directory layout, test placement, etc.) are mandatory
and are not duplicated in this file.

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

**`check` is the mandatory gate for every commit — it must pass green before
you commit, and skipping it is never acceptable.** It is also NOT cheap: it
builds and runs the entire workspace test suite and can take many minutes, so
do not run it more than once on the same diff. A single run already captures
every step's and every crate's full output to disk under `target/check/`, so
when something FAILs you read the saved per-crate log to find the cause — you
do not re-run just to see the same failure again. Re-running after you actually
change the code is of course expected: a fix is a new diff that must be
re-checked before committing. What wastes time is firing it twice over an
identical, unchanged tree — call it the split-run mistake: one run to grab the
failures, a second run to tally the counts. But the one summary already lists
BOTH the per-crate passed/failed counts AND every failing test's name, so the
split run is pure waste — the counts and the names both came out of that single
run. You never run `check` a second time on an unchanged tree to "collect
more"; there is nothing more to collect.

Run the project check — once — and read its summary:

```bash
mise run check
```

`mise run check` runs the canonical chain a single time — `cargo fmt --all`,
then `cargo clippy --workspace --all-targets -- -D warnings`, then
`cargo test -p <member>` for every workspace member (the `no-allow-dead-code`
crate is one of those members, so its scan runs here too). Do not run the
underlying `cargo` commands by hand.
