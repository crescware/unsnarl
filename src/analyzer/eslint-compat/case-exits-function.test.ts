import { describe, expect, test } from "vitest";

import { caseExitsFunction } from "./case-exits-function.js";

describe("caseExitsFunction", () => {
  test.each([
    {
      name: "empty consequent -> false (no exit)",
      consequent: [],
      expected: false,
    },
    {
      name: "ends in ReturnStatement -> true",
      consequent: [{ type: "ReturnStatement" }],
      expected: true,
    },
    {
      name: "ends in ThrowStatement -> true",
      consequent: [{ type: "ThrowStatement" }],
      expected: true,
    },
    {
      name: "ends in BreakStatement -> false (control exit but not fn exit)",
      consequent: [{ type: "BreakStatement" }],
      expected: false,
    },
    {
      name: "ends in ContinueStatement -> false",
      consequent: [{ type: "ContinueStatement" }],
      expected: false,
    },
    {
      name: "ends in ExpressionStatement -> false",
      consequent: [{ type: "ExpressionStatement" }],
      expected: false,
    },
    {
      name: "last item is not NodeLike -> false",
      consequent: [42],
      expected: false,
    },
  ])("$name", ({ consequent, expected }) => {
    expect(caseExitsFunction(consequent)).toBe(expected);
  });
});
