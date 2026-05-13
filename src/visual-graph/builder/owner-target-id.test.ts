import { describe, expect, test } from "vitest";

import { asFilledString } from "../../util/filled-string.js";
import { ownerTargetId } from "./owner-target-id.js";
import { baseWriteOp } from "./testing/make-write-op.js";
import type { WriteOp } from "./write-op.js";

const ops: WriteOp[] = [
  { ...baseWriteOp(), refId: "r1", varId: "owner", offset: 10 },
  { ...baseWriteOp(), refId: "r2", varId: "owner", offset: 20 },
];
const byVar = new Map<string, WriteOp[]>([["owner", ops]]);

describe("ownerTargetId", () => {
  test.each<{ name: string; offset: number; expected: string }>([
    {
      name: asFilledString("before any write returns the owner node id"),
      offset: 5,
      expected: "n_owner",
    },
    {
      name: asFilledString(
        "at first write offset (inclusive) returns its writeOp node",
      ),
      offset: 10,
      expected: "wr_r1",
    },
    {
      name: asFilledString("between writes returns first writeOp node"),
      offset: 15,
      expected: "wr_r1",
    },
    {
      name: asFilledString("at second write offset returns its writeOp node"),
      offset: 20,
      expected: "wr_r2",
    },
    {
      name: asFilledString("after last write returns last writeOp node"),
      offset: 999,
      expected: "wr_r2",
    },
  ])("$name", ({ offset, expected }) => {
    expect(ownerTargetId("owner", offset, byVar)).toEqual(expected);
  });

  test("owner without recorded writes falls back to nodeId", () => {
    expect(ownerTargetId("missing", 100, byVar)).toEqual("n_missing");
  });
});
