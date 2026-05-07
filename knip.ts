import type { KnipConfig } from "knip";

const config = {
  entry: [
    "src/index.ts",
    "src/main.ts",
    // Type-only assertion file: pins eslint-scope contract compatibility at
    // compile time. Has no runtime importer by design, so list it as an
    // entry so knip walks its imports (the compat-* contract types) and
    // does not report them as unused.
    "src/eslint-compat/contract/contract-assertion.ts",
    "integration/**/*.test.ts",
    "parity/**/*.test.ts",
  ],
  project: [
    "src/**/*.{ts,tsx}",
    "integration/**/*.{ts,tsx}",
    "!integration/fixtures/**",
    "parity/**/*.{ts,tsx}",
  ],
  ignoreDependencies: [
    // type-fest is pulled in solely because `mermaid`'s bundled `.d.ts` files
    // reference types from it. No `project` source imports type-fest directly,
    // so knip's source-graph scan reports it as "Unused devDependencies".
    "type-fest",
    // rimraf has no `import` site under `project`; it is consumed only as a
    // CLI in the `build` npm script. Without this entry knip reports it as
    // "Unused devDependencies" because the source-graph scan finds no use.
    "rimraf",
  ],
  ignoreBinaries: [
    // rimraf is invoked from the `build` npm script (`rimraf dist && tsgo ...`).
    // knip's binary resolver does not match the script-level invocation back
    // to the devDependency, so it reports rimraf as an "Unlisted binary".
    "rimraf",
  ],
} as const satisfies KnipConfig;

export default config;
