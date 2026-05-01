import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { lastWriteOpInScopeBefore } from "./last-write-op-in-scope-before.js";
import { baseScope } from "./testing/make-scope.js";
import { baseWriteOp } from "./testing/make-write-op.js";
import type { WriteOp } from "./write-op.js";

const root = { ...baseScope(), id: "root" };
const child = { ...baseScope(), id: "child", upper: "root" };
const sibling = { ...baseScope(), id: "sibling", upper: "root" };
const grandchild = { ...baseScope(), id: "grandchild", upper: "child" };
const scopeMap = new Map<string, SerializedScope>(
  [root, child, sibling, grandchild].map((s) => [s.id, s]),
);

const ops: WriteOp[] = [
  { ...baseWriteOp(), refId: "rRoot", offset: 5, scopeId: "root" },
  { ...baseWriteOp(), refId: "rChild", offset: 10, scopeId: "child" },
  { ...baseWriteOp(), refId: "rGrand", offset: 15, scopeId: "grandchild" },
  { ...baseWriteOp(), refId: "rSib", offset: 20, scopeId: "sibling" },
  { ...baseWriteOp(), refId: "rRoot2", offset: 25, scopeId: "root" },
];
const byVar = new Map<string, WriteOp[]>([["v", ops]]);

describe("lastWriteOpInScopeBefore", () => {
  test.each<{
    name: string;
    scopeId: string;
    offset: number;
    expected: string | null;
  }>([
    {
      name: "root scope sees ops from itself and all descendants",
      scopeId: "root",
      offset: 100,
      expected: "rRoot2",
    },
    {
      name: "child scope sees its own and grandchild ops, but not root or sibling",
      scopeId: "child",
      offset: 100,
      expected: "rGrand",
    },
    {
      name: "child scope before grandchild write picks the child write",
      scopeId: "child",
      offset: 12,
      expected: "rChild",
    },
    {
      name: "sibling scope sees only its own writes",
      scopeId: "sibling",
      offset: 100,
      expected: "rSib",
    },
    {
      name: "ops at or after the offset are excluded",
      scopeId: "root",
      offset: 5,
      expected: null,
    },
    {
      name: "no candidates -> null",
      scopeId: "root",
      offset: 0,
      expected: null,
    },
  ])("$name", ({ scopeId, offset, expected }) => {
    const result = lastWriteOpInScopeBefore(
      "v",
      scopeId,
      offset,
      byVar,
      scopeMap,
    );
    expect(result?.refId ?? null).toBe(expected);
  });

  test("variable with no recorded writes returns null", () => {
    expect(
      lastWriteOpInScopeBefore("missing", "root", 100, byVar, scopeMap),
    ).toBeNull();
  });
});
