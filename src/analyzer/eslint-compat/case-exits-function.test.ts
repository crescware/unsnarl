import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
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
      consequent: [{ type: AST_TYPE.ReturnStatement }],
      expected: true,
    },
    {
      name: "ends in ThrowStatement -> true",
      consequent: [{ type: AST_TYPE.ThrowStatement }],
      expected: true,
    },
    {
      name: "ends in BreakStatement -> false (control exit but not fn exit)",
      consequent: [{ type: AST_TYPE.BreakStatement }],
      expected: false,
    },
    {
      name: "ends in ContinueStatement -> false",
      consequent: [{ type: AST_TYPE.ContinueStatement }],
      expected: false,
    },
    {
      name: "ends in ExpressionStatement -> false",
      consequent: [{ type: AST_TYPE.ExpressionStatement }],
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
