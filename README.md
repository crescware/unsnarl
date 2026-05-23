# unsnarl

A single-file ECMAScript / TypeScript scope and reference analyzer that
emits a Mermaid flowchart and a deterministic IR (JSON).

```sh
npm i -g unsnarl
uns path/to/your-file.ts
```

The npm package ships a Node.js shim that dispatches to a prebuilt
platform binary via `optionalDependencies`. Only `darwin-arm64` is
published as of this release; on any other platform the shim prints a
"no prebuilt binary available" message. Build from source on
unsupported platforms (or to track `main`):

```sh
cargo build --release -p unsnarl
./target/release/uns path/to/your-file.ts
```

unsnarl parses a single source file with [oxc][oxc] (`oxc_parser`),
builds a [scope-manager][scope-manager]-compatible scope tree
(`Scope` / `Variable` / `Reference` / `Definition`), classifies each
identifier reference as read / write / call, detects unused
declarations, and serializes the result.

It does **not** cross file boundaries. TypeScript-only constructs
(`interface`, `type`, `enum`, `namespace`, type annotations) and class
member declarations are intentionally out of scope.

[oxc]: https://oxc.rs/
[scope-manager]: https://eslint.org/docs/latest/extend/scope-manager-interface

## CLI

```sh
uns <file>                                              # Mermaid flowchart to stdout
uns -f ir <file>                                        # JSON IR
uns -f markdown -r value -A 1 -o ./out file.ts          # write to ./out/value-a1.md
uns -f json --no-pretty-json <file>                     # compact JSON for piping
cat foo.ts | uns --stdin --stdin-lang ts
```

Exit codes: `0` success, `1` parse / runtime error, `2` argument error.

### Options

| Short | Long                     | Description                                                   |
| ----- | ------------------------ | ------------------------------------------------------------- |
| `-f`  | `--format <id>`          | Emitter: `mermaid` default, `ir`, `json`, `markdown`, `stats` |
|       | `--no-pretty-json`       | Disable pretty-printed JSON output                            |
|       | `--mermaid-renderer <r>` | Mermaid layout engine: `elk` default, `dagre`                 |
|       | `--color-theme <t>`      | Mermaid color theme: `dark` default, `light`                  |
|       | `--stdin`                | Read source from stdin                                        |
|       | `--stdin-lang <lang>`    | Language for stdin: `ts` default, `tsx`, `js`, `jsx`          |
| `-r`  | `--roots <queries>`      | Comma-separated root queries (repeatable) — see Pruning       |
| `-A`  | `--descendants <N>`      | Descendants generations — see Pruning                         |
| `-B`  | `--ancestors <N>`        | Ancestors generations — see Pruning                           |
| `-C`  | `--context <N>`          | `-A` and `-B` shorthand — see Pruning                         |
| `-H`  | `--highlight [queries]`  | Highlight matching nodes + adjacent edges — see Highlighting  |
|       | `--depth <N>`            | Set both `--depth-function` and `--depth-block` — see Depth   |
|       | `--depth-function <N>`   | Max function-scope nesting depth — see Depth                  |
|       | `--depth-block <N>`      | Max block-scope nesting depth — see Depth                     |
| `-o`  | `--out-dir <dir>`        | Write to directory with auto-named file — see Writing output  |
|       | `--out-file <path>`      | Write to that exact file path — see Writing output            |
|       | `--plugin <names>`       | Enable bundled plugin(s) (repeatable) — see Plugins           |
|       | `--debug`                | Annotate Mermaid labels with `NODE_KIND` / `SUBGRAPH_KIND`    |
|       | `--verbose`              | Stream diagnostic and timing logs to stderr                   |
| `-v`  | `--version`              | Show version                                                  |
| `-h`  | `--help`                 | Show help                                                     |

### Mermaid renderer

The `mermaid` and `markdown` emitters use `elk` for layout by default.
Pass `--mermaid-renderer dagre` to fall back to dagre — required in
environments that can't register the elk loader (e.g. GitHub's markdown
preview).

### Color theme

The Mermaid output is colored for a dark background by default. Pass
`--color-theme light` to switch every `classDef` (boundary stub,
var node, per-depth subgraph palette covering function wrappers, and
the elk empty-placeholder workaround) to a palette tuned for light
backgrounds. The two built-in themes are `dark` and `light`; the
background cannot be auto-detected, so the choice is always explicit.

```sh
uns -f mermaid --color-theme light file.ts
```

### Pruning the visual graph

Large files generate dense graphs that can be hard to read. Pass one or more
root queries with `-r` to keep only the neighborhood of the specified nodes.
Combine with `-A` / `-B` / `-C` to control how far the neighborhood expands.

```sh
uns -f mermaid -r 42:render -C 3 file.tsx       # 3 generations both ways
uns -f mermaid -r 9-13 -A 2 -B 0 file.ts        # range, descendants only
uns -f mermaid -r 10:foo,42 -r 99 file.ts       # multiple roots
```

Each query token is one of:

| form     | meaning                                    |
| -------- | ------------------------------------------ |
| `n`      | every node on line `n`                     |
| `n:id`   | node named `id` on line `n`                |
| `n-m`    | every node in line range `[n, m]`          |
| `n-m:id` | node named `id` within line range `[n, m]` |
| `id`     | every node named `id`, regardless of scope |

When `-r` is given but no generation flag is, the default is `-C 10`. Pruning
applies to every visual-graph emitter (`json`, `mermaid`, `markdown`,
`stats`); `ir` output is always emitted in full. If a query matches nothing,
a warning is written to stderr but the command still exits with `0`.

### Limiting nesting depth

Function and block scopes nest visually for every layer of source-level
scope, and past some depth the diagram becomes harder to read than the code
it was meant to clarify. Pass `--depth N` to collapse every scope deeper
than `N` into a single `((...))` placeholder so the outer shape stays
readable:

```sh
uns --depth 2 file.ts            # collapse anything past depth 2
uns --depth-function 1 file.ts   # function scopes only
uns --depth-block 0 file.ts      # block scopes only, very aggressive
```

Splits per scope kind:

- `--depth-function N` — function / arrow / method bodies.
- `--depth-block N` — `if` / `for` / `while` / `switch` /
  `try` / `catch` / `finally` / plain `{ ... }` blocks.
- `--depth N` is shorthand: it seeds both of the above, and either split
  flag overrides it for its kind.

Default for every kind is `10`. The placeholder node uses the same
dashed-circle treatment as the pruning boundary stubs, so "more graph
beyond this point" reads the same whether the cause is `-r/-A/-B/-C`
distance or `--depth` collapse. Depth applies to every visual-graph
emitter (`json`, `mermaid`, `markdown`, `stats`); `ir` output is always
emitted in full.

### Highlighting the visual graph

Once the graph is pruned (or even without `-r`), `-H` / `--highlight` paints
the matched nodes and every edge with at least one endpoint in that match
set yellow. Highlighting is purely visual and only takes effect on the
`mermaid` and `markdown` emitters.

```sh
uns -r b -C 1 -H file.ts                 # highlight whatever -r selected
uns -r b -C 1 -H a file.tsx              # highlight 'a' only (b stays uncolored)
uns -r 42:render -C 2 --highlight render file.tsx
```

Two modes:

- `-H` / `--highlight` with no value: the highlight set follows `-r/--roots`,
  i.e. every node that drove the pruning is painted yellow.
- `-H <queries>` / `--highlight <queries>`: the highlight uses its own
  query list (same grammar as `-r`); the `-r` matches stay uncolored.

Because `-H` accepts an optional value, write `--highlight=foo` (or place
the file before the flag) when the next argument would otherwise be parsed
as the value, e.g. `uns -H foo.ts` interprets `foo.ts` as the highlight
query, not as the input file.

### Writing output

There are two ways to send the result somewhere other than stdout:
`-o/--out-dir <dir>` writes to an auto-named file inside a directory, and
`--out-file <path>` writes verbatim to the given path. The two flags are
mutually exclusive — passing both exits with code `2`.

#### Auto-named: `-o <dir>`

Pass `-o <dir>` to write the output to a file inside `<dir>`. The filename
is derived from the `-r` queries and the radius flags so that successive
runs don't clobber each other:

```sh
uns -f markdown -r value -A 1 -o ./out file.ts
# -> ./out/value-a1.md
uns -f markdown -r 10-12 -C 2 -o ./out file.ts
# -> ./out/l10-12-c2.md
uns -f markdown -r 42:render -A 1 -o ./out file.ts
# -> ./out/l42-render-a1.md
```

Naming rules:

| Query token / flag       | Filename fragment           |
| ------------------------ | --------------------------- |
| `id`                     | `id`                        |
| `n`                      | `l<n>`                      |
| `n:id`                   | `l<n>-<id>`                 |
| `n-m`                    | `l<n>-<m>`                  |
| `n-m:id`                 | `l<n>-<m>-<id>`             |
| multiple `-r` queries    | joined with `+`             |
| `-A N` / `-B N` / `-C N` | `-a<N>` / `-b<N>` / `-c<N>` |

Radius fragments are appended in `a` → `b` → `c` order. When no radius flag
is given, no suffix is added. When both `-A` and `-B` are given explicitly,
`-C` is dropped from the filename because it has no remaining effect on
the run. When `-r` is omitted entirely, the input filename (without
extension) is used as the basename. Extensions per format:
`ir` / `json` → `.json`, `mermaid` → `.mmd`, `markdown` → `.md`,
`stats` → `.tsv`. The directory is created if missing, and existing files
are overwritten.

`-o` always treats its argument as a directory, even if it looks like a
filename. `uns -o graph.mmd file.ts` creates a directory `graph.mmd/` and
writes `graph.mmd/file.mmd` inside it. When the `-o` argument's basename
contains a dot, a notice is written to stderr suggesting `--out-file`.

#### Exact path: `--out-file <path>`

Pass `--out-file <path>` to write to that exact path with no auto-naming.
This is the right choice when you want to pin both the filename and the
extension yourself:

```sh
uns -f mermaid --out-file build/graph.mmd file.ts
# -> build/graph.mmd
cat foo.ts | uns --stdin --out-file out.json -f json
# -> out.json (works without -r because no naming is required)
```

Parent directories are created if missing, and the file is overwritten if
it exists. Unlike `-o`, `--out-file` does not require `-r/--roots` when
reading from stdin, because it does not derive a basename.

### Plugins

A plugin transforms the serialized IR between analysis and emission — after
the scope tree has been built but before any output format is rendered. Pass
`--plugin <name>` to enable one; the flag is repeatable and accepts
comma-separated values:

```sh
uns --plugin react file.tsx
uns --plugin react,other file.tsx
uns --plugin react --plugin other file.tsx
```

Names may include or omit the `unsnarl-plugin-` prefix (`react` and
`unsnarl-plugin-react` resolve to the same plugin). Only the plugins bundled
with the unsnarl build are available; passing an unknown name exits with `2`.

#### `unsnarl-plugin-react`

Elides the React hooks `useCallback` and `useMemo` so the IR reads as if the
hook wrappers were not present:

- `const x = useCallback(fn, deps)` — the variable's init is rewritten to
  point at the inner function, so `x` reads as a plain arrow assignment.
- `const x = useMemo(fn, deps)` — the variable's init stays as a call, but
  the `useMemo` callee and the dep-array references are dropped so it reads
  as an IIFE invocation of the inner function.

The `useCallback` / `useMemo` imports are removed when no other references
point at them. A hook referenced for non-call use (e.g. passed as a value)
is left in place.

## Setup

unsnarl is a Cargo workspace targeting Rust 1.95+. The Rust toolchain
(plus the Deno + Node runtimes used by helper scripts and the npm
publish flow) is pinned through [mise](https://mise.jdx.dev/):

```sh
mise install
cargo build --release -p unsnarl
```

The release binary lands at `target/release/uns`.

A TypeScript reference implementation lives under `ts/` and is kept
building so the Rust output can be compared against it byte-for-byte by
the parity benches. See `ts/CLAUDE.md` for that subtree's own
verification command.

Project-wide conventions (file naming, sibling test files, dead-code
policy, derive policy) are documented under `docs/`.

## Verification

The project-defined check that must pass before any change is committed:

```sh
cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

## Scripts

Workspace tasks are defined in `mise.toml` and invoked via
`mise run <name>` (or `mise run <name> -- <args>` to forward arguments):

| Task                       | Description                                                                                                                |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `build`                    | Release-build the `uns` CLI binary to `target/release/uns`.                                                                |
| `parity`                   | Run the parity harness against `ts/integration/fixtures/**/expected.ir.json`.                                              |
| `emit:ir`                  | Convenience runner for `uns -f ir` on the forwarded arguments.                                                             |
| `bench:ir-parity`          | Run `-f ir` against every `ts/src/**/*.ts(x)` and compare Rust vs TS byte-for-byte; per-file timings under `target/`.      |
| `bench:json-parity`        | Same sweep for `-f json` (visual-graph JSON).                                                                              |
| `bench:mermaid-parity`     | Same sweep for `-f mermaid` (CLI defaults: elk + dark theme).                                                              |
| `bench:markdown-parity`    | Same sweep for `-f markdown` (same mermaid render wrapped in markdown).                                                    |
| `bench:stats-parity`       | Same sweep for `-f stats`.                                                                                                 |
| `bench:variant-parity`     | Single-shot variant sweep: `mise run bench:variant-parity -- <format> <variant>`.                                          |
| `bench:variant-parity-all` | Full 5-format × 6-variant matrix (30 sweeps).                                                                              |
| `cov`                      | Build with `-Cinstrument-coverage`, run `cargo test --workspace`, and emit a per-file llvm-cov report under `target/coverage/`. |
| `check:no-allow-dead-code` | Scan the source tree for forbidden `#[allow(dead_code)]` / `#[expect(dead_code)]` attributes (see `docs/dead-code-policy.md`). |
| `bump`                     | Align every version field (Cargo + npm) AND both `publishConfig.tag` entries to the argument: `mise run bump -- 0.3.1`. The tag is derived from the semver pre-release id (no prerelease → `latest`, `-rc.*` → `rc`, anything else → `beta`). |
| `npm:build-darwin-arm64`   | Cross-compile the `aarch64-apple-darwin` release and stage it into `npm/unsnarl-darwin-arm64/bin/uns`. Triggered by the npm `prepack` hook. |
| `npm:publish`              | Publish `unsnarl-darwin-arm64` then `unsnarl` to npm. Each package's dist-tag is read from its own `publishConfig.tag` and passed via `--tag`. |
| `npm:publish:dry-run`      | Shortcut for `npm:publish -- --dry-run`.                                                                                   |

## Stack

- **Language**: Rust (edition 2021, MSRV 1.95).
- **Workspace**: Cargo workspace under `crates/`; the CLI binary `uns` is produced by `crates/unsnarl`.
- **Parser / AST**: [oxc][oxc] (`oxc_parser` / `oxc_ast` / `oxc_ast_visit`).
- **Task runner**: [mise](https://mise.jdx.dev/) pins the toolchain and exposes the workspace tasks above.
- **Helper scripts**: Deno TypeScript modules under `scripts/`.
- **Reference implementation**: TypeScript port under `ts/`, kept in lockstep for IR-byte parity.
- **Lint / format / test**: `cargo clippy -D warnings`, `cargo fmt`, `cargo test --workspace`.
