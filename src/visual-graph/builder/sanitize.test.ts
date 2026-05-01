import { describe, expect, test } from "vitest";

import { sanitize } from "./sanitize.js";

describe("sanitize", () => {
  test.each([
    {
      name: "alphanumerics and underscores pass through",
      input: "abc_123_XYZ",
      expected: "abc_123_XYZ",
    },
    { name: "dot becomes underscore", input: "a.b", expected: "a_b" },
    { name: "slash becomes underscore", input: "a/b", expected: "a_b" },
    { name: "hyphen becomes underscore", input: "a-b", expected: "a_b" },
    { name: "colon becomes underscore", input: "a:b", expected: "a_b" },
    { name: "space becomes underscore", input: "a b", expected: "a_b" },
    {
      name: "consecutive specials produce consecutive underscores",
      input: "a..b",
      expected: "a__b",
    },
    {
      name: "all-special becomes all underscores",
      input: "!@#$",
      expected: "____",
    },
    {
      name: "non-ascii letter becomes underscore",
      input: "あa",
      expected: "_a",
    },
    { name: "empty string returns empty", input: "", expected: "" },
  ])("$name", ({ input, expected }) => {
    expect(sanitize(input)).toBe(expected);
  });
});
