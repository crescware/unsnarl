import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { asFilledString } from "../../util/filled-string.js";
import { lastWriteOpInScopeBefore } from "./last-write-op-in-scope-before.js";
import { baseScope } from "./testing/make-scope.js";
import { baseWriteOp } from "./testing/make-write-op.js";
import type { WriteOp } from "./write-op.js";

const root = { ...baseScope(), id: asScopeId("root") };
const child = {
  ...baseScope(),
  id: asScopeId("child"),
  upper: asScopeId("root"),
};
const sibling = {
  ...baseScope(),
  id: asScopeId("sibling"),
  upper: asScopeId("root"),
};
const grandchild = {
  ...baseScope(),
  id: asScopeId("grandchild"),
  upper: asScopeId("child"),
};
const scopeMap = new Map<string, SerializedScope>(
  [root, child, sibling, grandchild].map((v) => [v.id, v]),
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
      name: asFilledString(
        "root scope sees ops from itself and all descendants",
      ),
      scopeId: "root",
      offset: 100,
      expected: "rRoot2",
    },
    {
      name: asFilledString(
        "child scope sees its own and grandchild ops, but not root or sibling",
      ),
      scopeId: "child",
      offset: 100,
      expected: "rGrand",
    },
    {
      name: asFilledString(
        "child scope before grandchild write picks the child write",
      ),
      scopeId: "child",
      offset: 12,
      expected: "rChild",
    },
    {
      name: asFilledString("sibling scope sees only its own writes"),
      scopeId: "sibling",
      offset: 100,
      expected: "rSib",
    },
    {
      name: asFilledString("ops at or after the offset are excluded"),
      scopeId: "root",
      offset: 5,
      expected: null,
    },
    {
      name: asFilledString("no candidates -> null"),
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
    expect(result?.refId ?? null).toEqual(expected);
  });

  test("variable with no recorded writes returns null", () => {
    expect(
      lastWriteOpInScopeBefore("missing", "root", 100, byVar, scopeMap),
    ).toEqual(null);
  });
});
