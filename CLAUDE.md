# Development Rules

## Verification Command

Upon completing any work, the following command must be executed without exception:

```bash
pnpm format && pnpm check
```

## Prohibited Actions

- All linters, checkers, and test runners required for this project are already provided as scripts in `package.json`. Do not reach for tools outside of these scripts for those purposes. General-purpose shell utilities such as `grep`, `find`, and `cat` remain free to use.
- The script collection in `package.json` reflects deliberate design intent; for tasks covered by those scripts, use only the scripts that are already defined rather than assembling your own equivalents.
- Do not invent your own verification methods. Trust the provided `format` and `check` commands as the source of truth for validating work.
