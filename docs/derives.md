# Derive Policy

Every `#[derive(...)]` entry must correspond to an actual in-tree use of
the trait. No anticipatory derives, no symmetry-for-symmetry's-sake
derives, no "this enum is small so `Copy` is free."

## What "used" means

A derive is justified only when at least one of the following holds
somewhere in the workspace:

- A method of the trait is called on a value of the type — `.clone()`,
  `==` / `assert_eq!`, `{:?}` formatting, `.hash(...)`, etc.
- A generic bound elsewhere requires the trait — e.g.
  `serde_json::to_string(&v)` requires `v`'s type to implement
  `Serialize`; using a value as a `HashMap` key requires `Hash + Eq`;
  `Result::unwrap_err` on `Result<T, _>` requires `T: Debug`.
- The value crosses a generated boundary that depends on the trait —
  serde `Serialize` / `Deserialize` at a serialization point, `Default`
  at a `..Default::default()` construction site, etc.

If none of these can be pointed to, delete the derive.

## When the rule changes

If a later change introduces a use that needs the derive, add the
derive at that point. Do not pre-add in anticipation.

## Why this isn't enforced by clippy

Derives expand into trait `impl` blocks, and trait impls are part of the
type's public API surface. The dead-code analysis (`dead_code`,
`unused_*`) does not cover them because any downstream consumer might
invoke them. Neither clippy nor rustc ships a lint that flags derives
whose generated capability is never exercised. The rule is therefore
enforced by review, not by tooling.
