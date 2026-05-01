import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { branchScopeOf } from "./branch-scope-of.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

const outer = { ...baseScope(), id: "outer" };
const ifBranch = {
  ...baseScope(),
  id: "if",
  upper: "outer",
  blockContext: baseBlockContext(),
};
const inner = { ...baseScope(), id: "inner", upper: "if" };
const deeper = { ...baseScope(), id: "deeper", upper: "inner" };
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
