import type { KnipConfig } from "knip";

const config = {
  workspaces: {
    ".": {
      entry: [
        "src/index.ts",
        "src/main.ts",
        // Type-only assertion file: pins eslint-scope contract compatibility at
        // compile time. Has no runtime importer by design, so list it as an
        // entry so knip walks its imports (the compat-* contract types) and
        // does not report them as unused.
        "src/boundary/eslint-scope/contract/contract-assertion.ts",
        "integration/*.test.ts",
        "parity/**/*.test.ts",
      ],
      ignore: [
        // Test fixture: a module shaped to lack a `default` export, used by
        // validate-plugin-module.test.ts via `await import(...)` to exercise
        // the "no default export" branch. Dynamic import is invisible to
        // knip's static scan, so the named export reads as unused. The file
        // exists only as a runtime artifact for that test.
        "src/cli/run-cli/testing/no-default-plugin.ts",
      ],
      project: [
        "src/**/*.{ts,tsx}",
        "integration/**/*.{ts,tsx}",
        "parity/**/*.{ts,tsx}",
      ],
      ignoreDependencies: [],
    },
    "scripts/no-to-be": {
      entry: ["**/*.test.ts"],
      project: ["**/*.{ts,tsx}"],
      ignoreDependencies: [
        // @typescript/native-preview supplies the tsgo compiler and pins the
        // TypeScript language version this script is written against. There is
        // no `import` site for it, so knip's source-graph scan reports it as
        // "Unused devDependencies" without this entry.
        "@typescript/native-preview",
      ],
    },
  },
} as const satisfies KnipConfig;

export default config;
