import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import type { BlockContext } from "../../ir/model.js";
import { isBranchScope } from "./is-branch-scope.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

describe("isBranchScope", () => {
  test.each<{ name: string; ctx: BlockContext | null; expected: boolean }>([
    {
      name: "if consequent block scope -> true",
      ctx: makeBlockContext(AST_TYPE.IfStatement, "consequent", 0),
      expected: true,
    },
    {
      name: "if alternate block scope -> true",
      ctx: makeBlockContext(AST_TYPE.IfStatement, "alternate", 0),
      expected: true,
    },
    {
      name: "switch case scope -> true",
      ctx: makeBlockContext(AST_TYPE.SwitchStatement, "cases", 0),
      expected: true,
    },
    {
      name: "try block scope -> false (try is control, not branch)",
      ctx: makeBlockContext(AST_TYPE.TryStatement, "block", 0),
      expected: false,
    },
    {
      name: "no blockContext -> false",
      ctx: null,
      expected: false,
    },
  ])("$name", ({ ctx, expected }) => {
    const scope = makeScope({ id: "s", blockContext: ctx });
    const map = new Map([[scope.id, scope]]);
    expect(isBranchScope("s", map)).toBe(expected);
  });

  test("scope id missing from the map -> false", () => {
    expect(isBranchScope("missing", new Map())).toBe(false);
  });
});
