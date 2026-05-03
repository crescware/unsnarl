# unsnarl

A single-file ECMAScript / TypeScript scope and reference analyzer that
emits a deterministic IR (JSON) and a Mermaid flowchart.

```sh
npm i -g unsnarl
uns path/to/your-file.ts
```

unsnarl parses a single source file with [oxc-parser], builds a
[scope-manager]-compatible scope tree (`Scope` / `Variable` /
`Reference` / `Definition`), classifies each identifier reference as
read / write / call, detects unused declarations, and serializes the
result.

It does **not** cross file boundaries. TypeScript-only constructs
(`interface`, `type`, `enum`, `namespace`, type annotations) and class
member declarations are intentionally out of scope.

[oxc-parser]: https://www.npmjs.com/package/oxc-parser
[scope-manager]: https://eslint.org/docs/latest/extend/scope-manager-interface

## CLI

```sh
uns <file>                                              # JSON IR to stdout
uns -f mermaid <file>                                   # Mermaid flowchart
uns -f markdown -r value -A 1 -o ./out file.ts          # write to ./out/value-a1.md
uns -f json --no-pretty-json <file>                     # compact JSON for piping
cat foo.ts | uns --stdin --stdin-lang ts
```

Exit codes: `0` success, `1` parse / runtime error, `2` argument error.

### Options

| Short | Long                     | Description                                                   |
| ----- | ------------------------ | ------------------------------------------------------------- |
| `-f`  | `--format <id>`          | Emitter: `ir` default, `json`, `mermaid`, `markdown`, `stats` |
|       | `--no-pretty-json`       | Disable pretty-printed JSON output                            |
|       | `--mermaid-renderer <r>` | Mermaid layout engine: `elk` default, `dagre`                 |
|       | `--stdin`                | Read source from stdin                                        |
|       | `--stdin-lang <lang>`    | Language for stdin: `ts` default, `tsx`, `js`, `jsx`          |
| `-r`  | `--roots <queries>`      | Comma-separated root queries (repeatable) — see Pruning       |
| `-A`  | `--descendants <N>`      | Descendants generations — see Pruning                         |
| `-B`  | `--ancestors <N>`        | Ancestors generations — see Pruning                           |
| `-C`  | `--context <N>`          | `-A` and `-B` shorthand — see Pruning                         |
| `-o`  | `--out-dir <dir>`        | Write to directory — see Writing to a directory               |
| `-v`  | `--version`              | Show version                                                  |
| `-h`  | `--help`                 | Show help                                                     |

### Mermaid renderer

The `mermaid` and `markdown` emitters use `elk` for layout by default.
Pass `--mermaid-renderer dagre` to fall back to dagre — required in
environments that can't register the elk loader (e.g. GitHub's markdown
preview).

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
applies to the visual-graph emitters (`json`, `mermaid`, `markdown`) only;
`ir` output is always emitted in full. If a query matches nothing, a warning
is written to stderr but the command still exits with `0`.

### Writing to a directory

Pass `-o <dir>` to write the output to a file inside `<dir>` instead of
stdout. The filename is derived from the `-r` queries and the radius flags
so that successive runs don't clobber each other:

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

## Setup

```sh
mise install
corepack enable
pnpm install
```

## Scripts

| Command            | Description                              |
| ------------------ | ---------------------------------------- |
| `pnpm check`       | Run all checks (types, lint, knip, test) |
| `pnpm check:types` | Type check                               |
| `pnpm check:lint`  | Lint and format check                    |
| `pnpm check:knip`  | Unused files/exports check               |
| `pnpm test`        | Run tests                                |
| `pnpm format`      | Fix lint and format                      |
| `pnpm build`       | Compile to `dist/` via tsgo              |

After `pnpm build` the CLI is at `dist/index.js`. Use `pnpm pack`
to produce a distributable tarball.

## Stack

- **Runtime**: Node.js 24 (via [mise](https://mise.jdx.dev/))
- **Package manager**: pnpm (via corepack)
- **Language**: TypeScript with [`@typescript/native-preview`](https://www.npmjs.com/package/@typescript/native-preview)
- **Parser**: [oxc-parser](https://oxc.rs/docs/guide/usage/parser)
- **Test**: [Vitest](https://vitest.dev/)
- **Lint**: [oxlint](https://oxc.rs/docs/guide/usage/linter)
- **Format**: [oxfmt](https://github.com/nicolo-ribaudo/oxfmt)
- **Unused code**: [Knip](https://knip.dev/)
