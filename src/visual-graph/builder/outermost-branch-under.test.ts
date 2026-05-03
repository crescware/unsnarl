import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { outermostBranchUnder } from "./outermost-branch-under.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

const root = { ...baseScope(), id: "root" };
const outer = { ...baseScope(), id: "outer", upper: "root" };
const ifBranch = {
  ...baseScope(),
  id: "if",
  upper: "outer",
  blockContext: baseBlockContext(),
};
const innerBranch = {
  ...baseScope(),
  id: "inner",
  upper: "if",
  blockContext: { ...baseBlockContext(), parentSpanOffset: 99 },
};
const map = new Map<string, SerializedScope>(
  [root, outer, ifBranch, innerBranch].map((s) => [s.id, s]),
);

describe("outermostBranchUnder", () => {
  test("returns null when scopeId equals branchId", () => {
    expect(outermostBranchUnder("outer", "outer", map)).toBeNull();
  });

  test("returns the immediate branch child when scopeId is that branch", () => {
    expect(outermostBranchUnder("outer", "if", map)).toBe("if");
  });

  test("walks up through nested branches and returns the outermost one under branchId", () => {
    expect(outermostBranchUnder("outer", "inner", map)).toBe("if");
  });

  test("returns null when scopeId is not under branchId", () => {
    expect(outermostBranchUnder("if", "outer", map)).toBeNull();
  });

  test("returns null when traversal hits the top without seeing branchId", () => {
    expect(outermostBranchUnder("missing", "inner", map)).toBeNull();
  });
});
