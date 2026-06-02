# Fixtures

This document describes how the snapshot fixtures under
`integration/fixtures/` are laid out, how a new one is added, and why
the baselines are generated rather than hand-written.

## Where fixtures live

Every fixture is a directory under `integration/fixtures/<category>/…`,
where `<category>` is one of:

```text
app-behavior  callback  class  declaration  exports
expression-statement  function  if-statement  imports
iteration-statement  jsx  switch-statement  try-statement
```

Nesting below the category is free-form (`callback/nested-map/…`,
`declaration/const/chain-five/…`). A directory becomes a fixture the
moment it contains an `input.*` file.

## What you write by hand: only `input.*`

A fixture is seeded by a single source file named `input` with one of
the supported extensions:

```text
.ts  .tsx  .js  .jsx  .mjs  .cjs
```

Everything else in the directory is **generated** — you never write or
edit it by hand:

| File                | Emitter format | Meaning                                  |
| ------------------- | -------------- | ---------------------------------------- |
| `expected.ir.json`  | `ir`           | The parsed/analyzed intermediate repr.   |
| `expected.json`     | `json`         | The flattened graph serialization.       |
| `expected.mermaid`  | `mermaid`      | The Mermaid diagram source.              |
| `expected.stats`    | `stats`        | Node/edge count summary.                 |
| `preview.md`        | `markdown`     | Human-readable preview (input + mermaid).|

The `input.*` should be minimal and focused on the one behavior the
fixture is meant to exercise. Define the source bindings the snippet
relies on (e.g. `const arr = [1, 2, 3];`) so the fixture is
self-contained, and follow the surrounding fixtures' style — arrow
parameters are written parenthesized (`(v) => …`) everywhere in the
tree.

## Adding a fixture: the procedure

1. Create `integration/fixtures/<category>/<name>/input.ts` (or another
   supported extension). Pick a `<name>` that describes the pattern and
   is distinct from its siblings — check the neighbors first so the new
   name does not duplicate an existing behavior.
2. (Optional) Inspect the analysis before blessing it, to confirm the
   fixture captures what you intended:

   ```sh
   mise run emit:ir -- integration/fixtures/<category>/<name>/input.ts
   # any of: -f ir | json | mermaid | markdown | stats
   # or pipe a snippet:  mise run emit:ir -- --stdin --stdin-lang ts < snippet.ts
   ```

3. Generate the baselines:

   ```sh
   mise run regen:fixtures
   ```

   This walks the whole fixture tree, builds the `uns` release binary
   first (the task depends on `build`), and writes the five `expected.*`
   / `preview.md` files for every fixture. **Afterward, `git status`
   should show only your new directory** — if any unrelated baseline
   changed, that is a real diff to investigate, not noise to commit.
4. Run the project verification command (see the per-implementation
   `CLAUDE.md`). The fixture is now exercised automatically — see below.

## No registration step

The fixture is discovered automatically. The CLI parity harness
(`crates/unsnarl/tests/parity.rs`) walks `integration/fixtures/` and
turns every directory that carries an `input.*` plus an `expected.*`
sibling into one test case per emitter format, comparing the binary's
output byte-for-byte against the baseline. There is no list of fixture
names to update anywhere; creating the directory and regenerating is
sufficient. To run just this harness:

```sh
mise run parity
```

## Variants (pruned / depth / highlight / plugin)

A fixture can also assert the output under non-default CLI flags. These
**variant** baselines live in sibling subdirectories named
`<kind>-<slug>` (e.g. `depth-1`, `pruned-a-c1`) and are declared in a
manifest that mirrors the fixture's path under a separate root:

```text
crates/unsnarl/tests/fixture-variants/<same-relative-path>/variants.json
```

For example, the manifest for `integration/fixtures/callback/nested-map`
lives at
`crates/unsnarl/tests/fixture-variants/callback/nested-map/variants.json`.

Each entry has a `kind` (default `pruned`), a `slug` (which forms the
directory name `<kind>-<slug>`), and the fields that kind needs:

```jsonc
{
  "variants": [
    { "slug": "a-c1", "roots": "a", "descendants": 1, "ancestors": 1 }, // pruned: -r a -A 1 -B 1
    { "kind": "depth", "slug": "1", "depths": { "uniform": 1 } }        // depth:  --depth 1
  ]
}
```

Recognized kinds and their flags: `pruned` (`-r`/`-A`/`-B`),
`depth` (`--depth`, or `--depth-function`/`--depth-block`),
`pruned-depth`, `highlight` (`-H`), and `pruned-highlight`. Declaring a
variant in the manifest is enough — `regen:fixtures` bootstraps the
`<kind>-<slug>/` directory and its baselines on first run. Variant
directories carry the four narrowing baselines (`expected.json`,
`expected.mermaid`, `preview.md`, `expected.stats`) but **not**
`expected.ir.json`: pruning, depth, and highlight only narrow the
downstream graph, so the parent IR is identical and would be redundant.

Plugin variants work by auto-discovery instead of the manifest: a
sibling `plugin-<slug>/` directory is regenerated with
`uns --plugin <slug>`, and because a plugin can reshape the IR itself,
plugin variants *do* carry `expected.ir.json`.

## Why baselines are generated, never edited

`regen:fixtures` is the single source of truth for the `expected.*`
files: it runs the real `uns` binary through every emitter, so the
baselines are exactly what the tool produces today. The parity harness
then compares the tool's output to those files byte-for-byte. Editing a
baseline by hand would either be immediately overwritten by the next
regen or — worse — encode an output the tool never produces, turning the
snapshot into a lie. The workflow is therefore always: change `input.*`
(or the implementation), regen, and review the resulting diff. A diff
you did not expect is the signal; the baseline itself is never the
place to make a change.
