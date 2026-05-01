import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import type { SerializedScope } from "../../ir/model.js";
import { branchScopeOf } from "./branch-scope-of.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

const outer = makeScope({ id: "outer" });
const ifBranch = makeScope({
  id: "if",
  upper: "outer",
  blockContext: makeBlockContext(AST_TYPE.IfStatement, "consequent", 0),
});
const inner = makeScope({ id: "inner", upper: "if" });
const deeper = makeScope({ id: "deeper", upper: "inner" });
const map = new Map<string, SerializedScope>(
  [outer, ifBranch, inner, deeper].map((s) => [s.id, s]),
);

describe("branchScopeOf", () => {
  test.each<{ name: string; start: string; expected: string | null }>([
    {
      name: "branch scope itself returns its own id",
      start: "if",
      expected: "if",
    },
    {
      name: "child of branch returns the branch id",
      start: "inner",
      expected: "if",
    },
    {
      name: "deeper descendant still resolves to branch",
      start: "deeper",
      expected: "if",
    },
    {
      name: "non-branch ancestor chain returns null",
      start: "outer",
      expected: null,
    },
    { name: "missing scope returns null", start: "missing", expected: null },
  ])("$name", ({ start, expected }) => {
    expect(branchScopeOf(start, map)).toBe(expected);
  });
});
