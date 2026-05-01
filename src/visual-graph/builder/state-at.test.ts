import { describe, expect, test } from "vitest";

import { stateAt } from "./state-at.js";
import { makeWriteOp } from "./testing/make-write-op.js";
import type { WriteOp } from "./write-op.js";

const ops: WriteOp[] = [
  makeWriteOp({ refId: "r1", varId: "v", offset: 10 }),
  makeWriteOp({ refId: "r2", varId: "v", offset: 20 }),
  makeWriteOp({ refId: "r3", varId: "v", offset: 30 }),
];
const byVar = new Map<string, WriteOp[]>([["v", ops]]);

describe("stateAt", () => {
  test.each<{ name: string; offset: number; expected: string }>([
    {
      name: "before any write returns the variable id",
      offset: 5,
      expected: "v",
    },
    {
      name: "exactly at first write offset still pre-write",
      offset: 10,
      expected: "v",
    },
    {
      name: "between first and second write returns first refId",
      offset: 15,
      expected: "r1",
    },
    {
      name: "between second and third returns second refId",
      offset: 25,
      expected: "r2",
    },
    {
      name: "after last write returns last refId",
      offset: 999,
      expected: "r3",
    },
  ])("$name", ({ offset, expected }) => {
    expect(stateAt("v", offset, byVar)).toBe(expected);
  });

  test("variable with no recorded writes returns the variable id", () => {
    expect(stateAt("missing", 100, byVar)).toBe("missing");
  });
});
