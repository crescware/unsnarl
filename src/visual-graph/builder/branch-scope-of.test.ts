import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { branchScopeOf } from "./branch-scope-of.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

const outer = { ...baseScope(), id: asScopeId("outer") };
const ifBranch = {
  ...baseScope(),
  id: asScopeId("if"),
  upper: asScopeId("outer"),
  blockContext: baseBlockContext(),
};
const inner = {
  ...baseScope(),
  id: asScopeId("inner"),
  upper: asScopeId("if"),
};
const deeper = {
  ...baseScope(),
  id: asScopeId("deeper"),
  upper: asScopeId("inner"),
};
const map = new Map<string, SerializedScope>(
  [outer, ifBranch, inner, deeper].map((v) => [v.id, v]),
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
    expect(branchScopeOf(start, map)).toEqual(expected);
  });
});
