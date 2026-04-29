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
