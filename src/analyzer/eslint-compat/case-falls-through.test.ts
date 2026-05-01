import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
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
      consequent: [{ type: AST_TYPE.BreakStatement }],
      expected: false,
    },
    {
      name: "ends in ReturnStatement -> false",
      consequent: [{ type: AST_TYPE.ReturnStatement }],
      expected: false,
    },
    {
      name: "ends in ThrowStatement -> false",
      consequent: [{ type: AST_TYPE.ThrowStatement }],
      expected: false,
    },
    {
      name: "ends in ContinueStatement -> false",
      consequent: [{ type: AST_TYPE.ContinueStatement }],
      expected: false,
    },
    {
      name: "ends in ExpressionStatement -> true (no exit)",
      consequent: [{ type: AST_TYPE.ExpressionStatement }],
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
        {
          type: AST_TYPE.BlockStatement,
          body: [{ type: AST_TYPE.ReturnStatement }],
        },
      ],
      expected: false,
    },
  ])("$name", ({ consequent, expected }) => {
    expect(caseFallsThrough(consequent)).toBe(expected);
  });
});
