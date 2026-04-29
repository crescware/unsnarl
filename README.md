# template

TypeScript project template.

## Stack

- **Runtime**: Node.js 24 (via [mise](https://mise.jdx.dev/))
- **Package manager**: pnpm (via corepack)
- **Language**: TypeScript ([native preview](https://github.com/nicolo-ribaudo/tc39-proposal-type-annotations))
- **Test**: [Vitest](https://vitest.dev/)
- **Lint**: [oxlint](https://oxc.rs/docs/guide/usage/linter)
- **Format**: [oxfmt](https://github.com/nicolo-ribaudo/oxfmt)
- **Unused code**: [Knip](https://knip.dev/)

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
