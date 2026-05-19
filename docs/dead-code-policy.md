# Dead Code Suppression Policy

Dead code must be deleted, not silenced.

## The rule

The following attribute forms are forbidden anywhere in the Rust source
tree:

- `#[allow(dead_code)]`
- `#[expect(dead_code)]`
- `#![allow(dead_code)]`
- `#![expect(dead_code)]`
- Any `allow` / `expect` list that contains `dead_code` as a token
  (e.g. `#[allow(unused, dead_code)]`).

Other suppressions are unaffected. `#[allow(clippy::too_many_arguments)]`
and similar attributes targeting different lints remain valid where
their usual justification applies.

## Enforcement

`crates/no-allow-dead-code` is a workspace member whose only purpose is
to scan every `.rs` file under the workspace and panic if one of the
forbidden patterns is found. It runs automatically as part of the
project's standard verification command (`cargo test --workspace`) and
can also be invoked directly:

```sh
mise run check:no-allow-dead-code
```

When the scan finds a violation, the test panics with the offending
file, line number, and the matching line.

## Why this isn't enforced by clippy or `forbid(dead_code)`

The natural-looking shortcut — `[workspace.lints.rust] dead_code =
"forbid"` — does not work in this workspace. `clap`'s `Parser` and
`ValueEnum` derive macros expand to code that itself contains
`#[allow(dead_code)]` on synthetic items. Under `forbid`, the macro
expansion fails to compile (`E0453: allow(dead_code) incompatible with
previous forbid`), so every type using those derives in the
`unsnarl` CLI crate stops building. There is no way to scope
`forbid(dead_code)` to user-written attributes only.

Clippy's `clippy::allow_attributes` restriction lint bans every
`#[allow(...)]` uniformly. Adopting it would also reject the legitimate
suppressions targeting unrelated lints (e.g.
`#[allow(clippy::too_many_arguments)]`), which is broader than the
intent here.

The intent is narrow: forbid silencing `dead_code` specifically, while
leaving other suppressions untouched. Neither rustc's lint levels nor
clippy's restriction group expresses that intent at the granularity
required, so the rule is enforced by a project-defined test instead.

## When the rule changes

If a future change forces dead code to be retained — for example, a
type whose fields are only constructed by a third-party derive macro
that does not itself emit a suppression — raise it for review rather
than working around it locally. The policy assumes that legitimate
exceptions are rare enough to be discussed case by case; broadening
the rule prematurely would defeat its purpose.
