import { describe, expect, test } from "vitest";

import { caseFallsThrough } from "./case-falls-through.js";

describe("caseFallsThrough", () => {
  test.each([
    {
      name: "empty consequent -> true (empty case falls through)",
      consequent: [],
      expected: true,
    },
    {
      name: "ends in BreakStatement -> false",
      consequent: [{ type: "BreakStatement" }],
      expected: false,
    },
    {
      name: "ends in ReturnStatement -> false",
      consequent: [{ type: "ReturnStatement" }],
      expected: false,
    },
    {
      name: "ends in ThrowStatement -> false",
      consequent: [{ type: "ThrowStatement" }],
      expected: false,
    },
    {
      name: "ends in ContinueStatement -> false",
      consequent: [{ type: "ContinueStatement" }],
      expected: false,
    },
    {
      name: "ends in ExpressionStatement -> true (no exit)",
      consequent: [{ type: "ExpressionStatement" }],
      expected: true,
    },
    {
      name: "last item is not NodeLike (string) -> true",
      consequent: ["not a node"],
      expected: true,
    },
    {
      name: "ends in BlockStatement that exits -> false",
      consequent: [
        { type: "BlockStatement", body: [{ type: "ReturnStatement" }] },
      ],
      expected: false,
    },
  ])("$name", ({ consequent, expected }) => {
    expect(caseFallsThrough(consequent)).toBe(expected);
  });
});
