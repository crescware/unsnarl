import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { outermostBranchUnder } from "./outermost-branch-under.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

const root = { ...baseScope(), id: asScopeId("root") };
const outer = {
  ...baseScope(),
  id: asScopeId("outer"),
  upper: asScopeId("root"),
};
const ifBranch = {
  ...baseScope(),
  id: asScopeId("if"),
  upper: asScopeId("outer"),
  blockContext: baseBlockContext(),
};
const innerBranch = {
  ...baseScope(),
  id: asScopeId("inner"),
  upper: asScopeId("if"),
  blockContext: { ...baseBlockContext(), parentSpanOffset: 99 },
};
const map = new Map<string, SerializedScope>(
  [root, outer, ifBranch, innerBranch].map((v) => [v.id, v]),
);

describe("outermostBranchUnder", () => {
  test("returns null when scopeId equals branchId", () => {
    expect(outermostBranchUnder("outer", "outer", map)).toEqual(null);
  });

  test("returns the immediate branch child when scopeId is that branch", () => {
    expect(outermostBranchUnder("outer", "if", map)).toEqual("if");
  });

  test("walks up through nested branches and returns the outermost one under branchId", () => {
    expect(outermostBranchUnder("outer", "inner", map)).toEqual("if");
  });

  test("returns null when scopeId is not under branchId", () => {
    expect(outermostBranchUnder("if", "outer", map)).toEqual(null);
  });

  test("returns null when traversal hits the top without seeing branchId", () => {
    expect(outermostBranchUnder("missing", "inner", map)).toEqual(null);
  });
});
