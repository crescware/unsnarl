import { describe, expect, test } from "vitest";

import { intermediateKey } from "./intermediate-key.js";

describe("intermediateKey", () => {
  test.each([
    { source: "./mod.js", originalName: "foo", expected: "./mod.js::foo" },
    {
      source: "a/b/c",
      originalName: "Some Name",
      expected: "a/b/c::Some Name",
    },
    { source: "", originalName: "", expected: "::" },
    { source: "react", originalName: "useState", expected: "react::useState" },
  ])(
    "intermediateKey($source, $originalName) = $expected",
    ({ source, originalName, expected }) => {
      expect(intermediateKey(source, originalName)).toBe(expected);
    },
  );
});
