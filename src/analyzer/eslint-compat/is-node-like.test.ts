import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import { isNodeLike } from "./is-node-like.js";

describe("isNodeLike", () => {
  test.each([
    {
      name: "object with string type -> true",
      value: { type: AST_TYPE.Identifier },
      expected: true,
    },
    {
      name: "object with extra fields -> true",
      value: { type: "Foo", x: 1 },
      expected: true,
    },
    { name: "null -> false", value: null, expected: false },
    { name: "undefined -> false", value: undefined, expected: false },
    { name: "string -> false", value: "Identifier", expected: false },
    { name: "number -> false", value: 42, expected: false },
    { name: "array -> false", value: [], expected: false },
    {
      name: "object without type -> false",
      value: { name: "x" },
      expected: false,
    },
    {
      name: "object with non-string type -> false",
      value: { type: 5 },
      expected: false,
    },
  ])("$name", ({ value, expected }) => {
    expect(isNodeLike(value)).toBe(expected);
  });
});
