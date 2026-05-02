import { describe, expect, test } from "vitest";

import { detectLanguage } from "./detect-language.js";

describe("detectLanguage", () => {
  test.each([
    { name: ".tsx → tsx", path: "Component.tsx", expected: "tsx" },
    { name: ".jsx → jsx", path: "Component.jsx", expected: "jsx" },
    { name: ".js → js", path: "foo.js", expected: "js" },
    { name: ".mjs → js", path: "foo.mjs", expected: "js" },
    { name: ".cjs → js", path: "foo.cjs", expected: "js" },
    { name: ".ejs → js", path: "foo.ejs", expected: "js" },
    { name: ".ts → ts", path: "foo.ts", expected: "ts" },
    { name: ".mts → ts", path: "foo.mts", expected: "ts" },
    { name: ".cts → ts", path: "foo.cts", expected: "ts" },
    {
      name: "unknown extension → ts (default branch)",
      path: "Makefile",
      expected: "ts",
    },
    {
      name: "nested paths still inspect the trailing suffix",
      path: "src/deep/Component.tsx",
      expected: "tsx",
    },
  ] as const)("$name", ({ path, expected }) => {
    expect(detectLanguage(path)).toBe(expected);
  });
});
