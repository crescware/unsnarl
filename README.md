# unsnarl

A single-file ECMAScript / TypeScript scope and reference analyzer that
emits a deterministic IR (JSON) and a Mermaid flowchart.

`unsnarl` parses a single source file with [oxc-parser], builds a
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
unsnarl <file>                      # JSON IR to stdout
unsnarl --format mermaid <file>     # Mermaid flowchart
cat foo.ts | unsnarl --stdin --lang ts
unsnarl --list-formats
```

Exit codes: `0` success, `1` parse / runtime error, `2` argument error.

### Pruning the visual graph

Large files generate dense graphs that can be hard to read. Pass one or more
root queries with `-r` / `--roots` to keep only the neighborhood of the
specified nodes. Combine with `-A` / `-B` / `-C` to control how far the
neighborhood expands.

```sh
unsnarl --format mermaid -r 42:render -C 3 file.tsx       # 3 generations both ways
unsnarl --format mermaid -r 9-13 -A 2 -B 0 file.ts        # range, descendants only
unsnarl --format mermaid -r 10:foo,42 -r 99 file.ts       # multiple roots
```

Each query token is one of:

| form     | meaning                                    |
| -------- | ------------------------------------------ |
| `n`      | every node on line `n`                     |
| `n:id`   | node named `id` on line `n`                |
| `n-m`    | every node in line range `[n, m]`          |
| `n-m:id` | node named `id` within line range `[n, m]` |
| `id`     | every node named `id`, regardless of scope |

Generation flags (and their long aliases):

- `-A N` / `--descendants N` – keep `N` generations of descendants
- `-B N` / `--ancestors N` – keep `N` generations of ancestors
- `-C N` / `--context N` – shorthand for `-A N -B N`

When `-r` is given but no generation flag is, the default is `-C 10`. Pruning
applies to the visual-graph emitters (`json`, `mermaid`, `markdown`) only;
`ir` output is always emitted in full. If a query matches nothing, a warning
is written to stderr but the command still exits with `0`.

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

After `pnpm build` the CLI is at `dist/cli/main.js`. Use `pnpm pack`
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
