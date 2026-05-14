import { describe, expect, test } from "vitest";

import { asFilledString } from "../../util/filled-string.js";
import { sanitize } from "./sanitize.js";

describe("sanitize", () => {
  test.each([
    {
      name: asFilledString("alphanumerics and underscores pass through"),
      input: "abc_123_XYZ",
      expected: "abc_123_XYZ",
    },
    {
      name: asFilledString("dot becomes underscore"),
      input: "a.b",
      expected: "a_b",
    },
    {
      name: asFilledString("slash becomes underscore"),
      input: "a/b",
      expected: "a_b",
    },
    {
      name: asFilledString("hyphen becomes underscore"),
      input: "a-b",
      expected: "a_b",
    },
    {
      name: asFilledString("colon becomes underscore"),
      input: "a:b",
      expected: "a_b",
    },
    {
      name: asFilledString("space becomes underscore"),
      input: "a b",
      expected: "a_b",
    },
    {
      name: asFilledString(
        "consecutive specials produce consecutive underscores",
      ),
      input: "a..b",
      expected: "a__b",
    },
    {
      name: asFilledString("all-special becomes all underscores"),
      input: "!@#$",
      expected: "____",
    },
    {
      name: asFilledString("non-ascii letter becomes underscore"),
      input: "あa",
      expected: "_a",
    },
    {
      name: asFilledString("empty string returns empty"),
      input: "",
      expected: "",
    },
  ])("$name", ({ input, expected }) => {
    expect(sanitize(input)).toEqual(expected);
  });
});
