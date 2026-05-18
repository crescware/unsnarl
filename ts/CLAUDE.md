# Development Rules (TypeScript)

## Verification Command

Upon completing any work, the following command must be executed without exception:

```bash
pnpm format && pnpm check
```

## Prohibited Actions

- The linters, checkers, and test runners are provided as scripts in
  `package.json`.
- Trust the provided `format` and `check` commands as the source of truth for
  validating work.

## Boundary Invariants

The source tree is split between an eslint-scope compatibility region
and an unsnarl-specific region. When editing, do not break the
following invariants. The reasoning behind them lives in
`docs/architecture.md`; this section only states what must hold.

- `src/boundary/eslint-scope/` must not import from
  `src/ir/annotations/` or from any unsnarl-only producer under
  `src/analyzer/` (e.g. `predicate.ts`, `return-container.ts`,
  `jsx-element-span.ts`, `block-context-of.ts`, `owner/`,
  `annotations-impl.ts`, `compute-nesting-depths.ts`, `case-*.ts`,
  `is-unused.ts`). The compat region must remain reachable on its
  own.
- The structure of the IR types (`Scope`, `Variable`, `Reference`,
  `Definition`) must stay assignable to the contract types in
  `src/boundary/eslint-scope/contract/`. Do not add unsnarl-specific
  fields or methods to those types. Attach unsnarl data through
  `src/ir/annotations/` and `src/analyzer/annotations-impl.ts`
  instead.
- Do not remove a field from a `compat-*.ts` contract type because
  eslint-scope still exposes that field (unless eslint-scope itself
  has marked it deprecated). Removing live contract surface is a
  parity-breaking change.
- The shared vocabulary files `scope-type.ts`, `definition-type.ts`,
  `diagnostic-kind.ts` live in `src/analyzer/` and are imported by
  both regions. Do not move them into the boundary directory.

For background — the responsibilities of each region, the judgment
axis for "where does this change belong", and the role of
`contract-assertion.ts` and the knip entry — see
`docs/architecture.md`.
